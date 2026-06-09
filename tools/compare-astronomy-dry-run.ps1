[CmdletBinding()]
param(
    [string]$ProjectRoot,
    [string]$Manifest = "data/generated/astronomy/manifests/astronomy-engine-v0-draft.json"
)

$ErrorActionPreference = "Stop"

# DRY_RUN_ONLY: M9 LOOP-026 comparison scaffold. This script must not create comparison rows yet.

if ([string]::IsNullOrWhiteSpace($ProjectRoot)) {
    $ProjectRoot = Join-Path $PSScriptRoot ".."
}

$resolvedProject = Resolve-Path -LiteralPath $ProjectRoot
$projectPath = $resolvedProject.ProviderPath

function Read-Json {
    param([string]$RelativePath)
    $path = Join-Path $projectPath $RelativePath
    if (-not (Test-Path -LiteralPath $path)) {
        throw "Missing astronomy planning file: $RelativePath"
    }
    return Get-Content -LiteralPath $path -Encoding UTF8 -Raw | ConvertFrom-Json
}

$relativeManifest = $Manifest
if ([System.IO.Path]::IsPathRooted($Manifest)) {
    $manifestPath = Resolve-Path -LiteralPath $Manifest
    $relativeManifest = [System.IO.Path]::GetRelativePath($projectPath, $manifestPath.ProviderPath)
}

$schema = Read-Json "data/generated/astronomy/comparison.schema.json"
$runnerPlan = Read-Json "data/generated/astronomy/comparison-runner-plan.json"
$artifactWriterPlan = Read-Json "data/generated/astronomy/artifact-writer-plan.json"
$manifestDoc = Read-Json $relativeManifest

if ($schema.status -ne "schema_only") {
    throw "Comparison schema must remain schema_only before generated rows exist."
}

if ($runnerPlan.status -ne "dry_run_only") {
    throw "Comparison runner plan must remain dry_run_only."
}

if ($runnerPlan.comparison_schema_id -ne $schema.schema_id) {
    throw "Comparison runner plan must reference the active schema."
}

if ($runnerPlan.artifact_writer_plan_id -ne $artifactWriterPlan.artifact_writer_plan_id) {
    throw "Comparison runner plan must reference the active artifact writer plan."
}

if ($runnerPlan.dry_run_result_policy.rows_compared -ne 0 -or $runnerPlan.dry_run_result_policy.difference_rows -ne 0) {
    throw "Comparison runner dry-run policy must keep zero rows."
}

if ($manifestDoc.acceptance_status -ne "not_accepted") {
    throw "Manifest must remain not_accepted during comparison dry-run."
}

$comparison = [pscustomobject]@{
    mode = "comparison_dry_run_only"
    comparison_runner_plan_id = $runnerPlan.comparison_runner_plan_id
    comparison_id = "android-astronomy-comparison-dry-run"
    manifest_id = $manifestDoc.manifest_id
    source_policy_id = $manifestDoc.source_policy_id
    android_algorithm_version = $runnerPlan.android_algorithm_version
    android_ruleset_id = $runnerPlan.android_ruleset_id
    astronomy_engine_version = $manifestDoc.engine_version
    generated_range = $manifestDoc.generated_range
    future_comparison_artifact = $runnerPlan.future_comparison_artifact
    required_bindings = $runnerPlan.required_bindings
    rows_compared = 0
    difference_summary = [pscustomobject]@{
        android_table_difference = 0
        astronomy_source_difference = 0
        ruleset_difference = 0
        timezone_history_difference = 0
        unresolved = 0
    }
    difference_rows = @()
    created_at_utc = "not_created"
    writes_performed = $false
    accepted_evidence = $false
}

$comparison | ConvertTo-Json -Depth 8
exit 0
