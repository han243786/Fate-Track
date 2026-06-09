[CmdletBinding()]
param(
    [string]$ProjectRoot
)

$ErrorActionPreference = "Stop"

# DRY_RUN_ONLY: M10 source capture procedure inspection.
# This script must not create payload directories, source payload files, external calls, hashes, or generated artifacts.

if ([string]::IsNullOrWhiteSpace($ProjectRoot)) {
    $ProjectRoot = Join-Path $PSScriptRoot ".."
}

$resolvedProject = Resolve-Path -LiteralPath $ProjectRoot
$projectPath = $resolvedProject.ProviderPath

function Read-Json {
    param([string]$RelativePath)
    $path = Join-Path $projectPath $RelativePath
    if (-not (Test-Path -LiteralPath $path)) {
        throw "Missing astronomy source capture planning file: $RelativePath"
    }
    return Get-Content -LiteralPath $path -Encoding UTF8 -Raw | ConvertFrom-Json
}

$manifest = Read-Json "data/generated/astronomy/source-snapshots/source-snapshot-manifest.json"
$policy = Read-Json "data/generated/astronomy/source-payload-materialization-policy.json"
$procedure = Read-Json "data/generated/astronomy/source-capture-procedure.json"
$materializedSourceIds = @("naif-cspice", "iau-sofa-ansi-c", "jpl-horizons-api", "gb-t-33661-2017")
$expectedHashes = @{
    "naif-cspice" = "4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2"
    "iau-sofa-ansi-c" = "436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f"
    "jpl-horizons-api" = "acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9"
    "gb-t-33661-2017" = "7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31"
}

if ($manifest.status -ne "selected_source_payload_materialized") {
    throw "Source snapshot manifest must be selected_source_payload_materialized in LOOP-046."
}

if ($policy.status -ne "selected_source_payload_materialized") {
    throw "Source payload policy must be selected_source_payload_materialized in LOOP-046."
}

if ($procedure.status -ne "selected_source_payload_materialized") {
    throw "Source capture procedure must record selected-source payload materialization."
}

if ($procedure.source_snapshot_manifest_id -ne $manifest.source_snapshot_manifest_id -or
    $procedure.source_payload_materialization_policy_id -ne $policy.source_payload_materialization_policy_id) {
    throw "Source capture procedure must reference the active source manifest and payload policy."
}

if ($procedure.full_gate_network_policy -ne "no_external_calls") {
    throw "Source capture procedure must preserve no external calls in full gate."
}

if ($procedure.payload_materialization_allowed -ne $true -or
    $procedure.payload_materialization_scope -ne "selected_source_only" -or
    $procedure.payload_hash_computation_allowed -ne $true -or
    $procedure.payload_hash_computation_scope -ne "selected_source_payload_only" -or
    $procedure.generated_artifact_allowed -ne $false -or
    $procedure.generated_artifact_hash_allowed -ne $false -or
    $procedure.manifest_acceptance_change_allowed -ne $false -or
    $procedure.runtime_behavior_change_allowed -ne $false -or
    $procedure.capability_status -ne "target") {
    throw "Source capture procedure must allow only selected source payload/hash and forbid artifacts, acceptance changes, runtime changes, or capability promotion."
}

$payloadDirectory = Join-Path $projectPath $procedure.payload_directory.path
if (-not (Test-Path -LiteralPath $payloadDirectory)) {
    throw "Source payload directory must exist after selected-source materialization: $($procedure.payload_directory.path)"
}

if ($procedure.payload_directory.status -ne "exists_selected_source_only") {
    throw "Source capture procedure payload directory policy must remain selected-source-only."
}

foreach ($sourceId in $materializedSourceIds) {
    if ($procedure.payload_directory.allowed_materialized_sources -notcontains $sourceId) {
        throw "Source capture procedure payload directory policy must allow materialized source: $sourceId"
    }
}

