[CmdletBinding()]
param(
    [string]$ProjectRoot
)

$ErrorActionPreference = "Stop"

# DRY_RUN_ONLY: M10 LOOP-050 post-IAU remaining source payload strategy inspection.
# This script must not create JPL/GB payload files, compute new source hashes, call external sources, or write generated artifacts.

if ([string]::IsNullOrWhiteSpace($ProjectRoot)) {
    $ProjectRoot = Join-Path $PSScriptRoot ".."
}

$resolvedProject = Resolve-Path -LiteralPath $ProjectRoot
$projectPath = $resolvedProject.ProviderPath
$naifSourceId = "naif-cspice"
$iauSourceId = "iau-sofa-ansi-c"
$jplSourceId = "jpl-horizons-api"
$gbtSourceId = "gb-t-33661-2017"
$naifHash = "4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2"
$iauHash = "436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f"
$jplHash = "acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9"
$gbtHash = "7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31"

function Read-Json {
    param([string]$RelativePath)
    $path = Join-Path $projectPath $RelativePath
    if (-not (Test-Path -LiteralPath $path)) {
        throw "Missing post-IAU remaining source payload strategy file: $RelativePath"
    }
    return Get-Content -LiteralPath $path -Encoding UTF8 -Raw | ConvertFrom-Json
}

$manifest = Read-Json "data/generated/astronomy/source-snapshots/source-snapshot-manifest.json"
$policy = Read-Json "data/generated/astronomy/source-payload-materialization-policy.json"
$procedure = Read-Json "data/generated/astronomy/source-capture-procedure.json"
$previousStrategy = Read-Json "data/generated/astronomy/remaining-source-payload-strategy.json"
$iauMaterialization = Read-Json "data/generated/astronomy/selected-iau-sofa-payload-materialization.json"
$jplMaterialization = Read-Json "data/generated/astronomy/selected-jpl-horizons-payload-materialization.json"
$gbtMaterialization = Read-Json "data/generated/astronomy/selected-gb-t-payload-materialization.json"
$strategy = Read-Json "data/generated/astronomy/post-iau-remaining-source-payload-strategy.json"
$draftManifest = Read-Json "data/generated/astronomy/manifests/astronomy-engine-v0-draft.json"

if ($strategy.status -ne "strategy_decision_only") {
    throw "Post-IAU remaining source payload strategy must remain strategy_decision_only."
}

if ($strategy.source_payload_materialization_policy_id -ne $policy.source_payload_materialization_policy_id -or
    $strategy.source_capture_procedure_id -ne $procedure.source_capture_procedure_id -or
    $strategy.source_snapshot_manifest_id -ne $manifest.source_snapshot_manifest_id -or
    $strategy.selected_iau_sofa_payload_materialization_id -ne $iauMaterialization.selected_source_payload_materialization_id -or
    $strategy.previous_remaining_source_payload_strategy_id -ne $previousStrategy.remaining_source_payload_strategy_id) {
    throw "Post-IAU remaining source payload strategy must reference active policy, procedure, manifest, IAU materialization, and previous strategy."
}

if ($manifest.status -ne "selected_source_payload_materialized" -or
    $policy.status -ne "selected_source_payload_materialized" -or
    $procedure.status -ne "selected_source_payload_materialized" -or
    $iauMaterialization.status -ne "selected_source_payload_materialized") {
    throw "Post-IAU strategy requires the current selected-source materialization state."
}

if (@($strategy.currently_materialized_sources).Count -ne 2) {
    throw "Post-IAU strategy must record exactly two materialized sources."
}

