[CmdletBinding()]
param(
    [string]$ProjectRoot
)

$ErrorActionPreference = "Stop"

# DRY_RUN_ONLY: M10 source payload materialization inspection.
# This script must not create payload directories, source payload files, hashes, or generated astronomy artifacts.

if ([string]::IsNullOrWhiteSpace($ProjectRoot)) {
    $ProjectRoot = Join-Path $PSScriptRoot ".."
}

$resolvedProject = Resolve-Path -LiteralPath $ProjectRoot
$projectPath = $resolvedProject.ProviderPath
$materializedSourceIds = @("naif-cspice", "iau-sofa-ansi-c", "jpl-horizons-api", "gb-t-33661-2017")
$expectedHashes = @{
    "naif-cspice" = "4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2"
    "iau-sofa-ansi-c" = "436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f"
    "jpl-horizons-api" = "acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9"
    "gb-t-33661-2017" = "7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31"
}

function Read-Json {
    param([string]$RelativePath)
    $path = Join-Path $projectPath $RelativePath
    if (-not (Test-Path -LiteralPath $path)) {
        throw "Missing astronomy source payload planning file: $RelativePath"
    }
    return Get-Content -LiteralPath $path -Encoding UTF8 -Raw | ConvertFrom-Json
}

$manifest = Read-Json "data/generated/astronomy/source-snapshots/source-snapshot-manifest.json"
$policy = Read-Json "data/generated/astronomy/source-payload-materialization-policy.json"
$sourceAdapterContract = Read-Json "data/generated/astronomy/source-adapter-contract.json"

if ($manifest.status -ne "selected_source_payload_materialized") {
    throw "Source snapshot manifest must be selected_source_payload_materialized in LOOP-046."
}

if ($policy.status -ne "selected_source_payload_materialized") {
    throw "Source payload materialization policy must be selected_source_payload_materialized in LOOP-046."
}

if ($policy.source_snapshot_manifest_id -ne $manifest.source_snapshot_manifest_id) {
    throw "Source payload policy must reference the active source snapshot manifest."
}

$payloadDirectory = Join-Path $projectPath $policy.payload_directory.path
if (-not (Test-Path -LiteralPath $payloadDirectory)) {
    throw "Source payload directory must exist after selected-source materialization: $($policy.payload_directory.path)"
}

if ($policy.payload_directory.status -ne "exists_selected_source_only") {
    throw "Source payload directory policy must remain selected-source-only."
}

foreach ($sourceId in $materializedSourceIds) {
    if ($policy.payload_directory.allowed_materialized_sources -notcontains $sourceId) {
        throw "Source payload directory policy must allow materialized source: $sourceId"
    }
}

$missingPayloadSourceIds = @()
$existingPayloadFiles = @()
$schemaFiles = @()
$materializedPayloads = @()
foreach ($source in $manifest.sources) {
    $payloadMatch = @($policy.planned_payloads | Where-Object { $_.source_id -eq $source.source_id })
    $adapterMatch = @($sourceAdapterContract.adapters | Where-Object { $_.source_id -eq $source.source_id })
    if ($payloadMatch.Count -ne 1 -or $adapterMatch.Count -ne 1) {
        $missingPayloadSourceIds += $source.source_id
        continue
    }

    $payload = $payloadMatch[0]
    $schemaPath = Join-Path $projectPath $payload.schema_path
    if (-not (Test-Path -LiteralPath $schemaPath)) {
        throw "Source payload schema missing: $($payload.schema_path)"
    }

    $schema = Get-Content -LiteralPath $schemaPath -Encoding UTF8 -Raw | ConvertFrom-Json
    if ($schema.status -ne "schema_only") {
        throw "Source payload schema must remain schema_only: $($payload.schema_path)"
    }
    if ($schema.source_id -ne $payload.source_id -or $schema.payload_kind -ne $payload.payload_kind) {
        throw "Source payload schema must match payload source and kind: $($payload.source_id)"
    }
    $schemaFiles += $payload.schema_path

    if ($payload.runtime_dependency -ne $false -or $payload.output_claim_allowed -ne $false) {
        throw "Source payload plan must forbid runtime dependency and output claim: $($payload.source_id)"
    }

    $payloadPath = Join-Path $projectPath $payload.path
    if (Test-Path -LiteralPath $payloadPath) {
        $existingPayloadFiles += $payload.path
    }

    if ($materializedSourceIds -contains $payload.source_id) {
        $expectedHash = $expectedHashes[$payload.source_id]
        if ($payload.payload_status -ne "materialized" -or $payload.hash_status -ne "computed" -or $payload.sha256 -ne $expectedHash) {
            throw "Selected source payload must be materialized with expected sha256: $($payload.source_id)"
        }
        if (-not (Test-Path -LiteralPath $payloadPath)) {
            throw "Selected source payload file missing: $($payload.path)"
        }
        $actualHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $payloadPath).Hash.ToLowerInvariant()
        if ($actualHash -ne $expectedHash) {
            throw "Selected source payload hash mismatch for $($payload.source_id): $actualHash"
        }
        $materializedPayloads += $payload.path
    } else {
        if ($payload.payload_status -ne "not_materialized" -or $payload.hash_status -ne "not_computed") {
            throw "Unselected source payload must remain not_materialized/not_computed: $($payload.source_id)"
        }
        if (Test-Path -LiteralPath $payloadPath) {
            throw "Unselected source payload file must not exist: $($payload.path)"
        }
    }
}

if ($missingPayloadSourceIds.Count -gt 0) {
    throw "Source payload policy missing source ids: $($missingPayloadSourceIds -join ', ')"
}

if ($materializedPayloads.Count -ne $materializedSourceIds.Count -or $existingPayloadFiles.Count -ne $materializedSourceIds.Count) {
    throw "Exactly $($materializedSourceIds.Count) selected source payloads must exist after LOOP-054."
}

$result = [pscustomobject]@{
    mode = "source_payload_materialization_selected_sources_dry_run"
    policy_id = $policy.source_payload_materialization_policy_id
    manifest_id = $manifest.source_snapshot_manifest_id
    selected_source_ids = $materializedSourceIds
    payload_directory = $policy.payload_directory.path
    payload_directory_exists = (Test-Path -LiteralPath $payloadDirectory)
    planned_payload_count = @($policy.planned_payloads).Count
    schema_file_count = @($schemaFiles).Count
    schema_files = $schemaFiles
    manifest_source_count = @($manifest.sources).Count
    existing_payload_files = $existingPayloadFiles
    materialized_payload_files = $materializedPayloads
    source_payloads_materialized = @($materializedSourceIds).Count
    payload_hashes_computed = @($materializedSourceIds).Count
    generated_artifacts_written = 0
    generated_artifact_hashes_computed = 0
    acceptance_status_changed = $false
    runtime_behavior_changed = $false
    writes_performed = $false
}

$result | ConvertTo-Json -Depth 8
exit 0
