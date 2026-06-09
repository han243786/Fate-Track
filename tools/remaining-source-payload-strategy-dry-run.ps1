[CmdletBinding()]
param(
    [string]$ProjectRoot
)

$ErrorActionPreference = "Stop"

# DRY_RUN_ONLY: M10 LOOP-047 remaining source payload strategy inspection.
# This script must not create payload files, compute new hashes, call external sources, or write generated artifacts.

if ([string]::IsNullOrWhiteSpace($ProjectRoot)) {
    $ProjectRoot = Join-Path $PSScriptRoot ".."
}

$resolvedProject = Resolve-Path -LiteralPath $ProjectRoot
$projectPath = $resolvedProject.ProviderPath
$materializedSourceId = "naif-cspice"
$nextSourceId = "iau-sofa-ansi-c"
$materializedHash = "4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2"
$iauSofaHash = "436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f"
$jplHash = "acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9"
$gbtHash = "7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31"

function Read-Json {
    param([string]$RelativePath)
    $path = Join-Path $projectPath $RelativePath
    if (-not (Test-Path -LiteralPath $path)) {
        throw "Missing remaining source payload strategy file: $RelativePath"
    }
    return Get-Content -LiteralPath $path -Encoding UTF8 -Raw | ConvertFrom-Json
}

$manifest = Read-Json "data/generated/astronomy/source-snapshots/source-snapshot-manifest.json"
$policy = Read-Json "data/generated/astronomy/source-payload-materialization-policy.json"
$procedure = Read-Json "data/generated/astronomy/source-capture-procedure.json"
$materialization = Read-Json "data/generated/astronomy/selected-source-payload-materialization.json"
$iauSofaMaterialization = Read-Json "data/generated/astronomy/selected-iau-sofa-payload-materialization.json"
$jplMaterialization = Read-Json "data/generated/astronomy/selected-jpl-horizons-payload-materialization.json"
$gbtMaterialization = Read-Json "data/generated/astronomy/selected-gb-t-payload-materialization.json"
$strategy = Read-Json "data/generated/astronomy/remaining-source-payload-strategy.json"
$draftManifest = Read-Json "data/generated/astronomy/manifests/astronomy-engine-v0-draft.json"

if ($strategy.status -ne "strategy_decision_only") {
    throw "Remaining source payload strategy must remain strategy_decision_only."
}

if ($strategy.source_payload_materialization_policy_id -ne $policy.source_payload_materialization_policy_id -or
    $strategy.source_capture_procedure_id -ne $procedure.source_capture_procedure_id -or
    $strategy.source_snapshot_manifest_id -ne $manifest.source_snapshot_manifest_id -or
    $strategy.selected_source_payload_materialization_id -ne $materialization.selected_source_payload_materialization_id) {
    throw "Remaining source payload strategy must reference active policy, procedure, manifest, and selected materialization evidence."
}

if ($manifest.status -ne "selected_source_payload_materialized" -or
    $policy.status -ne "selected_source_payload_materialized" -or
    $procedure.status -ne "selected_source_payload_materialized" -or
    $materialization.status -ne "selected_source_payload_materialized") {
    throw "Remaining source payload strategy requires the selected naif-cspice materialization state."
}

if (@($strategy.currently_materialized_sources).Count -ne 1 -or
    $strategy.currently_materialized_sources[0].source_id -ne $materializedSourceId -or
    $strategy.currently_materialized_sources[0].sha256 -ne $materializedHash) {
    throw "Remaining source payload strategy must record exactly one materialized naif-cspice source hash at decision time."
}

$materializedPayload = @($policy.planned_payloads | Where-Object { $_.source_id -eq $materializedSourceId })
if ($materializedPayload.Count -ne 1 -or
    $materializedPayload[0].payload_status -ne "materialized" -or
    $materializedPayload[0].hash_status -ne "computed" -or
    $materializedPayload[0].sha256 -ne $materializedHash) {
    throw "Policy must keep naif-cspice as the only materialized payload."
}

$materializedPayloadPath = Join-Path $projectPath $materializedPayload[0].path
if (-not (Test-Path -LiteralPath $materializedPayloadPath)) {
    throw "Materialized naif-cspice payload file is missing."
}
$actualHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $materializedPayloadPath).Hash.ToLowerInvariant()
if ($actualHash -ne $materializedHash) {
    throw "Materialized naif-cspice payload hash changed: $actualHash"
}

$iauSofaPayload = @($policy.planned_payloads | Where-Object { $_.source_id -eq "iau-sofa-ansi-c" })
if ($iauSofaPayload.Count -ne 1 -or
    $iauSofaPayload[0].payload_status -ne "materialized" -or
    $iauSofaPayload[0].hash_status -ne "computed" -or
    $iauSofaPayload[0].sha256 -ne $iauSofaHash) {
    throw "IAU SOFA payload must be materialized after the strategy closes."
}

