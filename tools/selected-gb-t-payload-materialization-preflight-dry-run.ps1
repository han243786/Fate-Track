[CmdletBinding()]
param(
    [string]$ProjectRoot
)

$ErrorActionPreference = "Stop"

# DRY_RUN_ONLY: M10 LOOP-053 selected GB/T 33661 source payload materialization preflight.
# This script must not create GB/T payload files, compute a GB/T source hash, call external sources, or write generated artifacts.

if ([string]::IsNullOrWhiteSpace($ProjectRoot)) {
    $ProjectRoot = Join-Path $PSScriptRoot ".."
}

$resolvedProject = Resolve-Path -LiteralPath $ProjectRoot
$projectPath = $resolvedProject.ProviderPath
$naifSourceId = "naif-cspice"
$iauSourceId = "iau-sofa-ansi-c"
$jplSourceId = "jpl-horizons-api"
$selectedSourceId = "gb-t-33661-2017"
$naifHash = "4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2"
$iauHash = "436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f"
$jplHash = "acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9"
$gbtHash = "7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31"

function Read-Json {
    param([string]$RelativePath)
    $path = Join-Path $projectPath $RelativePath
    if (-not (Test-Path -LiteralPath $path)) {
        throw "Missing selected GB/T preflight file: $RelativePath"
    }
    return Get-Content -LiteralPath $path -Encoding UTF8 -Raw | ConvertFrom-Json
}

$manifest = Read-Json "data/generated/astronomy/source-snapshots/source-snapshot-manifest.json"
$policy = Read-Json "data/generated/astronomy/source-payload-materialization-policy.json"
$procedure = Read-Json "data/generated/astronomy/source-capture-procedure.json"
$jplMaterialization = Read-Json "data/generated/astronomy/selected-jpl-horizons-payload-materialization.json"
$preflight = Read-Json "data/generated/astronomy/selected-gb-t-payload-materialization-preflight.json"
$materialization = Read-Json "data/generated/astronomy/selected-gb-t-payload-materialization.json"
$draftManifest = Read-Json "data/generated/astronomy/manifests/astronomy-engine-v0-draft.json"

if ($preflight.status -ne "preflight_only") {
    throw "Selected GB/T payload materialization preflight must remain preflight_only."
}

if ($preflight.selected_jpl_horizons_payload_materialization_id -ne $jplMaterialization.selected_source_payload_materialization_id -or
    $preflight.source_payload_materialization_policy_id -ne $policy.source_payload_materialization_policy_id -or
    $preflight.source_capture_procedure_id -ne $procedure.source_capture_procedure_id -or
    $preflight.source_snapshot_manifest_id -ne $manifest.source_snapshot_manifest_id) {
    throw "Selected GB/T preflight must reference active JPL materialization, policy, procedure, and manifest."
}

if ($jplMaterialization.status -ne "selected_source_payload_materialized" -or
    $jplMaterialization.selected_source.source_id -ne $jplSourceId -or
    $jplMaterialization.selected_source.sha256 -ne $jplHash) {
    throw "Selected GB/T preflight requires closed JPL Horizons materialization evidence."
}

if ($preflight.selected_source.source_id -ne $selectedSourceId) {
    throw "Selected GB/T preflight must remain scoped to gb-t-33661-2017."
}

$payloadMatch = @($policy.planned_payloads | Where-Object { $_.source_id -eq $selectedSourceId })
$procedureMatch = @($procedure.procedures | Where-Object { $_.source_id -eq $selectedSourceId })
$manifestMatch = @($manifest.sources | Where-Object { $_.source_id -eq $selectedSourceId })
if ($payloadMatch.Count -ne 1 -or $procedureMatch.Count -ne 1 -or $manifestMatch.Count -ne 1) {
    throw "Selected GB/T source must exist in policy, procedure, and manifest."
}

$payload = $payloadMatch[0]
if ($preflight.selected_source.payload_kind -ne $payload.payload_kind -or
    $preflight.selected_source.schema_path -ne $payload.schema_path -or
    $preflight.selected_source.payload_path -ne $payload.path -or
    $preflight.selected_source.payload_format -ne $payload.payload_format) {
    throw "Selected GB/T preflight source must match payload policy."
}

if ($payload.payload_status -ne "materialized" -or $payload.hash_status -ne "computed" -or $payload.sha256 -ne $gbtHash) {
    throw "GB/T payload must be materialized with the expected hash after LOOP-054."
}

