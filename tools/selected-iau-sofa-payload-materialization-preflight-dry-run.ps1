[CmdletBinding()]
param(
    [string]$ProjectRoot
)

$ErrorActionPreference = "Stop"

# DRY_RUN_ONLY: M10 LOOP-048 selected IAU SOFA source payload materialization preflight.
# This script must not create payload files, compute new hashes, call external sources, or write generated artifacts.

if ([string]::IsNullOrWhiteSpace($ProjectRoot)) {
    $ProjectRoot = Join-Path $PSScriptRoot ".."
}

$resolvedProject = Resolve-Path -LiteralPath $ProjectRoot
$projectPath = $resolvedProject.ProviderPath
$materializedSourceId = "naif-cspice"
$selectedSourceId = "iau-sofa-ansi-c"
$postSelectedSourceId = "jpl-horizons-api"
$materializedHash = "4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2"
$selectedSourceHash = "436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f"
$postSelectedSourceHash = "acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9"
$gbtSourceId = "gb-t-33661-2017"
$gbtHash = "7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31"

function Read-Json {
    param([string]$RelativePath)
    $path = Join-Path $projectPath $RelativePath
    if (-not (Test-Path -LiteralPath $path)) {
        throw "Missing selected IAU SOFA preflight file: $RelativePath"
    }
    return Get-Content -LiteralPath $path -Encoding UTF8 -Raw | ConvertFrom-Json
}

$manifest = Read-Json "data/generated/astronomy/source-snapshots/source-snapshot-manifest.json"
$policy = Read-Json "data/generated/astronomy/source-payload-materialization-policy.json"
$procedure = Read-Json "data/generated/astronomy/source-capture-procedure.json"
$strategy = Read-Json "data/generated/astronomy/remaining-source-payload-strategy.json"
$preflight = Read-Json "data/generated/astronomy/selected-iau-sofa-payload-materialization-preflight.json"
$materialization = Read-Json "data/generated/astronomy/selected-iau-sofa-payload-materialization.json"
$gbtMaterialization = Read-Json "data/generated/astronomy/selected-gb-t-payload-materialization.json"
$draftManifest = Read-Json "data/generated/astronomy/manifests/astronomy-engine-v0-draft.json"

if ($preflight.status -ne "preflight_only") {
    throw "Selected IAU SOFA payload materialization preflight must remain preflight_only."
}

if ($preflight.remaining_source_payload_strategy_id -ne $strategy.remaining_source_payload_strategy_id -or
    $preflight.source_payload_materialization_policy_id -ne $policy.source_payload_materialization_policy_id -or
    $preflight.source_capture_procedure_id -ne $procedure.source_capture_procedure_id -or
    $preflight.source_snapshot_manifest_id -ne $manifest.source_snapshot_manifest_id) {
    throw "Selected IAU SOFA preflight must reference active strategy, policy, procedure, and manifest."
}

if ($strategy.status -ne "strategy_decision_only" -or
    $strategy.next_selected_source.source_id -ne $selectedSourceId -or
    $strategy.allowed_next_loop.selected_source_payload_preflight -ne $true) {
    throw "Remaining source strategy must select IAU SOFA preflight before this preflight can close."
}

if ($preflight.selected_source.source_id -ne $selectedSourceId) {
    throw "Selected IAU SOFA preflight must remain scoped to iau-sofa-ansi-c."
}

$payloadMatch = @($policy.planned_payloads | Where-Object { $_.source_id -eq $selectedSourceId })
$procedureMatch = @($procedure.procedures | Where-Object { $_.source_id -eq $selectedSourceId })
$manifestMatch = @($manifest.sources | Where-Object { $_.source_id -eq $selectedSourceId })
if ($payloadMatch.Count -ne 1 -or $procedureMatch.Count -ne 1 -or $manifestMatch.Count -ne 1) {
    throw "Selected IAU SOFA source must exist in policy, procedure, and manifest."
}

$payload = $payloadMatch[0]
if ($preflight.selected_source.payload_kind -ne $payload.payload_kind -or
    $preflight.selected_source.schema_path -ne $payload.schema_path -or
    $preflight.selected_source.payload_path -ne $payload.path -or
    $preflight.selected_source.payload_format -ne $payload.payload_format) {
    throw "Selected IAU SOFA preflight source must match payload policy."
}

if ($payload.payload_status -ne "materialized" -or $payload.hash_status -ne "computed" -or $payload.sha256 -ne $selectedSourceHash) {
    throw "IAU SOFA payload must be materialized with the expected hash after preflight closes."
}

if ($procedureMatch[0].capture_status -ne "not_started" -or
    $procedureMatch[0].materialization_status -ne "not_materialized" -or
    $procedureMatch[0].hash_status -ne "not_computed") {
    if ($procedureMatch[0].capture_status -ne "completed_for_routine_version_payload" -or
        $procedureMatch[0].materialization_status -ne "routine_version_payload_materialized" -or
        $procedureMatch[0].hash_status -ne "computed" -or
        $procedureMatch[0].sha256 -ne $selectedSourceHash) {
        throw "IAU SOFA capture procedure must record closed preflight materialization after LOOP-049."
    }
}