if ($iauSofaMaterialization.status -ne "selected_source_payload_materialized" -or
    $iauSofaMaterialization.selected_source.source_id -ne "iau-sofa-ansi-c" -or
    $iauSofaMaterialization.selected_source.sha256 -ne $iauSofaHash) {
    throw "IAU SOFA selected materialization evidence must record the expected source hash."
}

$iauSofaPayloadPath = Join-Path $projectPath $iauSofaPayload[0].path
if (-not (Test-Path -LiteralPath $iauSofaPayloadPath)) {
    throw "Materialized IAU SOFA payload file is missing."
}
$actualIauSofaHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $iauSofaPayloadPath).Hash.ToLowerInvariant()
if ($actualIauSofaHash -ne $iauSofaHash) {
    throw "Materialized IAU SOFA payload hash changed: $actualIauSofaHash"
}

$jplPayload = @($policy.planned_payloads | Where-Object { $_.source_id -eq "jpl-horizons-api" })
if ($jplPayload.Count -ne 1 -or
    $jplPayload[0].payload_status -ne "materialized" -or
    $jplPayload[0].hash_status -ne "computed" -or
    $jplPayload[0].sha256 -ne $jplHash) {
    throw "JPL Horizons payload must be materialized after LOOP-052."
}

if ($jplMaterialization.status -ne "selected_source_payload_materialized" -or
    $jplMaterialization.selected_source.source_id -ne "jpl-horizons-api" -or
    $jplMaterialization.selected_source.sha256 -ne $jplHash) {
    throw "JPL Horizons selected materialization evidence must record the expected source hash."
}

$jplPayloadPath = Join-Path $projectPath $jplPayload[0].path
if (-not (Test-Path -LiteralPath $jplPayloadPath)) {
    throw "Materialized JPL Horizons payload file is missing."
}
$actualJplHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $jplPayloadPath).Hash.ToLowerInvariant()
if ($actualJplHash -ne $jplHash) {
    throw "Materialized JPL Horizons payload hash changed: $actualJplHash"
}

$gbtPayload = @($policy.planned_payloads | Where-Object { $_.source_id -eq "gb-t-33661-2017" })
if ($gbtPayload.Count -ne 1 -or
    $gbtPayload[0].payload_status -ne "materialized" -or
    $gbtPayload[0].hash_status -ne "computed" -or
    $gbtPayload[0].sha256 -ne $gbtHash) {
    throw "GB/T payload must be materialized after LOOP-054."
}

if ($gbtMaterialization.status -ne "selected_source_payload_materialized" -or
    $gbtMaterialization.selected_source.source_id -ne "gb-t-33661-2017" -or
    $gbtMaterialization.selected_source.sha256 -ne $gbtHash) {
    throw "GB/T selected materialization evidence must record the expected source hash."
}

$gbtPayloadPath = Join-Path $projectPath $gbtPayload[0].path
if (-not (Test-Path -LiteralPath $gbtPayloadPath)) {
    throw "Materialized GB/T payload file is missing."
}
$actualGbtHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $gbtPayloadPath).Hash.ToLowerInvariant()
if ($actualGbtHash -ne $gbtHash) {
    throw "Materialized GB/T payload hash changed: $actualGbtHash"
}

$existingPayloadFiles = @()
foreach ($payload in $policy.planned_payloads) {
    $payloadPath = Join-Path $projectPath $payload.path
    if (Test-Path -LiteralPath $payloadPath) {
        $existingPayloadFiles += $payload.path
    }
}

if ($existingPayloadFiles.Count -ne 4) {
    throw "Exactly four payload files must exist after LOOP-054."
}

if (@($strategy.remaining_source_sequence).Count -ne 3) {
    throw "Remaining source strategy must order all three unmaterialized sources."
}

$sequenceIds = @($strategy.remaining_source_sequence | Sort-Object order | ForEach-Object { $_.source_id })
if ($sequenceIds[0] -ne $nextSourceId -or
    $sequenceIds[1] -ne "jpl-horizons-api" -or
    $sequenceIds[2] -ne "gb-t-33661-2017") {
    throw "Remaining source strategy must select IAU SOFA first, then JPL Horizons, then GB/T."
}

$nextPayload = @($policy.planned_payloads | Where-Object { $_.source_id -eq $nextSourceId })
$nextProcedure = @($procedure.procedures | Where-Object { $_.source_id -eq $nextSourceId })
$nextManifestSource = @($manifest.sources | Where-Object { $_.source_id -eq $nextSourceId })
if ($nextPayload.Count -ne 1 -or $nextProcedure.Count -ne 1 -or $nextManifestSource.Count -ne 1) {
    throw "Next selected source must exist in policy, procedure, and manifest."
}