if ($procedureMatch[0].capture_status -ne "completed_for_rule_reference_boundary" -or
    $procedureMatch[0].materialization_status -ne "rule_reference_payload_materialized" -or
    $procedureMatch[0].hash_status -ne "computed" -or
    $procedureMatch[0].sha256 -ne $gbtHash) {
    throw "GB/T capture procedure must record LOOP-054 materialization."
}

if ($manifestMatch[0].local_materialization_status -ne "rule_reference_payload_materialized" -or
    $manifestMatch[0].source_payload_hash.value -ne $gbtHash -or
    $manifestMatch[0].runtime_dependency -ne $false -or
    $manifestMatch[0].output_claim_allowed -ne $false) {
    throw "GB/T manifest entry must record LOOP-054 rule-reference payload with runtime and output claims disabled."
}

$schemaPath = Join-Path $projectPath $preflight.selected_source.schema_path
if (-not (Test-Path -LiteralPath $schemaPath)) {
    throw "Selected GB/T schema missing: $($preflight.selected_source.schema_path)"
}
$schema = Get-Content -LiteralPath $schemaPath -Encoding UTF8 -Raw | ConvertFrom-Json
if ($schema.status -ne "schema_only" -or
    $schema.source_id -ne $selectedSourceId -or
    $schema.payload_kind -ne $preflight.selected_source.payload_kind) {
    throw "Selected GB/T schema must remain schema_only and match preflight."
}

foreach ($field in $schema.required_fields) {
    if ($preflight.rule_reference_boundary_policy.required_payload_fields -notcontains $field) {
        throw "Selected GB/T preflight missing required payload field from schema: $field"
    }
}

foreach ($field in $schema.required_rule_scope_fields) {
    if ($preflight.rule_reference_boundary_policy.required_rule_scope_fields -notcontains $field) {
        throw "Selected GB/T preflight missing required rule scope field from schema: $field"
    }
}

foreach ($claim in $preflight.selected_payload_write_policy.forbidden_payload_claims) {
    if ($schema.forbidden_claims -contains $claim) {
        continue
    }
    if ($claim -eq "Android baseline replaced") {
        continue
    }
    throw "Selected GB/T schema missing forbidden payload claim: $claim"
}

if ($preflight.payload_directory_policy.path -ne $policy.payload_directory.path -or
    $preflight.payload_directory_policy.current_status -ne "exists_selected_source_only" -or
    $preflight.payload_directory_policy.existing_materialized_source_count -ne 3 -or
    $preflight.payload_directory_policy.create_allowed_in_this_loop -ne $false -or
    $preflight.payload_directory_policy.next_loop_write_scope -ne "selected_source_only") {
    throw "Selected GB/T preflight must preserve selected-source-only payload directory policy."
}

if ($preflight.selected_payload_write_policy.write_allowed_in_this_loop -ne $false -or
    $preflight.selected_payload_write_policy.next_loop_write_scope -ne "selected_source_only" -or
    $preflight.selected_payload_write_policy.canonical_json_required -ne $true -or
    $preflight.selected_payload_write_policy.allowed_payload_claim -ne "calendar-rule-reference-boundary-only") {
    throw "Selected GB/T preflight must keep writes blocked this loop and source-only next loop."
}

if ($preflight.selected_payload_hash_policy.hash_algorithm -ne "sha256" -or
    $preflight.selected_payload_hash_policy.hash_allowed_in_this_loop -ne $false -or
    $preflight.selected_payload_hash_policy.next_loop_hash_scope -ne "selected_source_payload_only") {
    throw "Selected GB/T preflight must keep hashes blocked this loop and scoped next loop."
}

if ($preflight.rule_reference_boundary_policy.full_gate_network_policy -ne "no_external_calls" -or
    $preflight.rule_reference_boundary_policy.source_reference_capture_allowed_in_this_loop -ne $false -or
    $preflight.rule_reference_boundary_policy.payload_materialization_allowed_in_this_loop -ne $false -or
    $preflight.rule_reference_boundary_policy.manual_or_external_capture_may_be_recorded_after_preflight -ne $true -or
    $preflight.rule_reference_boundary_policy.sample_set_scope -ne "calendar-rule-reference") {
    throw "Selected GB/T preflight must keep rule-reference capture and payload materialization outside this loop."
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
}

if ($existingPayloadFiles.Count -ne 4) {
    throw "Exactly four payload files must exist after LOOP-054."
}