foreach ($expected in @(
    @{ source_id = $naifSourceId; sha256 = $naifHash },
    @{ source_id = $iauSourceId; sha256 = $iauHash }
)) {
    $strategySource = @($strategy.currently_materialized_sources | Where-Object { $_.source_id -eq $expected.source_id })
    $policyPayload = @($policy.planned_payloads | Where-Object { $_.source_id -eq $expected.source_id })
    $procedureSource = @($procedure.procedures | Where-Object { $_.source_id -eq $expected.source_id })
    $manifestSource = @($manifest.sources | Where-Object { $_.source_id -eq $expected.source_id })
    if ($strategySource.Count -ne 1 -or $strategySource[0].sha256 -ne $expected.sha256) {
        throw "Post-IAU strategy materialized source mismatch: $($expected.source_id)"
    }
    if ($policyPayload.Count -ne 1 -or
        $policyPayload[0].payload_status -ne "materialized" -or
        $policyPayload[0].hash_status -ne "computed" -or
        $policyPayload[0].sha256 -ne $expected.sha256) {
        throw "Policy materialized source mismatch: $($expected.source_id)"
    }
    if ($procedureSource.Count -ne 1 -or
        $procedureSource[0].hash_status -ne "computed" -or
        $procedureSource[0].sha256 -ne $expected.sha256) {
        throw "Procedure materialized source mismatch: $($expected.source_id)"
    }
    if ($manifestSource.Count -ne 1 -or
        $manifestSource[0].source_payload_hash.value -ne $expected.sha256) {
        throw "Manifest materialized source mismatch: $($expected.source_id)"
    }

    $payloadPath = Join-Path $projectPath $policyPayload[0].path
    if (-not (Test-Path -LiteralPath $payloadPath)) {
        throw "Materialized payload file is missing: $($policyPayload[0].path)"
    }
    $actualHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $payloadPath).Hash.ToLowerInvariant()
    if ($actualHash -ne $expected.sha256) {
        throw "Materialized payload hash changed for $($expected.source_id): $actualHash"
    }
}

$existingPayloadFiles = @()
foreach ($payload in $policy.planned_payloads) {
    $payloadPath = Join-Path $projectPath $payload.path
    if (Test-Path -LiteralPath $payloadPath) {
        $existingPayloadFiles += $payload.path
    }

    if ($payload.source_id -eq $jplSourceId) {
        if ($payload.payload_status -ne "materialized" -or $payload.hash_status -ne "computed" -or $payload.sha256 -ne $jplHash) {
            throw "JPL Horizons source payload must be materialized after LOOP-052."
        }
        if (-not (Test-Path -LiteralPath $payloadPath)) {
            throw "JPL Horizons source payload file must exist after LOOP-052: $($payload.path)"
        }
        $actualJplHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $payloadPath).Hash.ToLowerInvariant()
        if ($actualJplHash -ne $jplHash) {
            throw "JPL Horizons source payload hash changed after LOOP-052: $actualJplHash"
        }
    }

    if ($payload.source_id -eq $gbtSourceId) {
        if ($payload.payload_status -ne "materialized" -or $payload.hash_status -ne "computed" -or $payload.sha256 -ne $gbtHash) {
            throw "GB/T source payload must be materialized after LOOP-054."
        }
        if (-not (Test-Path -LiteralPath $payloadPath)) {
            throw "GB/T source payload file must exist after LOOP-054: $($payload.path)"
        }
        $actualGbtHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $payloadPath).Hash.ToLowerInvariant()
        if ($actualGbtHash -ne $gbtHash) {
            throw "GB/T source payload hash changed after LOOP-054: $actualGbtHash"
        }
    }
}

if ($gbtMaterialization.status -ne "selected_source_payload_materialized" -or
    $gbtMaterialization.selected_source.source_id -ne $gbtSourceId -or
    $gbtMaterialization.selected_source.sha256 -ne $gbtHash) {
    throw "GB/T materialization evidence must record the expected source hash."
}

if ($jplMaterialization.status -ne "selected_source_payload_materialized" -or
    $jplMaterialization.selected_source.source_id -ne $jplSourceId -or
    $jplMaterialization.selected_source.sha256 -ne $jplHash) {
    throw "JPL materialization evidence must record the expected source hash."
}

if ($existingPayloadFiles.Count -ne 4) {
    throw "Exactly four payload files must exist after LOOP-054."
}

if (@($strategy.remaining_source_sequence).Count -ne 2) {
    throw "Post-IAU strategy must order exactly JPL Horizons and GB/T."
}

$sequenceIds = @($strategy.remaining_source_sequence | Sort-Object order | ForEach-Object { $_.source_id })
if ($sequenceIds[0] -ne $jplSourceId -or $sequenceIds[1] -ne $gbtSourceId) {
    throw "Post-IAU strategy must select JPL Horizons first, then GB/T."
}

$nextPayload = @($policy.planned_payloads | Where-Object { $_.source_id -eq $jplSourceId })
$nextProcedure = @($procedure.procedures | Where-Object { $_.source_id -eq $jplSourceId })
$nextManifestSource = @($manifest.sources | Where-Object { $_.source_id -eq $jplSourceId })
if ($nextPayload.Count -ne 1 -or $nextProcedure.Count -ne 1 -or $nextManifestSource.Count -ne 1) {
    throw "Post-IAU next selected source must exist in policy, procedure, and manifest."
}