if ($strategy.next_selected_source.source_id -ne $nextSourceId -or
    $strategy.next_selected_source.payload_kind -ne $nextPayload[0].payload_kind -or
    $strategy.next_selected_source.schema_path -ne $nextPayload[0].schema_path -or
    $strategy.next_selected_source.payload_path -ne $nextPayload[0].path -or
    $strategy.next_selected_source.payload_format -ne $nextPayload[0].payload_format) {
    throw "Next selected source must match payload policy."
}

$nextSchemaPath = Join-Path $projectPath $strategy.next_selected_source.schema_path
if (-not (Test-Path -LiteralPath $nextSchemaPath)) {
    throw "Next selected source schema missing: $($strategy.next_selected_source.schema_path)"
}
$nextSchema = Get-Content -LiteralPath $nextSchemaPath -Encoding UTF8 -Raw | ConvertFrom-Json
if ($nextSchema.status -ne "schema_only" -or
    $nextSchema.source_id -ne $nextSourceId -or
    $nextSchema.payload_kind -ne $strategy.next_selected_source.payload_kind) {
    throw "Next selected source schema must remain schema_only and match strategy."
}

if ($strategy.allowed_next_loop.selected_source_payload_preflight -ne $true -or
    $strategy.allowed_next_loop.selected_source_id -ne $nextSourceId -or
    $strategy.allowed_next_loop.payload_materialization -ne $false -or
    $strategy.allowed_next_loop.payload_hash_computation -ne $false -or
    $strategy.allowed_next_loop.generated_astronomy_artifacts -ne $false -or
    $strategy.allowed_next_loop.generated_artifact_hashes -ne $false -or
    $strategy.allowed_next_loop.draft_manifest_acceptance_change -ne $false -or
    $strategy.allowed_next_loop.runtime_behavior_change -ne $false -or
    $strategy.allowed_next_loop.capability_promotion -ne $false) {
    throw "Remaining source strategy must allow only next-loop preflight for IAU SOFA."
}

if ($draftManifest.acceptance_status -ne "not_accepted") {
    throw "Draft manifest must remain not_accepted during remaining source strategy."
}

foreach ($check in @(
    "exactly one source payload is materialized before strategy decision",
    "naif-cspice payload hash remains unchanged",
    "remaining sources are not_materialized",
    "next source is selected from remaining sources",
    "next loop is preflight-only",
    "no external API call in full project gate",
    "generated artifact paths remain absent",
    "draft manifest remains not_accepted",
    "runtime behavior unchanged",
    "astronomy-engine remains target"
)) {
    if ($strategy.strategy_checks -notcontains $check) {
        throw "Remaining source strategy missing check: $check"
    }
}

foreach ($forbidden in @(
    "write iau-sofa payload file",
    "write jpl-horizons payload file",
    "write gb-t payload file",
    "compute new source payload hash",
    "perform external API call in full project gate",
    "write generated astronomy artifacts",
    "compute generated artifact hashes",
    "mark draft manifest accepted",
    "change calendar-date-query runtime behavior",
    "change chart-create runtime behavior",
    "replace android-date-layer-v1",
    "claim astronomy-engine supported"
)) {
    if ($strategy.forbidden_in_strategy_stage -notcontains $forbidden) {
        throw "Remaining source strategy missing forbidden item: $forbidden"
    }
}

$result = [pscustomobject]@{
    mode = "remaining_source_payload_strategy_closed_dry_run"
    strategy_id = $strategy.remaining_source_payload_strategy_id
    selected_materialization_id = $materialization.selected_source_payload_materialization_id
    iau_sofa_materialization_id = $iauSofaMaterialization.selected_source_payload_materialization_id
    jpl_horizons_materialization_id = $jplMaterialization.selected_source_payload_materialization_id
    gbt_materialization_id = $gbtMaterialization.selected_source_payload_materialization_id
    materialized_source_count = 4
    materialized_source_ids = @("naif-cspice", "iau-sofa-ansi-c", "jpl-horizons-api", "gb-t-33661-2017")
    materialized_payload_hashes = @{
        "naif-cspice" = $materializedHash
        "iau-sofa-ansi-c" = $iauSofaHash
        "jpl-horizons-api" = $jplHash
        "gb-t-33661-2017" = $gbtHash
    }
    remaining_source_count = 0
    next_selected_source_id = "jpl-horizons-api"
    next_selected_payload_kind = "validation-query-snapshot-set"
    next_loop_action = "remaining_source_payload_strategy_after_iau_sofa"
    existing_payload_files = $existingPayloadFiles
    source_payloads_materialized = 4
    new_source_payloads_written = 3
    new_source_payload_hashes_computed = 3
    external_calls_performed = $false
    generated_artifacts_written = 0
    generated_artifact_hashes_computed = 0
    acceptance_status_changed = $false
    runtime_behavior_changed = $false
    writes_performed = $false
}

$result | ConvertTo-Json -Depth 8
exit 0