$schemaPath = Join-Path $projectPath $preflight.selected_source.schema_path
if (-not (Test-Path -LiteralPath $schemaPath)) {
    throw "Selected IAU SOFA schema missing: $($preflight.selected_source.schema_path)"
}
$schema = Get-Content -LiteralPath $schemaPath -Encoding UTF8 -Raw | ConvertFrom-Json
if ($schema.status -ne "schema_only" -or
    $schema.source_id -ne $selectedSourceId -or
    $schema.payload_kind -ne $preflight.selected_source.payload_kind) {
    throw "Selected IAU SOFA schema must remain schema_only and match preflight."
}

foreach ($claim in $preflight.selected_payload_write_policy.forbidden_payload_claims) {
    if ($schema.forbidden_claims -contains $claim) {
        continue
    }
    if ($claim -in @("Android baseline replaced")) {
        continue
    }
    throw "Selected IAU SOFA schema missing forbidden payload claim: $claim"
}

if ($preflight.payload_directory_policy.path -ne $policy.payload_directory.path -or
    $preflight.payload_directory_policy.current_status -ne "exists_selected_source_only" -or
    $preflight.payload_directory_policy.existing_materialized_source_count -ne 1 -or
    $preflight.payload_directory_policy.create_allowed_in_this_loop -ne $false -or
    $preflight.payload_directory_policy.next_loop_write_scope -ne "selected_source_only") {
    throw "Selected IAU SOFA preflight must preserve selected-source-only payload directory policy."
}

if ($preflight.selected_payload_write_policy.write_allowed_in_this_loop -ne $false -or
    $preflight.selected_payload_write_policy.next_loop_write_scope -ne "selected_source_only" -or
    $preflight.selected_payload_write_policy.canonical_json_required -ne $true -or
    $preflight.selected_payload_write_policy.allowed_payload_claim -ne "local-routine-version-boundary-only") {
    throw "Selected IAU SOFA preflight must keep writes blocked this loop and source-only next loop."
}

if ($preflight.selected_payload_hash_policy.hash_algorithm -ne "sha256" -or
    $preflight.selected_payload_hash_policy.hash_allowed_in_this_loop -ne $false -or
    $preflight.selected_payload_hash_policy.next_loop_hash_scope -ne "selected_source_payload_only") {
    throw "Selected IAU SOFA preflight must keep hashes blocked this loop and scoped next loop."
}

$payloadDirectory = Join-Path $projectPath $preflight.payload_directory_policy.path
if (-not (Test-Path -LiteralPath $payloadDirectory)) {
    throw "Payload directory must already exist from naif-cspice materialization."
}

$existingPayloadFiles = @()
foreach ($plannedPayload in $policy.planned_payloads) {
    $plannedPayloadPath = Join-Path $projectPath $plannedPayload.path
    if (Test-Path -LiteralPath $plannedPayloadPath) {
        $existingPayloadFiles += $plannedPayload.path
    }

    if ($plannedPayload.source_id -notin @($materializedSourceId, $selectedSourceId, $postSelectedSourceId, $gbtSourceId) -and (Test-Path -LiteralPath $plannedPayloadPath)) {
        throw "Only selected NAIF, IAU SOFA, JPL Horizons, and GB/T payloads may exist after LOOP-054: $($plannedPayload.path)"
    }
}

if ($existingPayloadFiles.Count -ne 4) {
    throw "Exactly four payload files must exist after LOOP-054."
}

$materializedPayload = @($policy.planned_payloads | Where-Object { $_.source_id -eq $materializedSourceId })
$materializedPayloadPath = Join-Path $projectPath $materializedPayload[0].path
$actualHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $materializedPayloadPath).Hash.ToLowerInvariant()
if ($actualHash -ne $materializedHash) {
    throw "Existing naif-cspice payload hash changed during IAU SOFA preflight: $actualHash"
}

$selectedPayloadPath = Join-Path $projectPath $preflight.selected_source.payload_path
if (-not (Test-Path -LiteralPath $selectedPayloadPath)) {
    throw "IAU SOFA selected payload must exist after preflight closes: $($preflight.selected_source.payload_path)"
}
$actualSelectedHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $selectedPayloadPath).Hash.ToLowerInvariant()
if ($actualSelectedHash -ne $selectedSourceHash) {
    throw "IAU SOFA selected payload hash mismatch after preflight closes: $actualSelectedHash"
}