if ($strategy.next_selected_source.source_id -ne $jplSourceId -or
    $strategy.next_selected_source.payload_kind -ne $nextPayload[0].payload_kind -or
    $strategy.next_selected_source.schema_path -ne $nextPayload[0].schema_path -or
    $strategy.next_selected_source.payload_path -ne $nextPayload[0].path -or
    $strategy.next_selected_source.payload_format -ne $nextPayload[0].payload_format) {
    throw "Post-IAU next selected source must match payload policy."
}

$nextSchemaPath = Join-Path $projectPath $strategy.next_selected_source.schema_path
if (-not (Test-Path -LiteralPath $nextSchemaPath)) {
    throw "Post-IAU next selected source schema missing: $($strategy.next_selected_source.schema_path)"
}
$nextSchema = Get-Content -LiteralPath $nextSchemaPath -Encoding UTF8 -Raw | ConvertFrom-Json
if ($nextSchema.status -ne "schema_only" -or
    $nextSchema.source_id -ne $jplSourceId -or
    $nextSchema.payload_kind -ne $strategy.next_selected_source.payload_kind) {
    throw "Post-IAU next selected source schema must remain schema_only and match strategy."
}

if ($strategy.allowed_next_loop.selected_source_payload_preflight -ne $true -or
    $strategy.allowed_next_loop.selected_source_id -ne $jplSourceId -or
    $strategy.allowed_next_loop.payload_materialization -ne $false -or
    $strategy.allowed_next_loop.payload_hash_computation -ne $false -or
    $strategy.allowed_next_loop.generated_astronomy_artifacts -ne $false -or
    $strategy.allowed_next_loop.generated_artifact_hashes -ne $false -or
    $strategy.allowed_next_loop.draft_manifest_acceptance_change -ne $false -or
    $strategy.allowed_next_loop.runtime_behavior_change -ne $false -or
    $strategy.allowed_next_loop.capability_promotion -ne $false) {
    throw "Post-IAU strategy must allow only next-loop JPL preflight."
}

if ($draftManifest.acceptance_status -ne "not_accepted") {
    throw "Draft manifest must remain not_accepted during post-IAU strategy."
}

foreach ($check in @(
    "exactly two source payloads are materialized before post-IAU strategy decision",
    "naif-cspice payload hash remains unchanged",
    "iau-sofa payload hash remains unchanged",
    "JPL Horizons payload is absent before preflight",
    "GB/T payload is absent before preflight",
    "next source is selected from remaining sources",
    "next loop is preflight-only",
    "no external API call in full project gate",
    "generated artifact paths remain absent",
    "draft manifest remains not_accepted",
    "runtime behavior unchanged",
    "astronomy-engine remains target"
)) {
    if ($strategy.strategy_checks -notcontains $check) {
        throw "Post-IAU strategy missing check: $check"
    }
}

foreach ($forbidden in @(
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
        throw "Post-IAU strategy missing forbidden item: $forbidden"
    }
}

$result = [pscustomobject]@{
    mode = "post_iau_remaining_source_payload_strategy_closed_dry_run"
    strategy_id = $strategy.post_iau_remaining_source_payload_strategy_id
    previous_strategy_id = $previousStrategy.remaining_source_payload_strategy_id
    iau_sofa_materialization_id = $iauMaterialization.selected_source_payload_materialization_id
    jpl_horizons_materialization_id = $jplMaterialization.selected_source_payload_materialization_id
    gbt_materialization_id = $gbtMaterialization.selected_source_payload_materialization_id
    materialized_source_count = 4
    materialized_source_ids = @($naifSourceId, $iauSourceId, $jplSourceId, $gbtSourceId)
    materialized_payload_hashes = @{
        "naif-cspice" = $naifHash
        "iau-sofa-ansi-c" = $iauHash
        "jpl-horizons-api" = $jplHash
        "gb-t-33661-2017" = $gbtHash
    }
    remaining_source_count = 0
    next_selected_source_id = $jplSourceId
    next_selected_payload_kind = "validation-query-snapshot-set"
    next_loop_action = "selected_source_payload_materialized_in_loop_052"
    existing_payload_count = $existingPayloadFiles.Count
    existing_payload_files = $existingPayloadFiles
    source_payloads_materialized = 4
    new_source_payloads_written = 2
    new_source_payload_hashes_computed = 2
    external_calls_performed = $false
    generated_artifacts_written = 0
    generated_artifact_hashes_computed = 0
    acceptance_status_changed = $false
    runtime_behavior_changed = $false
    writes_performed = $false
}

$result | ConvertTo-Json -Depth 8
exit 0
