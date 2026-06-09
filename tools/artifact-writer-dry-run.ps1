[CmdletBinding()]
param(
    [string]$ProjectRoot
)

$ErrorActionPreference = "Stop"

# DRY_RUN_ONLY: M9 LOOP-033 artifact writer scaffold. This script must not create directories or files.

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

$writerPlan = Read-Json "data/generated/astronomy/artifact-writer-plan.json"
$generatorContract = Read-Json "data/generated/astronomy/generator-contract.json"
$sourceAdapterContract = Read-Json "data/generated/astronomy/source-adapter-contract.json"

if ($writerPlan.status -ne "dry_run_only") {
    throw "Artifact writer plan must remain dry_run_only."
}

if ($writerPlan.write_mode -ne "no_write_preview") {
    throw "Artifact writer plan must remain no_write_preview."
}

if ($writerPlan.hash_algorithm -ne $generatorContract.hash_algorithm -or $writerPlan.hash_algorithm -ne "sha256") {
    throw "Artifact writer plan must use the active sha256 generator contract."
}

if ($writerPlan.generator_contract_id -ne $generatorContract.generator_contract_id) {
    throw "Artifact writer plan must reference the active generator contract."
}

if ($writerPlan.source_adapter_contract_id -ne $sourceAdapterContract.source_adapter_contract_id) {
    throw "Artifact writer plan must reference the active source adapter contract."
}

$outDirPath = Join-Path $projectPath $writerPlan.output_directory
$existingArtifacts = @()
$planned = @()

foreach ($artifact in $writerPlan.planned_artifacts) {
    if ($artifact.hash_status -ne "not_computed" -or $artifact.write_status -ne "not_written") {
        throw "Artifact writer dry-run artifacts must remain not_computed/not_written: $($artifact.path)"
    }

    $contractMatch = @($generatorContract.planned_outputs | Where-Object { $_.path -eq $artifact.path -and $_.kind -eq $artifact.kind })
    if ($contractMatch.Count -ne 1) {
        throw "Artifact writer plan output is not in generator contract: $($artifact.path)"
    }

    $artifactPath = Join-Path $projectPath $artifact.path
    if (Test-Path -LiteralPath $artifactPath) {
        $existingArtifacts += $artifact.path
    }

    $planned += [pscustomobject]@{
        path = $artifact.path
        kind = $artifact.kind
        hash_algorithm = $writerPlan.hash_algorithm
        hash_status = $artifact.hash_status
        write_status = $artifact.write_status
    }
}

$result = [pscustomobject]@{
    mode = "artifact_writer_dry_run_only"
    artifact_writer_plan_id = $writerPlan.artifact_writer_plan_id
    generator_contract_id = $writerPlan.generator_contract_id
    source_adapter_contract_id = $writerPlan.source_adapter_contract_id
    output_directory = $writerPlan.output_directory
    output_directory_exists = (Test-Path -LiteralPath $outDirPath)
    planned_artifact_count = @($writerPlan.planned_artifacts).Count
    planned_artifacts = $planned
    existing_planned_artifacts = $existingArtifacts
    writes_performed = $false
    hashes_computed = 0
    accepted_evidence = $false
}

$result | ConvertTo-Json -Depth 8
exit 0
