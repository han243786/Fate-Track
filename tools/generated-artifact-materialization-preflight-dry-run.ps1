[CmdletBinding()]
param(
    [string]$ProjectRoot
)

$ErrorActionPreference = "Stop"
$DRY_RUN_MARKER = "DRY_RUN_ONLY"

if ([string]::IsNullOrWhiteSpace($ProjectRoot)) {
    $ProjectRoot = Join-Path $PSScriptRoot ".."
}

$resolvedProject = Resolve-Path -LiteralPath $ProjectRoot
$projectPath = $resolvedProject.ProviderPath

$preflightPath = Join-Path $projectPath "data/generated/astronomy/generated-artifact-materialization-preflight.json"
if (-not (Test-Path -LiteralPath $preflightPath)) {
    throw "Missing generated-artifact-materialization-preflight.json"
}
$preflight = Get-Content -LiteralPath $preflightPath -Encoding UTF8 -Raw | ConvertFrom-Json

# Verify preflight status
if ($preflight.status -ne "preflight_only") {
    throw "Generated artifact materialization preflight must remain preflight_only"
}

if ($preflight.generated_artifact_materialization_preflight_id -ne "m10-generated-artifact-materialization-preflight-v1") {
    throw "Unexpected preflight id"
}

# Verify write policy
if ($preflight.artifact_write_policy.write_allowed_in_this_loop) {
    throw "Artifact writes must not be allowed in preflight loop"
}

if (-not $preflight.artifact_write_policy.next_loop_write_scope -eq "planned_artifacts_only") {
    throw "Next loop write scope must be planned_artifacts_only"
}

if ($preflight.artifact_write_policy.hash_allowed_in_this_loop) {
    throw "Hash computation must not be allowed in preflight loop"
}

# Verify output directory does not exist
$outDir = Join-Path $projectPath "data/generated/astronomy/out"
if (Test-Path -LiteralPath $outDir) {
    throw "Output directory must not exist in preflight stage: $outDir"
}

# Verify no generated artifact files exist
foreach ($artifact in $preflight.planned_generated_artifacts) {
    $artifactPath = Join-Path $projectPath $artifact.path
    if (Test-Path -LiteralPath $artifactPath) {
        throw "Generated artifact must not exist in preflight stage: $($artifact.path)"
    }
    if ($artifact.status -ne "not_generated") {
        throw "Artifact status must be not_generated: $($artifact.path)"
    }
    if ($artifact.hash_status -ne "not_computed") {
        throw "Artifact hash must be not_computed: $($artifact.path)"
    }
}

# Verify all 4 source payload prerequisites
$sourcePayloadDir = Join-Path $projectPath "data/generated/astronomy/source-snapshots/payloads"
$expectedSources = @(
    @{id="naif-cspice"; file="naif-cspice-kernel-boundary.json"; hash="4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2"},
    @{id="iau-sofa-ansi-c"; file="iau-sofa-routine-version.json"; hash="436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f"},
    @{id="jpl-horizons-api"; file="jpl-horizons-validation-samples.json"; hash="acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9"},
    @{id="gb-t-33661-2017"; file="gb-t-33661-2017-rule-reference.json"; hash="7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31"}
)

$materializedCount = 0
foreach ($src in $expectedSources) {
    $payloadFile = Join-Path $sourcePayloadDir $src.file
    if (-not (Test-Path -LiteralPath $payloadFile)) {
        throw "Source payload missing: $($src.file)"
    }
    $prereq = $preflight.source_payload_prerequisites | Where-Object { $_.source_id -eq $src.id }
    if (-not $prereq -or $prereq.status -ne "materialized") {
        throw "Source payload prerequisite not materialized: $($src.id)"
    }
    if ($prereq.sha256 -ne $src.hash) {
        throw "Source payload hash mismatch for: $($src.id)"
    }
    $materializedCount++
}

if ($materializedCount -ne 4) {
    throw "Expected 4 materialized source payloads, found $materializedCount"
}

# Verify generator contract
$generatorContractPath = Join-Path $projectPath "data/generated/astronomy/generator-contract.json"
$generatorContract = Get-Content -LiteralPath $generatorContractPath -Encoding UTF8 -Raw | ConvertFrom-Json
if ($generatorContract.status -ne "contract_only") {
    throw "Generator contract must remain contract_only"
}

# Verify artifact writer plan
$artifactWriterPlanPath = Join-Path $projectPath "data/generated/astronomy/artifact-writer-plan.json"
$artifactWriterPlan = Get-Content -LiteralPath $artifactWriterPlanPath -Encoding UTF8 -Raw | ConvertFrom-Json
if ($artifactWriterPlan.status -ne "dry_run_only") {
    throw "Artifact writer plan must remain dry_run_only"
}

# Verify draft manifest
$draftManifestPath = Join-Path $projectPath "data/generated/astronomy/manifests/astronomy-engine-v0-draft.json"
$draftManifest = Get-Content -LiteralPath $draftManifestPath -Encoding UTF8 -Raw | ConvertFrom-Json
if ($draftManifest.acceptance_status -ne "not_accepted") {
    throw "Draft manifest must remain not_accepted"
}
if ($draftManifest.artifact_hashes.status -ne "missing") {
    throw "Draft manifest artifact_hashes must remain missing"
}

# Verify capability ledger
$ledgerPath = Join-Path $projectPath "markdown/20-roadmap/93-capability-promotion-ledger.md"
$ledger = [System.IO.File]::ReadAllText($ledgerPath, [System.Text.Encoding]::UTF8)
if (-not $ledger.Contains("``astronomy-engine`` | target | M10 |")) {
    throw "astronomy-engine must remain target in capability ledger"
}

# Verify preflight materialization scope
$mat = $preflight.materialization_allowed_after_preflight
if (-not $mat.generated_astronomy_artifacts) {
    throw "Next loop must allow generated astronomy artifact materialization"
}
if (-not $mat.generated_artifact_hashes) {
    throw "Next loop must allow generated artifact hash computation"
}
if ($mat.draft_manifest_acceptance_change) {
    throw "Next loop must not allow draft manifest acceptance changes"
}
if ($mat.runtime_behavior_change) {
    throw "Next loop must not allow runtime behavior changes"
}
if ($mat.android_baseline_replacement) {
    throw "Next loop must not allow Android baseline replacement"
}
if ($mat.capability_promotion) {
    throw "Next loop must not allow capability promotion"
}

$report = @{
    status                  = "preflight_only"
    preflight_id            = $preflight.generated_artifact_materialization_preflight_id
    milestone               = $preflight.milestone
    loop                    = $preflight.loop
    output_directory_exists = (Test-Path -LiteralPath $outDir)
    generated_artifacts     = 0
    generated_hashes        = 0
    source_payloads_materialized = $materializedCount
    acceptance_unchanged    = $true
    runtime_unchanged       = $true
    android_baseline_unchanged = $true
    astronomy_engine_target = $true
    writes                  = $false
}

$report | ConvertTo-Json -Depth 4
Write-Host "Generated artifact materialization preflight dry-run passed"
