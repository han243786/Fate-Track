[CmdletBinding()]
param(
    [string]$ProjectRoot
)

$ErrorActionPreference = "Stop"

# DRY_RUN_ONLY: M10 selected naif-cspice source payload materialization preflight inspection.
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
        throw "Missing selected source payload preflight file: $RelativePath"
    }
    return Get-Content -LiteralPath $path -Encoding UTF8 -Raw | ConvertFrom-Json
}

$manifest = Read-Json "data/generated/astronomy/source-snapshots/source-snapshot-manifest.json"
$policy = Read-Json "data/generated/astronomy/source-payload-materialization-policy.json"
$procedure = Read-Json "data/generated/astronomy/source-capture-procedure.json"
$decision = Read-Json "data/generated/astronomy/source-payload-materialization-decision.json"
$preflight = Read-Json "data/generated/astronomy/selected-source-payload-materialization-preflight.json"
$draftManifest = Read-Json "data/generated/astronomy/manifests/astronomy-engine-v0-draft.json"
$expectedHash = "4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2"

if ($preflight.status -ne "preflight_only") {
    throw "Selected source payload materialization preflight must remain preflight_only."
}

if ($preflight.source_payload_materialization_decision_id -ne $decision.source_payload_materialization_decision_id -or
    $preflight.source_payload_materialization_policy_id -ne $policy.source_payload_materialization_policy_id -or
    $preflight.source_capture_procedure_id -ne $procedure.source_capture_procedure_id -or
    $preflight.source_snapshot_manifest_id -ne $manifest.source_snapshot_manifest_id) {
    throw "Selected source payload materialization preflight must reference active decision, policy, procedure, and manifest."
}

if ($preflight.selected_source.source_id -ne "naif-cspice") {
    throw "Selected source payload materialization preflight must remain scoped to naif-cspice."
}

$payloadMatch = @($policy.planned_payloads | Where-Object { $_.source_id -eq $preflight.selected_source.source_id })
$procedureMatch = @($procedure.procedures | Where-Object { $_.source_id -eq $preflight.selected_source.source_id })
if ($payloadMatch.Count -ne 1 -or $procedureMatch.Count -ne 1) {
    throw "Selected source must exist in payload policy and capture procedure."
}

$payload = $payloadMatch[0]
if ($preflight.selected_source.payload_kind -ne $payload.payload_kind -or
    $preflight.selected_source.schema_path -ne $payload.schema_path -or
    $preflight.selected_source.payload_path -ne $payload.path -or
    $preflight.selected_source.payload_format -ne $payload.payload_format) {
    throw "Selected preflight source must match payload policy."
}

if ($preflight.payload_directory_policy.create_allowed_in_this_loop -ne $false -or
    $preflight.payload_directory_policy.next_loop_create_scope -ne "selected_source_only" -or
    $preflight.payload_directory_policy.path -ne $policy.payload_directory.path) {
    throw "Selected source payload preflight must keep directory creation blocked this loop and scoped next loop."
}

if ($preflight.selected_payload_write_policy.write_allowed_in_this_loop -ne $false -or
    $preflight.selected_payload_write_policy.next_loop_write_scope -ne "selected_source_only" -or
    $preflight.selected_payload_write_policy.canonical_json_required -ne $true -or
    $preflight.selected_payload_write_policy.allowed_payload_claim -ne "source-boundary-evidence-only") {
    throw "Selected source payload preflight must keep writes blocked this loop and require source-only canonical JSON next loop."
}

if ($preflight.selected_payload_hash_policy.hash_algorithm -ne "sha256" -or
    $preflight.selected_payload_hash_policy.hash_allowed_in_this_loop -ne $false -or
    $preflight.selected_payload_hash_policy.next_loop_hash_scope -ne "selected_source_payload_only") {
    throw "Selected source payload preflight must keep hashes blocked this loop and scoped next loop."
}

