[CmdletBinding()]
param(
    [string]$ProjectRoot
)

$ErrorActionPreference = "Stop"

# DRY_RUN_ONLY: M10 LOOP-051 selected JPL Horizons source payload materialization preflight.
# This script must not create JPL/GB payload files, compute new source hashes, call external sources, or write generated artifacts.

if ([string]::IsNullOrWhiteSpace($ProjectRoot)) {
    $ProjectRoot = Join-Path $PSScriptRoot ".."
}

$resolvedProject = Resolve-Path -LiteralPath $ProjectRoot
$projectPath = $resolvedProject.ProviderPath
$naifSourceId = "naif-cspice"
$iauSourceId = "iau-sofa-ansi-c"
$selectedSourceId = "jpl-horizons-api"
$gbtSourceId = "gb-t-33661-2017"
$naifHash = "4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2"
$iauHash = "436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f"
$jplHash = "acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9"
$gbtHash = "7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31"

function Read-Json {
    param([string]$RelativePath)
    $path = Join-Path $projectPath $RelativePath
    if (-not (Test-Path -LiteralPath $path)) {
        throw "Missing selected JPL Horizons preflight file: $RelativePath"
    }
    return Get-Content -LiteralPath $path -Encoding UTF8 -Raw | ConvertFrom-Json
}

$manifest = Read-Json "data/generated/astronomy/source-snapshots/source-snapshot-manifest.json"
$policy = Read-Json "data/generated/astronomy/source-payload-materialization-policy.json"
$procedure = Read-Json "data/generated/astronomy/source-capture-procedure.json"
$strategy = Read-Json "data/generated/astronomy/post-iau-remaining-source-payload-strategy.json"
$preflight = Read-Json "data/generated/astronomy/selected-jpl-horizons-payload-materialization-preflight.json"
$materialization = Read-Json "data/generated/astronomy/selected-jpl-horizons-payload-materialization.json"
$gbtMaterialization = Read-Json "data/generated/astronomy/selected-gb-t-payload-materialization.json"
$draftManifest = Read-Json "data/generated/astronomy/manifests/astronomy-engine-v0-draft.json"

if ($preflight.status -ne "preflight_only") {
    throw "Selected JPL Horizons payload materialization preflight must remain preflight_only."
}

if ($preflight.post_iau_remaining_source_payload_strategy_id -ne $strategy.post_iau_remaining_source_payload_strategy_id -or
    $preflight.source_payload_materialization_policy_id -ne $policy.source_payload_materialization_policy_id -or
    $preflight.source_capture_procedure_id -ne $procedure.source_capture_procedure_id -or
    $preflight.source_snapshot_manifest_id -ne $manifest.source_snapshot_manifest_id) {
    throw "Selected JPL Horizons preflight must reference active post-IAU strategy, policy, procedure, and manifest."
}

if ($strategy.status -ne "strategy_decision_only" -or
    $strategy.next_selected_source.source_id -ne $selectedSourceId -or
    $strategy.allowed_next_loop.selected_source_payload_preflight -ne $true) {
    throw "Post-IAU remaining source strategy must select JPL Horizons preflight before this preflight can close."
}

if ($preflight.selected_source.source_id -ne $selectedSourceId) {
    throw "Selected JPL Horizons preflight must remain scoped to jpl-horizons-api."
}

$payloadMatch = @($policy.planned_payloads | Where-Object { $_.source_id -eq $selectedSourceId })
$procedureMatch = @($procedure.procedures | Where-Object { $_.source_id -eq $selectedSourceId })
$manifestMatch = @($manifest.sources | Where-Object { $_.source_id -eq $selectedSourceId })
if ($payloadMatch.Count -ne 1 -or $procedureMatch.Count -ne 1 -or $manifestMatch.Count -ne 1) {
    throw "Selected JPL Horizons source must exist in policy, procedure, and manifest."
}

$payload = $payloadMatch[0]
if ($preflight.selected_source.payload_kind -ne $payload.payload_kind -or
    $preflight.selected_source.schema_path -ne $payload.schema_path -or
    $preflight.selected_source.payload_path -ne $payload.path -or
    $preflight.selected_source.payload_format -ne $payload.payload_format) {
    throw "Selected JPL Horizons preflight source must match payload policy."
}

if ($payload.payload_status -ne "materialized" -or $payload.hash_status -ne "computed" -or $payload.sha256 -ne $jplHash) {
    throw "JPL Horizons payload must be materialized with the expected hash after preflight closes."
}

if ($procedureMatch[0].capture_status -ne "completed_for_validation_query_snapshot_boundary" -or
    $procedureMatch[0].materialization_status -ne "validation_query_snapshot_payload_materialized" -or
    $procedureMatch[0].hash_status -ne "computed" -or
    $procedureMatch[0].sha256 -ne $jplHash) {
    throw "JPL Horizons capture procedure must record closed preflight materialization after LOOP-052."
}

