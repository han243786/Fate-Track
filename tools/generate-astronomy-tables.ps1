[CmdletBinding()]
param(
    [string]$ProjectRoot,
    [string]$Manifest = "data/generated/astronomy/manifests/astronomy-engine-v0-draft.json",
    [switch]$DryRun,
    [switch]$PrepareImplementation,
    [switch]$AllowSourceSnapshotUse,
    [switch]$AllowArtifactWrite,
    [switch]$AllowManifestUpdate
)

$ErrorActionPreference = "Stop"

# DRY_RUN_ONLY: M9 LOOP-024 skeleton remains available for preflight inspection.
# GUARDED_IMPLEMENTATION_ENTRY: M10 LOOP-038 allows non-dry-run entry inspection,
# but still forbids artifact writes, manifest updates, and capability promotion.

if ([string]::IsNullOrWhiteSpace($ProjectRoot)) {
    $ProjectRoot = Join-Path $PSScriptRoot ".."
}

$resolvedProject = Resolve-Path -LiteralPath $ProjectRoot
$projectPath = $resolvedProject.ProviderPath

if (-not $DryRun -and -not $PrepareImplementation) {
    throw "Generation materialization is not enabled. Use -DryRun for preflight inspection or -PrepareImplementation for the guarded M10 entrypoint."
}

if ($DryRun -and $PrepareImplementation) {
    throw "Choose either -DryRun or -PrepareImplementation, not both."
}

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

$sourcePolicy = Read-Json "data/generated/astronomy/source-policy.json"
$generationPlan = Read-Json "data/generated/astronomy/generation-plan.json"
$generatorContract = Read-Json "data/generated/astronomy/generator-contract.json"
$entryPlan = Read-Json "data/generated/astronomy/generator-implementation-entry.json"
$manifestDoc = Read-Json $relativeManifest

if ($generationPlan.status -ne "draft_not_runnable") {
    throw "Generation plan is not in draft_not_runnable status."
}

if ($generatorContract.status -ne "contract_only") {
    throw "Generator contract must remain contract_only before generated rows exist."
}

if ($generatorContract.hash_algorithm -ne "sha256") {
    throw "Generator contract must require sha256 hashes."
}

if ($generatorContract.manifest_id -ne $manifestDoc.manifest_id) {
    throw "Generator contract must reference the active manifest."
}

if ($generationPlan.intended_command.script_status -ne "dry_run_only") {
    throw "Generation plan must declare the script as dry_run_only."
}

if ($entryPlan.status -ne "guarded_entrypoint_only") {
    throw "Generator implementation entry must remain guarded_entrypoint_only."
}

if ($entryPlan.capability_status -ne "target") {
    throw "Generator implementation entry must keep astronomy-engine target."
}

if ($manifestDoc.acceptance_status -ne "not_accepted") {
    throw "Manifest must remain not_accepted during dry-run."
}

$outDir = Join-Path $projectPath "data/generated/astronomy/out"
$existingArtifacts = @()
foreach ($artifact in $generationPlan.planned_artifacts) {
    $artifactPath = Join-Path $projectPath $artifact.path
    if (Test-Path -LiteralPath $artifactPath) {
        $existingArtifacts += $artifact.path
    }
}

foreach ($artifact in $generatorContract.planned_outputs) {
    if ($artifact.status -ne "not_generated") {
        throw "Generator contract outputs must remain not_generated: $($artifact.path)"
    }
    if (-not $artifact.hash_required) {
        throw "Generator contract output must require hash: $($artifact.path)"
    }
}

if ($PrepareImplementation) {
    $sourceSnapshotManifest = Join-Path $projectPath "data/generated/astronomy/source-snapshots/source-snapshot-manifest.json"
    $sourceSnapshotManifestExists = Test-Path -LiteralPath $sourceSnapshotManifest

    if ($AllowSourceSnapshotUse -or $AllowArtifactWrite -or $AllowManifestUpdate) {
        throw "M10 guarded entry does not allow source use, artifact writes, or manifest updates until the source snapshot manifest and acceptance gates exist."
    }

    $entryResult = [pscustomobject]@{
        mode = "implementation_entry_guarded"
        dry_run = $false
        source_policy_id = $sourcePolicy.source_policy_id
        manifest_id = $manifestDoc.manifest_id
        generator_contract_id = $generatorContract.generator_contract_id
        implementation_entry_id = $entryPlan.implementation_entry_id
        capability_status = $entryPlan.capability_status
        source_snapshot_manifest = "data/generated/astronomy/source-snapshots/source-snapshot-manifest.json"
        source_snapshot_manifest_exists = $sourceSnapshotManifestExists
        requested_guard_flags = [pscustomobject]@{
            allow_source_snapshot_use = [bool]$AllowSourceSnapshotUse
            allow_artifact_write = [bool]$AllowArtifactWrite
            allow_manifest_update = [bool]$AllowManifestUpdate
        }
        generation_blocked = $true
        block_reasons = $entryPlan.block_reasons
        planned_artifact_count = @($generationPlan.planned_artifacts).Count
        writes_performed = $false
        hashes_computed = 0
        acceptance_status_changed = $false
        runtime_behavior_changed = $false
    }

    $entryResult | ConvertTo-Json -Depth 8
    exit 0
}

$result = [pscustomobject]@{
    mode = "dry_run_only"
    source_policy_id = $sourcePolicy.source_policy_id
    manifest_id = $manifestDoc.manifest_id
    generator_contract_id = $generatorContract.generator_contract_id
    hash_algorithm = $generatorContract.hash_algorithm
    manifest_status = $manifestDoc.acceptance_status
    generation_plan_id = $generationPlan.generation_plan_id
    generation_plan_status = $generationPlan.status
    planned_artifact_count = @($generationPlan.planned_artifacts).Count
    planned_artifacts = $generationPlan.planned_artifacts
    output_directory = "data/generated/astronomy/out"
    output_directory_exists = (Test-Path -LiteralPath $outDir)
    existing_planned_artifacts = $existingArtifacts
    writes_performed = $false
    acceptance_status_changed = $false
}

$result | ConvertTo-Json -Depth 8
exit 0
