[CmdletBinding()]
param(
    [string]$ProjectRoot
)

$ErrorActionPreference = "Stop"

# DRY_RUN_ONLY: M10 source snapshot manifest inspection.
# This script must not create source payload files or generated astronomy artifacts.

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
        throw "Missing astronomy source snapshot planning file: $RelativePath"
    }
    return Get-Content -LiteralPath $path -Encoding UTF8 -Raw | ConvertFrom-Json
}

$schema = Read-Json "data/generated/astronomy/source-snapshot-manifest.schema.json"
$plan = Read-Json "data/generated/astronomy/source-snapshot-manifest-plan.json"
$sourcePolicy = Read-Json "data/generated/astronomy/source-policy.json"
$sourceAdapterContract = Read-Json "data/generated/astronomy/source-adapter-contract.json"
$generatorEntry = Read-Json "data/generated/astronomy/generator-implementation-entry.json"
$manifestPath = Join-Path $projectPath $plan.manifest_path
$manifestDirectory = Split-Path -Parent $manifestPath

if ($schema.status -ne "schema_only") {
    throw "Source snapshot manifest schema must remain schema_only."
}

if ($plan.status -ne "manifest_materialized_metadata_only") {
    throw "Source snapshot manifest plan must remain manifest_materialized_metadata_only."
}

if ($generatorEntry.source_snapshot_manifest.path -ne $plan.manifest_path) {
    throw "Generator entry must point to the planned source snapshot manifest path."
}

if (-not (Test-Path -LiteralPath $manifestPath)) {
    throw "Source snapshot manifest must exist after LOOP-040."
}

$manifest = Get-Content -LiteralPath $manifestPath -Encoding UTF8 -Raw | ConvertFrom-Json

if ($manifest.status -ne "selected_source_payload_materialized") {
    throw "Source snapshot manifest must be selected_source_payload_materialized in LOOP-046."
}

if ($manifest.source_policy_id -ne $sourcePolicy.source_policy_id -or
    $manifest.source_adapter_contract_id -ne $sourceAdapterContract.source_adapter_contract_id -or
    $manifest.generator_contract_id -ne $generatorEntry.generator_contract_id) {
    throw "Source snapshot manifest must reference active source policy, adapter contract, and generator contract."
}

if ($manifest.snapshot_range.start_year -ne $sourcePolicy.first_generated_range.start_year -or
    $manifest.snapshot_range.end_year -ne $sourcePolicy.first_generated_range.end_year) {
    throw "Source snapshot manifest range must match source policy first generated range."
}

$sourcePayloadsMaterialized = 0
$sourcePayloadHashesComputed = 0
foreach ($source in $manifest.sources) {
    if ($source.runtime_dependency -ne $false -or $source.output_claim_allowed -ne $false) {
        throw "Source snapshot manifest source must forbid runtime dependency and output claim: $($source.source_id)"
    }

    if ($materializedSourceIds -contains $source.source_id) {
        $expectedHash = $expectedHashes[$source.source_id]
        if ($source.source_id -eq "naif-cspice" -and $source.local_materialization_status -ne "source_boundary_payload_materialized") {
            throw "NAIF selected source must be source_boundary_payload_materialized."
        }
        if ($source.source_id -eq "iau-sofa-ansi-c" -and $source.local_materialization_status -ne "routine_version_payload_materialized") {
            throw "IAU SOFA selected source must be routine_version_payload_materialized."
        }
        if ($source.source_id -eq "jpl-horizons-api" -and $source.local_materialization_status -ne "validation_query_snapshot_payload_materialized") {
            throw "JPL Horizons selected source must be validation_query_snapshot_payload_materialized."
        }
        if ($source.source_id -eq "gb-t-33661-2017" -and $source.local_materialization_status -ne "rule_reference_payload_materialized") {
            throw "GB/T selected source must be rule_reference_payload_materialized."
        }
        if ($source.source_payload_hash.algorithm -ne "sha256" -or $source.source_payload_hash.value -ne $expectedHash) {
            throw "Selected source payload hash metadata mismatch: $($source.source_id)"
        }
        $payloadPath = Join-Path $projectPath $source.source_payload_path
        if (-not (Test-Path -LiteralPath $payloadPath)) {
            throw "Selected source payload file missing: $($source.source_payload_path)"
        }
        $actualHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $payloadPath).Hash.ToLowerInvariant()
        if ($actualHash -ne $expectedHash) {
            throw "Selected source payload hash mismatch for $($source.source_id): $actualHash"
        }
        $sourcePayloadsMaterialized += 1
        $sourcePayloadHashesComputed += 1
    } else {
        if ($source.local_materialization_status -ne "not_materialized") {
            throw "Unselected source must remain not_materialized: $($source.source_id)"
        }
    }
}

if ($sourcePayloadsMaterialized -ne $materializedSourceIds.Count -or $sourcePayloadHashesComputed -ne $materializedSourceIds.Count) {
    throw "Exactly $($materializedSourceIds.Count) selected source payloads and hashes must be materialized."
}

$result = [pscustomobject]@{
    mode = "source_snapshot_manifest_selected_payload_dry_run"
    schema_id = $schema.schema_id
    plan_id = $plan.source_snapshot_manifest_plan_id
    manifest_id = $manifest.source_snapshot_manifest_id
    manifest_status = $manifest.status
    manifest_path = $plan.manifest_path
    manifest_exists = (Test-Path -LiteralPath $manifestPath)
    manifest_directory_exists = (Test-Path -LiteralPath $manifestDirectory)
    selected_source_ids = $materializedSourceIds
    planned_source_count = @($plan.planned_sources).Count
    manifest_source_count = @($manifest.sources).Count
    planned_sources = $plan.planned_sources
    writes_performed = $false
    source_snapshots_materialized = @($materializedSourceIds).Count
    source_payload_hashes_computed = @($materializedSourceIds).Count
    generated_artifacts_written = 0
    generated_artifact_hashes_computed = 0
    acceptance_status_changed = $false
    runtime_behavior_changed = $false
}

$result | ConvertTo-Json -Depth 8
exit 0