$postSelectedPayload = @($policy.planned_payloads | Where-Object { $_.source_id -eq $postSelectedSourceId })
$postSelectedPayloadPath = Join-Path $projectPath $postSelectedPayload[0].path
if (-not (Test-Path -LiteralPath $postSelectedPayloadPath)) {
    throw "JPL Horizons selected payload must exist after LOOP-052: $($postSelectedPayload[0].path)"
}
$actualPostSelectedHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $postSelectedPayloadPath).Hash.ToLowerInvariant()
if ($actualPostSelectedHash -ne $postSelectedSourceHash) {
    throw "JPL Horizons selected payload hash mismatch after LOOP-052: $actualPostSelectedHash"
}

$gbtPayload = @($policy.planned_payloads | Where-Object { $_.source_id -eq $gbtSourceId })
$gbtPayloadPath = Join-Path $projectPath $gbtPayload[0].path
if (-not (Test-Path -LiteralPath $gbtPayloadPath)) {
    throw "GB/T selected payload must exist after LOOP-054: $($gbtPayload[0].path)"
}
$actualGbtHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $gbtPayloadPath).Hash.ToLowerInvariant()
if ($actualGbtHash -ne $gbtHash) {
    throw "GB/T selected payload hash mismatch after LOOP-054: $actualGbtHash"
}

if ($gbtMaterialization.status -ne "selected_source_payload_materialized" -or
    $gbtMaterialization.selected_source.source_id -ne $gbtSourceId -or
    $gbtMaterialization.selected_source.sha256 -ne $gbtHash) {
    throw "GB/T materialization evidence must record the selected payload and expected hash."
}

if ($materialization.status -ne "selected_source_payload_materialized" -or
    $materialization.selected_source.source_id -ne $selectedSourceId -or
    $materialization.selected_source.sha256 -ne $selectedSourceHash) {
    throw "IAU SOFA materialization evidence must record the selected payload and expected hash."
}

if ($draftManifest.acceptance_status -ne "not_accepted") {
    throw "Draft manifest must remain not_accepted during IAU SOFA preflight."
}

foreach ($check in @(
    "remaining source strategy dry-run passes",
    "selected source remains iau-sofa-ansi-c",
    "selected schema remains schema_only",
    "existing naif-cspice payload hash remains unchanged",
    "iau-sofa payload is absent before materialization",
    "jpl-horizons payload is absent before materialization",
    "gb-t payload is absent before materialization",
    "no external API call in full project gate",
    "generated artifact paths remain absent",
    "draft manifest remains not_accepted",
    "runtime behavior unchanged",
    "astronomy-engine remains target"
)) {
    if ($preflight.preflight_checks -notcontains $check) {
        throw "Selected IAU SOFA preflight missing check: $check"
    }
}

if ($preflight.materialization_allowed_after_preflight.selected_source_payload -ne $true -or
    $preflight.materialization_allowed_after_preflight.selected_source_id -ne $selectedSourceId -or
    $preflight.materialization_allowed_after_preflight.other_remaining_source_payloads -ne $false -or
    $preflight.materialization_allowed_after_preflight.generated_astronomy_artifacts -ne $false -or
    $preflight.materialization_allowed_after_preflight.generated_artifact_hashes -ne $false -or
    $preflight.materialization_allowed_after_preflight.draft_manifest_acceptance_change -ne $false -or
    $preflight.materialization_allowed_after_preflight.runtime_behavior_change -ne $false -or
    $preflight.materialization_allowed_after_preflight.capability_promotion -ne $false) {
    throw "Selected IAU SOFA preflight must allow only selected source payload after preflight."
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
    if ($preflight.forbidden_in_preflight_stage -notcontains $forbidden) {
        throw "Selected IAU SOFA preflight missing forbidden item: $forbidden"
    }
}

$result = [pscustomobject]@{
    mode = "selected_iau_sofa_payload_materialization_preflight_closed_dry_run"
    preflight_id = $preflight.selected_source_payload_materialization_preflight_id
    strategy_id = $strategy.remaining_source_payload_strategy_id
    materialization_id = $materialization.selected_source_payload_materialization_id
    selected_source_id = $preflight.selected_source.source_id
    selected_payload_kind = $preflight.selected_source.payload_kind
    selected_payload_path = $preflight.selected_source.payload_path
    payload_directory = $preflight.payload_directory_policy.path
    payload_directory_exists = (Test-Path -LiteralPath $payloadDirectory)
    selected_payload_exists = (Test-Path -LiteralPath $selectedPayloadPath)
    existing_payload_files = $existingPayloadFiles
    existing_payload_count = @($existingPayloadFiles).Count
    source_payloads_materialized = 4
    new_source_payloads_written = 1
    new_source_payload_hashes_computed = 1
    next_loop_write_scope = $preflight.selected_payload_write_policy.next_loop_write_scope
    next_loop_hash_scope = $preflight.selected_payload_hash_policy.next_loop_hash_scope
    external_calls_performed = $false
    generated_artifacts_written = 0
    generated_artifact_hashes_computed = 0
    acceptance_status_changed = $false
    runtime_behavior_changed = $false
    writes_performed = $false
}

$result | ConvertTo-Json -Depth 8
exit 0