$schemaPath = Join-Path $projectPath $preflight.selected_source.schema_path
if (-not (Test-Path -LiteralPath $schemaPath)) {
    throw "Selected JPL Horizons schema missing: $($preflight.selected_source.schema_path)"
}
$schema = Get-Content -LiteralPath $schemaPath -Encoding UTF8 -Raw | ConvertFrom-Json
if ($schema.status -ne "schema_only" -or
    $schema.source_id -ne $selectedSourceId -or
    $schema.payload_kind -ne $preflight.selected_source.payload_kind) {
    throw "Selected JPL Horizons schema must remain schema_only and match preflight."
}

foreach ($field in $schema.required_fields) {
    if ($preflight.offline_query_boundary_policy.required_payload_fields -notcontains $field) {
        throw "Selected JPL Horizons preflight missing required payload field from schema: $field"
    }
}

foreach ($field in $schema.required_query_snapshot_fields) {
    if ($preflight.offline_query_boundary_policy.required_query_snapshot_fields -notcontains $field) {
        throw "Selected JPL Horizons preflight missing required query snapshot field from schema: $field"
    }
}

foreach ($claim in $preflight.selected_payload_write_policy.forbidden_payload_claims) {
    if ($schema.forbidden_claims -contains $claim) {
        continue
    }
    if ($claim -eq "Android baseline replaced") {
        continue
    }
    throw "Selected JPL Horizons schema missing forbidden payload claim: $claim"
}

if ($preflight.payload_directory_policy.path -ne $policy.payload_directory.path -or
    $preflight.payload_directory_policy.current_status -ne "exists_selected_source_only" -or
    $preflight.payload_directory_policy.existing_materialized_source_count -ne 2 -or
    $preflight.payload_directory_policy.create_allowed_in_this_loop -ne $false -or
    $preflight.payload_directory_policy.next_loop_write_scope -ne "selected_source_only") {
    throw "Selected JPL Horizons preflight must preserve selected-source-only payload directory policy."
}

if ($preflight.selected_payload_write_policy.write_allowed_in_this_loop -ne $false -or
    $preflight.selected_payload_write_policy.next_loop_write_scope -ne "selected_source_only" -or
    $preflight.selected_payload_write_policy.canonical_json_required -ne $true -or
    $preflight.selected_payload_write_policy.allowed_payload_claim -ne "offline-validation-query-snapshot-boundary-only") {
    throw "Selected JPL Horizons preflight must keep writes blocked this loop and source-only next loop."
}

if ($preflight.selected_payload_hash_policy.hash_algorithm -ne "sha256" -or
    $preflight.selected_payload_hash_policy.hash_allowed_in_this_loop -ne $false -or
    $preflight.selected_payload_hash_policy.next_loop_hash_scope -ne "selected_source_payload_only") {
    throw "Selected JPL Horizons preflight must keep hashes blocked this loop and scoped next loop."
}

if ($preflight.offline_query_boundary_policy.full_gate_network_policy -ne "no_external_calls" -or
    $preflight.offline_query_boundary_policy.query_execution_allowed_in_this_loop -ne $false -or
    $preflight.offline_query_boundary_policy.query_execution_allowed_in_full_gate -ne $false -or
    $preflight.offline_query_boundary_policy.manual_or_external_capture_may_be_recorded_after_preflight -ne $true -or
    $preflight.offline_query_boundary_policy.sample_set_scope -ne "validation-query-snapshot-set") {
    throw "Selected JPL Horizons preflight must keep query execution outside this loop and outside full gate."
}

$payloadDirectory = Join-Path $projectPath $preflight.payload_directory_policy.path
if (-not (Test-Path -LiteralPath $payloadDirectory)) {
    throw "Payload directory must already exist from selected source materializations."
}

$existingPayloadFiles = @()
foreach ($plannedPayload in $policy.planned_payloads) {
    $plannedPayloadPath = Join-Path $projectPath $plannedPayload.path
    if (Test-Path -LiteralPath $plannedPayloadPath) {
        $existingPayloadFiles += $plannedPayload.path
    }

    if ($plannedPayload.source_id -notin @($naifSourceId, $iauSourceId, $selectedSourceId, $gbtSourceId) -and (Test-Path -LiteralPath $plannedPayloadPath)) {
        throw "Only NAIF, IAU SOFA, JPL Horizons, and GB/T payloads may exist after LOOP-054: $($plannedPayload.path)"
    }
}

if ($existingPayloadFiles.Count -ne 4) {
    throw "Exactly four payload files must exist after LOOP-054."
}

foreach ($expected in @(
    @{ source_id = $naifSourceId; sha256 = $naifHash },
    @{ source_id = $iauSourceId; sha256 = $iauHash }
)) {
    $materializedPayload = @($policy.planned_payloads | Where-Object { $_.source_id -eq $expected.source_id })
    $materializedPayloadPath = Join-Path $projectPath $materializedPayload[0].path
    if (-not (Test-Path -LiteralPath $materializedPayloadPath)) {
        throw "Existing materialized payload missing during JPL Horizons preflight: $($materializedPayload[0].path)"
    }
    $actualHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $materializedPayloadPath).Hash.ToLowerInvariant()
    if ($actualHash -ne $expected.sha256) {
        throw "Existing materialized payload hash changed during JPL Horizons preflight for $($expected.source_id): $actualHash"
    }
}

