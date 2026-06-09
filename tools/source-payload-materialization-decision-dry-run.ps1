[CmdletBinding()]
param(
    [string]$ProjectRoot
)

$ErrorActionPreference = "Stop"

# DRY_RUN_ONLY: M10 first source payload materialization decision inspection.
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
        throw "Missing astronomy source payload decision planning file: $RelativePath"
    }
    return Get-Content -LiteralPath $path -Encoding UTF8 -Raw | ConvertFrom-Json
}

$manifest = Read-Json "data/generated/astronomy/source-snapshots/source-snapshot-manifest.json"
$policy = Read-Json "data/generated/astronomy/source-payload-materialization-policy.json"
$procedure = Read-Json "data/generated/astronomy/source-capture-procedure.json"
$decision = Read-Json "data/generated/astronomy/source-payload-materialization-decision.json"
$expectedHash = "4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2"

if ($decision.status -ne "decision_only") {
    throw "Source payload materialization decision must remain decision_only."
}

if ($decision.decision -ne "select_first_source_payload_candidate") {
    throw "Source payload materialization decision must select the first source payload candidate."
}

if ($decision.source_snapshot_manifest_id -ne $manifest.source_snapshot_manifest_id -or
    $decision.source_payload_materialization_policy_id -ne $policy.source_payload_materialization_policy_id -or
    $decision.source_capture_procedure_id -ne $procedure.source_capture_procedure_id) {
    throw "Source payload materialization decision must reference active manifest, policy, and procedure."
}

if ($decision.decision_scope -ne "single_source_only") {
    throw "Source payload materialization decision must stay single_source_only."
}

if ($decision.payload_materialization_allowed_in_this_loop -ne $false -or
    $decision.payload_directory_creation_allowed_in_this_loop -ne $false -or
    $decision.payload_hash_computation_allowed_in_this_loop -ne $false -or
    $decision.generated_artifact_allowed_in_this_loop -ne $false -or
    $decision.generated_artifact_hash_allowed_in_this_loop -ne $false -or
    $decision.manifest_acceptance_change_allowed_in_this_loop -ne $false -or
    $decision.runtime_behavior_change_allowed_in_this_loop -ne $false -or
    $decision.capability_status -ne "target") {
    throw "Source payload materialization decision must not allow payload writes, hashes, artifacts, acceptance changes, runtime changes, or capability promotion."
}

$selectedSourceId = $decision.selected_source.source_id
$payloadMatch = @($policy.planned_payloads | Where-Object { $_.source_id -eq $selectedSourceId })
$procedureMatch = @($procedure.procedures | Where-Object { $_.source_id -eq $selectedSourceId })
$manifestMatch = @($manifest.sources | Where-Object { $_.source_id -eq $selectedSourceId })
if ($payloadMatch.Count -ne 1 -or $procedureMatch.Count -ne 1 -or $manifestMatch.Count -ne 1) {
    throw "Selected source must exist in policy, procedure, and manifest: $selectedSourceId"
}

$payload = $payloadMatch[0]
$sourceProcedure = $procedureMatch[0]
if ($decision.selected_source.payload_kind -ne $payload.payload_kind -or
    $decision.selected_source.schema_path -ne $payload.schema_path -or
    $decision.selected_source.payload_path -ne $payload.path -or
    $decision.selected_source.payload_format -ne $payload.payload_format) {
    throw "Selected source decision must match payload policy fields: $selectedSourceId"
}

if ($sourceProcedure.capture_status -ne "completed_for_boundary_payload" -or
    $sourceProcedure.materialization_status -ne "source_boundary_payload_materialized" -or
    $sourceProcedure.hash_status -ne "computed" -or
    $sourceProcedure.sha256 -ne $expectedHash) {
    throw "Selected source procedure must record selected-source materialization: $selectedSourceId"
}

$schemaPath = Join-Path $projectPath $decision.selected_source.schema_path
if (-not (Test-Path -LiteralPath $schemaPath)) {
    throw "Selected source schema missing: $($decision.selected_source.schema_path)"
}
$schema = Get-Content -LiteralPath $schemaPath -Encoding UTF8 -Raw | ConvertFrom-Json
if ($schema.status -ne "schema_only" -or
    $schema.source_id -ne $selectedSourceId -or
    $schema.payload_kind -ne $decision.selected_source.payload_kind) {
    throw "Selected source schema must remain schema_only and match decision: $selectedSourceId"
}

$payloadDirectory = Join-Path $projectPath $policy.payload_directory.path
$selectedPayloadPath = Join-Path $projectPath $decision.selected_source.payload_path
$existingPayloadFiles = @()
foreach ($plannedPayload in $policy.planned_payloads) {
    $plannedPayloadPath = Join-Path $projectPath $plannedPayload.path
    if (Test-Path -LiteralPath $plannedPayloadPath) {
        $existingPayloadFiles += $plannedPayload.path
    }
}

if (-not (Test-Path -LiteralPath $payloadDirectory)) {
    throw "Source payload directory must exist after selected-source materialization: $($policy.payload_directory.path)"
}

if (-not (Test-Path -LiteralPath $selectedPayloadPath)) {
    throw "Selected source payload file must exist after selected-source materialization: $($decision.selected_source.payload_path)"
}

$actualHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $selectedPayloadPath).Hash.ToLowerInvariant()
if ($actualHash -ne $expectedHash) {
    throw "Selected source payload hash mismatch in decision dry-run: $actualHash"
}

if ($existingPayloadFiles.Count -ne 4 -or
    $existingPayloadFiles -notcontains $decision.selected_source.payload_path -or
    $existingPayloadFiles -notcontains "data/generated/astronomy/source-snapshots/payloads/iau-sofa-routine-version.json" -or
    $existingPayloadFiles -notcontains "data/generated/astronomy/source-snapshots/payloads/jpl-horizons-validation-samples.json" -or
    $existingPayloadFiles -notcontains "data/generated/astronomy/source-snapshots/payloads/gb-t-33661-2017-rule-reference.json") {
    throw "Selected NAIF, IAU SOFA, JPL Horizons, and GB/T source payload files must exist after LOOP-054: $($existingPayloadFiles -join ', ')"
}

$result = [pscustomobject]@{
    mode = "source_payload_materialization_decision_selected_payload_dry_run"
    decision_id = $decision.source_payload_materialization_decision_id
    policy_id = $policy.source_payload_materialization_policy_id
    procedure_id = $procedure.source_capture_procedure_id
    manifest_id = $manifest.source_snapshot_manifest_id
    selected_source_id = $selectedSourceId
    selected_payload_kind = $decision.selected_source.payload_kind
    selected_payload_path = $decision.selected_source.payload_path
    payload_directory = $policy.payload_directory.path
    payload_directory_exists = (Test-Path -LiteralPath $payloadDirectory)
    selected_payload_exists = (Test-Path -LiteralPath $selectedPayloadPath)
    planned_payload_count = @($policy.planned_payloads).Count
    existing_payload_files = $existingPayloadFiles
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