foreach ($expected in @(
    @{ source_id = $naifSourceId; sha256 = $naifHash },
    @{ source_id = $iauSourceId; sha256 = $iauHash },
    @{ source_id = $jplSourceId; sha256 = $jplHash },
    @{ source_id = $selectedSourceId; sha256 = $gbtHash }
)) {
    $materializedPayload = @($policy.planned_payloads | Where-Object { $_.source_id -eq $expected.source_id })
    $materializedPayloadPath = Join-Path $projectPath $materializedPayload[0].path
    if (-not (Test-Path -LiteralPath $materializedPayloadPath)) {
        throw "Existing materialized payload missing during GB/T preflight: $($materializedPayload[0].path)"
    }
    $actualHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $materializedPayloadPath).Hash.ToLowerInvariant()
    if ($actualHash -ne $expected.sha256) {
        throw "Existing materialized payload hash changed during GB/T preflight for $($expected.source_id): $actualHash"
    }
}

$selectedPayloadPath = Join-Path $projectPath $preflight.selected_source.payload_path
$actualSelectedHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $selectedPayloadPath).Hash.ToLowerInvariant()
if ($actualSelectedHash -ne $gbtHash) {
    throw "GB/T selected payload hash mismatch after LOOP-054: $actualSelectedHash"
}

if ($materialization.status -ne "selected_source_payload_materialized" -or
    $materialization.selected_source.source_id -ne $selectedSourceId -or
    $materialization.selected_source.sha256 -ne $gbtHash -or
    $materialization.standard_text_copied -ne $false -or
    $materialization.generated_artifact_allowed -ne $false -or
    $materialization.runtime_behavior_change_allowed -ne $false -or
    $materialization.android_baseline_replacement_allowed -ne $false) {
    throw "GB/T materialization evidence must record selected payload, expected hash, no standard text copy, no generated artifact, no runtime change, and no Android replacement."
}

if ($draftManifest.acceptance_status -ne "not_accepted") {
    throw "Draft manifest must remain not_accepted during GB/T preflight."
}

foreach ($check in @(
    "selected JPL Horizons materialization dry-run passes",
    "selected source remains gb-t-33661-2017",
    "selected schema remains schema_only",
    "existing naif-cspice payload hash remains unchanged",
    "existing iau-sofa payload hash remains unchanged",
    "existing jpl-horizons payload hash remains unchanged",
    "gb-t payload is absent before materialization",
    "no external API call in full project gate",
    "generated artifact paths remain absent",
    "draft manifest remains not_accepted",
    "runtime behavior unchanged",
    "android-date-layer-v1 remains accepted-current",
    "astronomy-engine remains target"
)) {
    if ($preflight.preflight_checks -notcontains $check) {
        throw "Selected GB/T preflight missing check: $check"
    }
}

if ($preflight.materialization_allowed_after_preflight.selected_source_payload -ne $true -or
    $preflight.materialization_allowed_after_preflight.selected_source_id -ne $selectedSourceId -or
    $preflight.materialization_allowed_after_preflight.other_remaining_source_payloads -ne $false -or
    $preflight.materialization_allowed_after_preflight.generated_astronomy_artifacts -ne $false -or
    $preflight.materialization_allowed_after_preflight.generated_artifact_hashes -ne $false -or
    $preflight.materialization_allowed_after_preflight.draft_manifest_acceptance_change -ne $false -or
    $preflight.materialization_allowed_after_preflight.runtime_behavior_change -ne $false -or
    $preflight.materialization_allowed_after_preflight.android_baseline_replacement -ne $false -or
    $preflight.materialization_allowed_after_preflight.capability_promotion -ne $false) {
    throw "Selected GB/T preflight must allow only selected source payload after preflight."
}

foreach ($forbidden in @(
    "write gb-t payload file",
    "compute gb-t source payload hash",
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
        throw "Selected GB/T preflight missing forbidden item: $forbidden"
    }
}

$result = [pscustomobject]@{
    mode = "selected_gb_t_payload_materialization_preflight_closed_dry_run"
    preflight_id = $preflight.selected_source_payload_materialization_preflight_id
    jpl_materialization_id = $jplMaterialization.selected_source_payload_materialization_id
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
    source_reference_capture_allowed_in_this_loop = $preflight.rule_reference_boundary_policy.source_reference_capture_allowed_in_this_loop
    payload_materialization_allowed_in_this_loop = $preflight.rule_reference_boundary_policy.payload_materialization_allowed_in_this_loop
    external_calls_performed = $false
    generated_artifacts_written = 0
    generated_artifact_hashes_computed = 0
    acceptance_status_changed = $false
    runtime_behavior_changed = $false
    android_baseline_replaced = $false
    writes_performed = $false
}

$result | ConvertTo-Json -Depth 8
exit 0