$selectedPayloadPath = Join-Path $projectPath $preflight.selected_source.payload_path
if (-not (Test-Path -LiteralPath $selectedPayloadPath)) {
    throw "JPL Horizons selected payload must exist after preflight closes: $($preflight.selected_source.payload_path)"
}
$actualSelectedHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $selectedPayloadPath).Hash.ToLowerInvariant()
if ($actualSelectedHash -ne $jplHash) {
    throw "JPL Horizons selected payload hash mismatch after preflight closes: $actualSelectedHash"
}

if ($materialization.status -ne "selected_source_payload_materialized" -or
    $materialization.selected_source.source_id -ne $selectedSourceId -or
    $materialization.selected_source.sha256 -ne $jplHash -or
    $materialization.online_query_executed_in_full_gate -ne $false -or
    $materialization.external_calls_performed -ne $false -or
    $materialization.response_bodies_materialized -ne $false) {
    throw "JPL Horizons materialization evidence must record selected payload, expected hash, no full-gate query, no external calls, and no response bodies."
}

$gbtPayload = @($policy.planned_payloads | Where-Object { $_.source_id -eq $gbtSourceId })
if ($gbtPayload.Count -ne 1) {
    throw "GB/T payload policy entry is missing."
}
$gbtPayloadPath = Join-Path $projectPath $gbtPayload[0].path
if (-not (Test-Path -LiteralPath $gbtPayloadPath)) {
    throw "GB/T payload must exist after LOOP-054: $($gbtPayload[0].path)"
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

if ($draftManifest.acceptance_status -ne "not_accepted") {
    throw "Draft manifest must remain not_accepted during JPL Horizons preflight."
}

foreach ($check in @(
    "post-IAU remaining source strategy dry-run passes",
    "selected source remains jpl-horizons-api",
    "selected schema remains schema_only",
    "existing naif-cspice payload hash remains unchanged",
    "existing iau-sofa payload hash remains unchanged",
    "jpl-horizons payload is absent before materialization",
    "gb-t payload is absent before materialization",
    "no external API call in full project gate",
    "query execution is not part of full project gate",
    "generated artifact paths remain absent",
    "draft manifest remains not_accepted",
    "runtime behavior unchanged",
    "astronomy-engine remains target"
)) {
    if ($preflight.preflight_checks -notcontains $check) {
        throw "Selected JPL Horizons preflight missing check: $check"
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
    throw "Selected JPL Horizons preflight must allow only selected source payload after preflight."
}

foreach ($forbidden in @(
    "write jpl-horizons payload file",
    "write gb-t payload file",
    "compute new source payload hash",
    "perform external API call in full project gate",
    "execute online JPL Horizons query in full project gate",
    "write generated astronomy artifacts",
    "compute generated artifact hashes",
    "mark draft manifest accepted",
    "change calendar-date-query runtime behavior",
    "change chart-create runtime behavior",
    "replace android-date-layer-v1",
    "claim astronomy-engine supported"
)) {
    if ($preflight.forbidden_in_preflight_stage -notcontains $forbidden) {
        throw "Selected JPL Horizons preflight missing forbidden item: $forbidden"
    }
}

$result = [pscustomobject]@{
    mode = "selected_jpl_horizons_payload_materialization_preflight_closed_dry_run"
    preflight_id = $preflight.selected_source_payload_materialization_preflight_id
    strategy_id = $strategy.post_iau_remaining_source_payload_strategy_id
    selected_source_id = $preflight.selected_source.source_id
    selected_payload_kind = $preflight.selected_source.payload_kind
    selected_payload_path = $preflight.selected_source.payload_path
    payload_directory = $preflight.payload_directory_policy.path
    payload_directory_exists = (Test-Path -LiteralPath $payloadDirectory)
    selected_payload_exists = (Test-Path -LiteralPath $selectedPayloadPath)
    existing_payload_files = $existingPayloadFiles
    existing_payload_count = @($existingPayloadFiles).Count
    materialization_id = $materialization.selected_source_payload_materialization_id
    source_payloads_materialized = 4
    new_source_payloads_written = 1
    new_source_payload_hashes_computed = 1
    next_loop_write_scope = $preflight.selected_payload_write_policy.next_loop_write_scope
    next_loop_hash_scope = $preflight.selected_payload_hash_policy.next_loop_hash_scope
    query_execution_allowed_in_full_gate = $preflight.offline_query_boundary_policy.query_execution_allowed_in_full_gate
    external_calls_performed = $false
    generated_artifacts_written = 0
    generated_artifact_hashes_computed = 0
    acceptance_status_changed = $false
    runtime_behavior_changed = $false
    writes_performed = $false
}

$result | ConvertTo-Json -Depth 8
exit 0