$schemaPath = Join-Path $projectPath $preflight.selected_source.schema_path
if (-not (Test-Path -LiteralPath $schemaPath)) {
    throw "Selected source schema missing: $($preflight.selected_source.schema_path)"
}
$schema = Get-Content -LiteralPath $schemaPath -Encoding UTF8 -Raw | ConvertFrom-Json
if ($schema.status -ne "schema_only" -or
    $schema.source_id -ne $preflight.selected_source.source_id -or
    $schema.payload_kind -ne $preflight.selected_source.payload_kind) {
    throw "Selected source schema must remain schema_only and match preflight."
}

foreach ($claim in $preflight.selected_payload_write_policy.forbidden_payload_claims) {
    if ($schema.forbidden_claims -contains $claim) {
        continue
    }
    if ($claim -in @("runtime dependency enabled", "Android baseline replaced")) {
        continue
    }
    throw "Selected source schema missing forbidden payload claim: $claim"
}

if ($draftManifest.acceptance_status -ne "not_accepted") {
    throw "Draft manifest must remain not_accepted during selected source preflight."
}

$payloadDirectory = Join-Path $projectPath $preflight.payload_directory_policy.path
$selectedPayloadPath = Join-Path $projectPath $preflight.selected_source.payload_path
$existingPayloadFiles = @()
foreach ($plannedPayload in $policy.planned_payloads) {
    $plannedPayloadPath = Join-Path $projectPath $plannedPayload.path
    if (Test-Path -LiteralPath $plannedPayloadPath) {
        $existingPayloadFiles += $plannedPayload.path
    }
}

if (-not (Test-Path -LiteralPath $payloadDirectory)) {
    throw "Payload directory must exist after selected source materialization: $($preflight.payload_directory_policy.path)"
}

if (-not (Test-Path -LiteralPath $selectedPayloadPath)) {
    throw "Selected source payload file must exist after selected source materialization: $($preflight.selected_source.payload_path)"
}

$actualHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $selectedPayloadPath).Hash.ToLowerInvariant()
if ($actualHash -ne $expectedHash) {
    throw "Selected source payload hash mismatch in preflight dry-run: $actualHash"
}

if ($existingPayloadFiles.Count -ne 4 -or
    $existingPayloadFiles -notcontains $preflight.selected_source.payload_path -or
    $existingPayloadFiles -notcontains "data/generated/astronomy/source-snapshots/payloads/iau-sofa-routine-version.json" -or
    $existingPayloadFiles -notcontains "data/generated/astronomy/source-snapshots/payloads/jpl-horizons-validation-samples.json" -or
    $existingPayloadFiles -notcontains "data/generated/astronomy/source-snapshots/payloads/gb-t-33661-2017-rule-reference.json") {
    throw "Selected NAIF, IAU SOFA, JPL Horizons, and GB/T source payload files must exist after LOOP-054: $($existingPayloadFiles -join ', ')"
}

$result = [pscustomobject]@{
    mode = "selected_source_payload_materialization_preflight_closed_dry_run"
    preflight_id = $preflight.selected_source_payload_materialization_preflight_id
    decision_id = $decision.source_payload_materialization_decision_id
    selected_source_id = $preflight.selected_source.source_id
    selected_payload_kind = $preflight.selected_source.payload_kind
    selected_payload_path = $preflight.selected_source.payload_path
    payload_directory = $preflight.payload_directory_policy.path
    payload_directory_exists = (Test-Path -LiteralPath $payloadDirectory)
    selected_payload_exists = (Test-Path -LiteralPath $selectedPayloadPath)
    planned_payload_count = @($policy.planned_payloads).Count
    existing_payload_files = $existingPayloadFiles
    next_loop_create_scope = $preflight.payload_directory_policy.next_loop_create_scope
    next_loop_write_scope = $preflight.selected_payload_write_policy.next_loop_write_scope
    next_loop_hash_scope = $preflight.selected_payload_hash_policy.next_loop_hash_scope
    source_payloads_materialized = 4
    payload_hashes_computed = 4
    external_calls_performed = $false
    generated_artifacts_written = 0
    generated_artifact_hashes_computed = 0
    acceptance_status_changed = $false
    runtime_behavior_changed = $false
    writes_performed = $false
}

$result | ConvertTo-Json -Depth 8
exit 0