$existingPayloadFiles = @()
$schemaFiles = @()
$materializedPayloadFiles = @()
$missingProcedureSourceIds = @()
foreach ($payload in $policy.planned_payloads) {
    $procedureMatch = @($procedure.procedures | Where-Object { $_.source_id -eq $payload.source_id })
    $manifestMatch = @($manifest.sources | Where-Object { $_.source_id -eq $payload.source_id })
    if ($procedureMatch.Count -ne 1 -or $manifestMatch.Count -ne 1) {
        $missingProcedureSourceIds += $payload.source_id
        continue
    }

    $sourceProcedure = $procedureMatch[0]
    if ($sourceProcedure.payload_kind -ne $payload.payload_kind -or
        $sourceProcedure.schema_path -ne $payload.schema_path -or
        $sourceProcedure.payload_path -ne $payload.path) {
        throw "Source capture procedure must match payload policy for source: $($payload.source_id)"
    }

    if ($materializedSourceIds -contains $payload.source_id) {
        $expectedHash = $expectedHashes[$payload.source_id]
        if ($sourceProcedure.hash_status -ne "computed" -or $sourceProcedure.sha256 -ne $expectedHash) {
            throw "Selected source capture procedure must record expected sha256: $($payload.source_id)"
        }
        if ($payload.source_id -eq "naif-cspice" -and
            ($sourceProcedure.capture_status -ne "completed_for_boundary_payload" -or
            $sourceProcedure.materialization_status -ne "source_boundary_payload_materialized")) {
            throw "NAIF source capture procedure must record boundary payload materialization."
        }
        if ($payload.source_id -eq "iau-sofa-ansi-c" -and
            ($sourceProcedure.capture_status -ne "completed_for_routine_version_payload" -or
            $sourceProcedure.materialization_status -ne "routine_version_payload_materialized")) {
            throw "IAU SOFA source capture procedure must record routine version payload materialization."
        }
        if ($payload.source_id -eq "jpl-horizons-api" -and
            ($sourceProcedure.capture_status -ne "completed_for_validation_query_snapshot_boundary" -or
            $sourceProcedure.materialization_status -ne "validation_query_snapshot_payload_materialized")) {
            throw "JPL Horizons source capture procedure must record validation query snapshot payload materialization."
        }
        if ($payload.source_id -eq "gb-t-33661-2017" -and
            ($sourceProcedure.capture_status -ne "completed_for_rule_reference_boundary" -or
            $sourceProcedure.materialization_status -ne "rule_reference_payload_materialized")) {
            throw "GB/T source capture procedure must record rule reference payload materialization."
        }
    } else {
        if ($sourceProcedure.capture_status -ne "not_started" -or
            $sourceProcedure.materialization_status -ne "not_materialized" -or
            $sourceProcedure.hash_status -ne "not_computed") {
            throw "Unselected source capture procedure must remain not_started/not_materialized/not_computed: $($payload.source_id)"
        }
    }

    if (@($sourceProcedure.capture_steps).Count -lt 5 -or @($sourceProcedure.required_evidence_fields).Count -lt 5) {
        throw "Source capture procedure must define capture steps and evidence fields: $($payload.source_id)"
    }

    $schemaPath = Join-Path $projectPath $sourceProcedure.schema_path
    if (-not (Test-Path -LiteralPath $schemaPath)) {
        throw "Source capture procedure references missing schema: $($sourceProcedure.schema_path)"
    }
    $schema = Get-Content -LiteralPath $schemaPath -Encoding UTF8 -Raw | ConvertFrom-Json
    if ($schema.status -ne "schema_only" -or
        $schema.source_id -ne $sourceProcedure.source_id -or
        $schema.payload_kind -ne $sourceProcedure.payload_kind) {
        throw "Source capture schema must remain schema_only and match procedure: $($sourceProcedure.source_id)"
    }
    $schemaFiles += $sourceProcedure.schema_path

    $payloadPath = Join-Path $projectPath $sourceProcedure.payload_path
    if (Test-Path -LiteralPath $payloadPath) {
        if ($materializedSourceIds -notcontains $payload.source_id) {
            throw "Unselected source payload file must not exist: $($sourceProcedure.payload_path)"
        }
        $expectedHash = $expectedHashes[$payload.source_id]
        $actualHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $payloadPath).Hash.ToLowerInvariant()
        if ($actualHash -ne $expectedHash) {
            throw "Selected source payload hash mismatch in capture procedure dry-run for $($payload.source_id): $actualHash"
        }
        $existingPayloadFiles += $sourceProcedure.payload_path
        $materializedPayloadFiles += $sourceProcedure.payload_path
    } elseif ($materializedSourceIds -contains $payload.source_id) {
        throw "Selected source payload file missing: $($sourceProcedure.payload_path)"
    }
}

if ($missingProcedureSourceIds.Count -gt 0) {
    throw "Source capture procedure missing source ids: $($missingProcedureSourceIds -join ', ')"
}

if ($existingPayloadFiles.Count -ne $materializedSourceIds.Count -or $materializedPayloadFiles.Count -ne $materializedSourceIds.Count) {
    throw "Exactly $($materializedSourceIds.Count) selected source payloads must exist in source capture procedure dry-run."
}

$result = [pscustomobject]@{
    mode = "source_capture_procedure_selected_payloads_dry_run"
    procedure_id = $procedure.source_capture_procedure_id
    policy_id = $policy.source_payload_materialization_policy_id
    manifest_id = $manifest.source_snapshot_manifest_id
    selected_source_ids = $materializedSourceIds
    payload_directory = $procedure.payload_directory.path
    payload_directory_exists = (Test-Path -LiteralPath $payloadDirectory)
    planned_source_count = @($policy.planned_payloads).Count
    procedure_source_count = @($procedure.procedures).Count
    schema_file_count = @($schemaFiles).Count
    existing_payload_files = $existingPayloadFiles
    materialized_payload_files = $materializedPayloadFiles
    source_payloads_materialized = @($materializedSourceIds).Count
    payload_hashes_computed = @($materializedSourceIds).Count
    external_calls_performed = $false
    generated_artifacts_written = 0
    generated_artifact_hashes_computed = 0
    acceptance_status_changed = $false
    runtime_behavior_changed = $false
    writes_performed = $false
}

$result | ConvertTo-Json -Depth 8
exit 0
