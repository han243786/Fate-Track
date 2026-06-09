[CmdletBinding()]
param(
    [string]$ProjectRoot
)

$ErrorActionPreference = "Stop"

if ([string]::IsNullOrWhiteSpace($ProjectRoot)) {
    $ProjectRoot = Join-Path $PSScriptRoot ".."
}

$resolvedProject = Resolve-Path -LiteralPath $ProjectRoot
$projectPath = $resolvedProject.ProviderPath

function Read-Text {
    param([string]$RelativePath)
    $path = Join-Path $projectPath $RelativePath
    if (-not (Test-Path -LiteralPath $path)) {
        throw "Missing M9 preflight artifact: $RelativePath"
    }
    return [System.IO.File]::ReadAllText($path, [System.Text.Encoding]::UTF8)
}

function Assert-Contains {
    param(
        [string]$Text,
        [string]$Needle,
        [string]$Message
    )
    if (-not $Text.Contains($Needle)) {
        throw $Message
    }
}

$adr = Read-Text "docs/decisions/0015-m9-astronomy-parallel-strategy.md"
$sourceAdr = Read-Text "docs/decisions/0016-m9-astronomy-source-stack.md"
$implementationAdr = Read-Text "docs/decisions/0017-m9-generated-data-implementation-path.md"
$gates = Read-Text "markdown/20-roadmap/90-decision-gates.md"
$preflight = Read-Text "markdown/20-roadmap/27-milestone-09-preflight.md"
$precloseoutAuditDoc = Read-Text "markdown/20-roadmap/36-milestone-09-pre-closeout-audit.md"
$preflightCloseoutDoc = Read-Text "markdown/20-roadmap/44-milestone-09-preflight-closeout.md"
$ledger = Read-Text "markdown/20-roadmap/93-capability-promotion-ledger.md"
$dataReadme = Read-Text "data/generated/astronomy/README.md"
$reportTemplate = Read-Text "data/generated/astronomy/comparison-report-template.md"
$replayPolicy = Read-Text "data/generated/astronomy/replay-policy-draft.md"
$sourcePolicyPath = Join-Path $projectPath "data/generated/astronomy/source-policy.json"
$generationPlanPath = Join-Path $projectPath "data/generated/astronomy/generation-plan.json"
$implementationPlanPath = Join-Path $projectPath "data/generated/astronomy/implementation-plan.json"
$generatorContractPath = Join-Path $projectPath "data/generated/astronomy/generator-contract.json"
$generatorImplementationEntryPath = Join-Path $projectPath "data/generated/astronomy/generator-implementation-entry.json"
$sourceAdapterContractPath = Join-Path $projectPath "data/generated/astronomy/source-adapter-contract.json"
$sourceSnapshotManifestSchemaPath = Join-Path $projectPath "data/generated/astronomy/source-snapshot-manifest.schema.json"
$sourceSnapshotManifestPlanPath = Join-Path $projectPath "data/generated/astronomy/source-snapshot-manifest-plan.json"
$sourceSnapshotManifestPath = Join-Path $projectPath "data/generated/astronomy/source-snapshots/source-snapshot-manifest.json"
$sourcePayloadMaterializationPolicyPath = Join-Path $projectPath "data/generated/astronomy/source-payload-materialization-policy.json"
$sourceCaptureProcedurePath = Join-Path $projectPath "data/generated/astronomy/source-capture-procedure.json"
$sourcePayloadMaterializationDecisionPath = Join-Path $projectPath "data/generated/astronomy/source-payload-materialization-decision.json"
$selectedSourcePayloadMaterializationPreflightPath = Join-Path $projectPath "data/generated/astronomy/selected-source-payload-materialization-preflight.json"
$selectedSourcePayloadMaterializationPath = Join-Path $projectPath "data/generated/astronomy/selected-source-payload-materialization.json"
$remainingSourcePayloadStrategyPath = Join-Path $projectPath "data/generated/astronomy/remaining-source-payload-strategy.json"
$postIauRemainingSourcePayloadStrategyPath = Join-Path $projectPath "data/generated/astronomy/post-iau-remaining-source-payload-strategy.json"
$selectedJplHorizonsPayloadMaterializationPreflightPath = Join-Path $projectPath "data/generated/astronomy/selected-jpl-horizons-payload-materialization-preflight.json"
$selectedJplHorizonsPayloadMaterializationPath = Join-Path $projectPath "data/generated/astronomy/selected-jpl-horizons-payload-materialization.json"
$selectedGbTPayloadMaterializationPreflightPath = Join-Path $projectPath "data/generated/astronomy/selected-gb-t-payload-materialization-preflight.json"
$selectedGbTPayloadMaterializationPath = Join-Path $projectPath "data/generated/astronomy/selected-gb-t-payload-materialization.json"
$selectedIauSofaPayloadMaterializationPreflightPath = Join-Path $projectPath "data/generated/astronomy/selected-iau-sofa-payload-materialization-preflight.json"
$selectedIauSofaPayloadMaterializationPath = Join-Path $projectPath "data/generated/astronomy/selected-iau-sofa-payload-materialization.json"
$artifactWriterPlanPath = Join-Path $projectPath "data/generated/astronomy/artifact-writer-plan.json"
$comparisonRunnerPlanPath = Join-Path $projectPath "data/generated/astronomy/comparison-runner-plan.json"
$comparisonSchemaPath = Join-Path $projectPath "data/generated/astronomy/comparison.schema.json"
$goldenPlanPath = Join-Path $projectPath "data/generated/astronomy/golden-cases-plan.json"
$goldenReadinessPlanPath = Join-Path $projectPath "data/generated/astronomy/golden-row-readiness-plan.json"
$replayReadinessPlanPath = Join-Path $projectPath "data/generated/astronomy/replay-test-readiness-plan.json"
$precloseoutAuditPath = Join-Path $projectPath "data/generated/astronomy/precloseout-audit.json"
$preflightCloseoutDecisionPath = Join-Path $projectPath "data/generated/astronomy/preflight-closeout-decision.json"
$schemaPath = Join-Path $projectPath "data/generated/astronomy/manifest.schema.json"
$draftManifestPath = Join-Path $projectPath "data/generated/astronomy/manifests/astronomy-engine-v0-draft.json"
$generatorScript = Join-Path $projectPath "tools/generate-astronomy-tables.ps1"
$sourceSnapshotManifestDryRunScript = Join-Path $projectPath "tools/source-snapshot-manifest-dry-run.ps1"
$sourcePayloadMaterializationDryRunScript = Join-Path $projectPath "tools/source-payload-materialization-dry-run.ps1"
$sourceCaptureProcedureDryRunScript = Join-Path $projectPath "tools/source-capture-procedure-dry-run.ps1"
$sourcePayloadMaterializationDecisionDryRunScript = Join-Path $projectPath "tools/source-payload-materialization-decision-dry-run.ps1"
$selectedSourcePayloadMaterializationPreflightDryRunScript = Join-Path $projectPath "tools/selected-source-payload-materialization-preflight-dry-run.ps1"
$remainingSourcePayloadStrategyDryRunScript = Join-Path $projectPath "tools/remaining-source-payload-strategy-dry-run.ps1"
$postIauRemainingSourcePayloadStrategyDryRunScript = Join-Path $projectPath "tools/post-iau-remaining-source-payload-strategy-dry-run.ps1"
$selectedJplHorizonsPayloadMaterializationPreflightDryRunScript = Join-Path $projectPath "tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1"
$selectedGbTPayloadMaterializationPreflightDryRunScript = Join-Path $projectPath "tools/selected-gb-t-payload-materialization-preflight-dry-run.ps1"
$selectedIauSofaPayloadMaterializationPreflightDryRunScript = Join-Path $projectPath "tools/selected-iau-sofa-payload-materialization-preflight-dry-run.ps1"
$artifactWriterDryRunScript = Join-Path $projectPath "tools/artifact-writer-dry-run.ps1"
$comparisonDryRunScript = Join-Path $projectPath "tools/compare-astronomy-dry-run.ps1"
$goldenDryRunScript = Join-Path $projectPath "tools/golden-cases-dry-run.ps1"
$replayDryRunScript = Join-Path $projectPath "tools/replay-policy-dry-run.ps1"

$sourcePolicy = Get-Content -LiteralPath $sourcePolicyPath -Encoding UTF8 -Raw | ConvertFrom-Json
$generationPlan = Get-Content -LiteralPath $generationPlanPath -Encoding UTF8 -Raw | ConvertFrom-Json
$implementationPlan = Get-Content -LiteralPath $implementationPlanPath -Encoding UTF8 -Raw | ConvertFrom-Json
$generatorContract = Get-Content -LiteralPath $generatorContractPath -Encoding UTF8 -Raw | ConvertFrom-Json
$generatorImplementationEntry = Get-Content -LiteralPath $generatorImplementationEntryPath -Encoding UTF8 -Raw | ConvertFrom-Json
$sourceAdapterContract = Get-Content -LiteralPath $sourceAdapterContractPath -Encoding UTF8 -Raw | ConvertFrom-Json
$sourceSnapshotManifestSchema = Get-Content -LiteralPath $sourceSnapshotManifestSchemaPath -Encoding UTF8 -Raw | ConvertFrom-Json
$sourceSnapshotManifestPlan = Get-Content -LiteralPath $sourceSnapshotManifestPlanPath -Encoding UTF8 -Raw | ConvertFrom-Json
$sourceSnapshotManifest = Get-Content -LiteralPath $sourceSnapshotManifestPath -Encoding UTF8 -Raw | ConvertFrom-Json
$sourcePayloadMaterializationPolicy = Get-Content -LiteralPath $sourcePayloadMaterializationPolicyPath -Encoding UTF8 -Raw | ConvertFrom-Json
$sourceCaptureProcedure = Get-Content -LiteralPath $sourceCaptureProcedurePath -Encoding UTF8 -Raw | ConvertFrom-Json
$sourcePayloadMaterializationDecision = Get-Content -LiteralPath $sourcePayloadMaterializationDecisionPath -Encoding UTF8 -Raw | ConvertFrom-Json
$selectedSourcePayloadMaterializationPreflight = Get-Content -LiteralPath $selectedSourcePayloadMaterializationPreflightPath -Encoding UTF8 -Raw | ConvertFrom-Json
$selectedSourcePayloadMaterialization = Get-Content -LiteralPath $selectedSourcePayloadMaterializationPath -Encoding UTF8 -Raw | ConvertFrom-Json
$remainingSourcePayloadStrategy = Get-Content -LiteralPath $remainingSourcePayloadStrategyPath -Encoding UTF8 -Raw | ConvertFrom-Json
$postIauRemainingSourcePayloadStrategy = Get-Content -LiteralPath $postIauRemainingSourcePayloadStrategyPath -Encoding UTF8 -Raw | ConvertFrom-Json
$selectedJplHorizonsPayloadMaterializationPreflight = Get-Content -LiteralPath $selectedJplHorizonsPayloadMaterializationPreflightPath -Encoding UTF8 -Raw | ConvertFrom-Json
$selectedJplHorizonsPayloadMaterialization = Get-Content -LiteralPath $selectedJplHorizonsPayloadMaterializationPath -Encoding UTF8 -Raw | ConvertFrom-Json
$selectedGbTPayloadMaterializationPreflight = Get-Content -LiteralPath $selectedGbTPayloadMaterializationPreflightPath -Encoding UTF8 -Raw | ConvertFrom-Json
$selectedGbTPayloadMaterialization = Get-Content -LiteralPath $selectedGbTPayloadMaterializationPath -Encoding UTF8 -Raw | ConvertFrom-Json
$selectedIauSofaPayloadMaterializationPreflight = Get-Content -LiteralPath $selectedIauSofaPayloadMaterializationPreflightPath -Encoding UTF8 -Raw | ConvertFrom-Json
$selectedIauSofaPayloadMaterialization = Get-Content -LiteralPath $selectedIauSofaPayloadMaterializationPath -Encoding UTF8 -Raw | ConvertFrom-Json
$artifactWriterPlan = Get-Content -LiteralPath $artifactWriterPlanPath -Encoding UTF8 -Raw | ConvertFrom-Json
$comparisonRunnerPlan = Get-Content -LiteralPath $comparisonRunnerPlanPath -Encoding UTF8 -Raw | ConvertFrom-Json
$comparisonSchema = Get-Content -LiteralPath $comparisonSchemaPath -Encoding UTF8 -Raw | ConvertFrom-Json
$goldenPlan = Get-Content -LiteralPath $goldenPlanPath -Encoding UTF8 -Raw | ConvertFrom-Json
$goldenReadinessPlan = Get-Content -LiteralPath $goldenReadinessPlanPath -Encoding UTF8 -Raw | ConvertFrom-Json
$replayReadinessPlan = Get-Content -LiteralPath $replayReadinessPlanPath -Encoding UTF8 -Raw | ConvertFrom-Json
$precloseoutAudit = Get-Content -LiteralPath $precloseoutAuditPath -Encoding UTF8 -Raw | ConvertFrom-Json
$preflightCloseoutDecision = Get-Content -LiteralPath $preflightCloseoutDecisionPath -Encoding UTF8 -Raw | ConvertFrom-Json
$schema = Get-Content -LiteralPath $schemaPath -Encoding UTF8 -Raw | ConvertFrom-Json
$draftManifest = Get-Content -LiteralPath $draftManifestPath -Encoding UTF8 -Raw | ConvertFrom-Json
$selectedMaterializedSourceId = "naif-cspice"
$selectedSourcePayloadHash = "4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2"
$nextRemainingSourceId = "iau-sofa-ansi-c"
$selectedIauSofaPayloadHash = "436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f"
$jplHorizonsSourceId = "jpl-horizons-api"
$selectedJplHorizonsPayloadHash = "acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9"
$gbtSourceId = "gb-t-33661-2017"
$selectedGbTPayloadHash = "7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31"

foreach ($field in @(
    "source_policy_id",
    "manifest_id",
    "acceptance_status",
    "artifact_type",
    "engine_id",
    "engine_version",
    "source_references",
    "generated_range",
    "generation_command",
    "artifact_hashes",
    "comparison_report",
    "difference_taxonomy",
    "evidence_requirements",
    "acceptance_blockers",
    "created_at_utc"
)) {
    if ($schema.required_fields -notcontains $field) {
        throw "Astronomy manifest schema missing required field: $field"
    }
}

foreach ($category in @(
    "android_table_difference",
    "astronomy_source_difference",
    "ruleset_difference",
    "timezone_history_difference",
    "unresolved"
)) {
    if ($schema.required_difference_taxonomy -notcontains $category) {
        throw "Astronomy manifest schema missing taxonomy: $category"
    }
    Assert-Contains $reportTemplate $category "Comparison template missing taxonomy category: $category"
}

Assert-Contains $adr "parallel track first" "ADR 0015 must choose a parallel-first strategy"
Assert-Contains $adr "replacement requires a later ADR" "ADR 0015 must forbid silent replacement"
Assert-Contains $sourceAdr "NASA/JPL Horizons API" "ADR 0016 must name Horizons API"
Assert-Contains $sourceAdr "IAU SOFA ANSI C" "ADR 0016 must name SOFA ANSI C"
Assert-Contains $sourceAdr "NAIF CSPICE" "ADR 0016 must name CSPICE"
Assert-Contains $sourceAdr "GB/T 33661-2017" "ADR 0016 must name GB/T 33661-2017"
Assert-Contains $implementationAdr "Continue inside M9 with generated-data implementation planning" "ADR 0017 must choose continued M9 generated-data planning"
Assert-Contains $implementationAdr "No generated row may be accepted" "ADR 0017 must forbid accepting rows before hash/manifest rules"
Assert-Contains $gates "closed for M9 preflight: parallel first by ADR 0015" "DG-008 is not closed for M9 preflight"
Assert-Contains $preflight "No astronomy calculation claim." "M9 preflight must forbid calculation claims"
Assert-Contains $precloseoutAuditDoc "Full M9 astronomy-engine closeout is blocked" "M9 pre-closeout audit must block full closeout"
Assert-Contains $precloseoutAuditDoc "preflight closeout review" "M9 pre-closeout audit must allow only preflight review"
Assert-Contains $preflightCloseoutDoc "closes M9 only as a preflight milestone" "M9 preflight closeout must be explicitly preflight-only"
Assert-Contains $preflightCloseoutDoc "does not accept generated astronomy artifacts" "M9 preflight closeout must forbid generated-data acceptance"
Assert-Contains $preflightCloseoutDoc "M10 Generated Astronomy Implementation" "M9 preflight closeout must define the next generated-data milestone"
Assert-Contains $ledger "| ``astronomy-engine`` | target | M10 |" "astronomy-engine must remain target in capability ledger"
Assert-Contains $dataReadme "no generated astronomy table has been accepted yet" "Astronomy generated-data README must avoid supported claims"
Assert-Contains $dataReadme "not-accepted planning manifest instance" "Astronomy README must label the draft manifest as not accepted"
Assert-Contains $dataReadme "draft_not_runnable" "Astronomy README must label the generation plan as not runnable"
Assert-Contains $dataReadme "planning artifacts only" "Astronomy README must label comparison/golden/replay files as planning only"
Assert-Contains $dataReadme "precloseout-audit.json" "Astronomy README must mention the M9 pre-closeout audit"
Assert-Contains $dataReadme "implementation-plan.json" "Astronomy README must mention the M9 implementation plan"
Assert-Contains $dataReadme "generator-contract.json" "Astronomy README must mention the M9 generator contract"
Assert-Contains $dataReadme "generator-implementation-entry.json" "Astronomy README must mention the M10 generator implementation entry"
Assert-Contains $dataReadme "source-adapter-contract.json" "Astronomy README must mention the M9 source adapter contract"
Assert-Contains $dataReadme "source-snapshot-manifest.schema.json" "Astronomy README must mention the M10 source snapshot manifest schema"
Assert-Contains $dataReadme "source-snapshot-manifest-plan.json" "Astronomy README must mention the M10 source snapshot manifest plan"
Assert-Contains $dataReadme "source-payload-materialization-policy.json" "Astronomy README must mention the M10 source payload materialization policy"
Assert-Contains $dataReadme "source-capture-procedure.json" "Astronomy README must mention the M10 source capture procedure"
Assert-Contains $dataReadme "source-payload-materialization-decision.json" "Astronomy README must mention the M10 source payload materialization decision"
Assert-Contains $dataReadme "selected-source-payload-materialization-preflight.json" "Astronomy README must mention the M10 selected source payload materialization preflight"
Assert-Contains $dataReadme "selected-source-payload-materialization.json" "Astronomy README must mention the M10 selected source payload materialization evidence"
Assert-Contains $dataReadme "remaining-source-payload-strategy.json" "Astronomy README must mention the M10 remaining source payload strategy"
Assert-Contains $dataReadme "post-iau-remaining-source-payload-strategy.json" "Astronomy README must mention the M10 post-IAU remaining source payload strategy"
Assert-Contains $dataReadme "selected-jpl-horizons-payload-materialization-preflight.json" "Astronomy README must mention the M10 selected JPL Horizons payload materialization preflight"
Assert-Contains $dataReadme "selected-jpl-horizons-payload-materialization.json" "Astronomy README must mention the M10 selected JPL Horizons payload materialization evidence"
Assert-Contains $dataReadme "jpl-horizons-validation-samples.json" "Astronomy README must mention the M10 selected JPL Horizons payload file"
Assert-Contains $dataReadme "selected-gb-t-payload-materialization-preflight.json" "Astronomy README must mention the M10 selected GB/T payload materialization preflight"
Assert-Contains $dataReadme "selected-iau-sofa-payload-materialization-preflight.json" "Astronomy README must mention the M10 selected IAU SOFA payload materialization preflight"
Assert-Contains $dataReadme "selected-iau-sofa-payload-materialization.json" "Astronomy README must mention the M10 selected IAU SOFA payload materialization evidence"
Assert-Contains $dataReadme "iau-sofa-routine-version.json" "Astronomy README must mention the M10 selected IAU SOFA payload file"
Assert-Contains $dataReadme "artifact-writer-plan.json" "Astronomy README must mention the M9 artifact writer plan"
Assert-Contains $dataReadme "comparison-runner-plan.json" "Astronomy README must mention the M9 comparison runner plan"
Assert-Contains $dataReadme "golden-row-readiness-plan.json" "Astronomy README must mention the M9 golden row readiness plan"
Assert-Contains $dataReadme "replay-test-readiness-plan.json" "Astronomy README must mention the M9 replay test readiness plan"
Assert-Contains $dataReadme "preflight-closeout-decision.json" "Astronomy README must mention the M9 preflight closeout decision"
Assert-Contains $replayPolicy "Silent replacement" "Replay policy must forbid silent replacement"
Assert-Contains (Read-Text "tools/generate-astronomy-tables.ps1") "DRY_RUN_ONLY" "Generator skeleton must be marked dry-run only"
Assert-Contains (Read-Text "tools/generate-astronomy-tables.ps1") "GUARDED_IMPLEMENTATION_ENTRY" "Generator must expose the guarded M10 implementation entry marker"
Assert-Contains (Read-Text "tools/source-snapshot-manifest-dry-run.ps1") "DRY_RUN_ONLY" "Source snapshot manifest tool must be marked dry-run only"
Assert-Contains (Read-Text "tools/source-payload-materialization-dry-run.ps1") "DRY_RUN_ONLY" "Source payload materialization tool must be marked dry-run only"
Assert-Contains (Read-Text "tools/source-capture-procedure-dry-run.ps1") "DRY_RUN_ONLY" "Source capture procedure tool must be marked dry-run only"
Assert-Contains (Read-Text "tools/source-payload-materialization-decision-dry-run.ps1") "DRY_RUN_ONLY" "Source payload materialization decision tool must be marked dry-run only"
Assert-Contains (Read-Text "tools/selected-source-payload-materialization-preflight-dry-run.ps1") "DRY_RUN_ONLY" "Selected source payload materialization preflight tool must be marked dry-run only"
Assert-Contains (Read-Text "tools/remaining-source-payload-strategy-dry-run.ps1") "DRY_RUN_ONLY" "Remaining source payload strategy tool must be marked dry-run only"
Assert-Contains (Read-Text "tools/post-iau-remaining-source-payload-strategy-dry-run.ps1") "DRY_RUN_ONLY" "Post-IAU remaining source payload strategy tool must be marked dry-run only"
Assert-Contains (Read-Text "tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1") "DRY_RUN_ONLY" "Selected JPL Horizons payload materialization preflight tool must be marked dry-run only"
Assert-Contains (Read-Text "tools/selected-gb-t-payload-materialization-preflight-dry-run.ps1") "DRY_RUN_ONLY" "Selected GB/T payload materialization preflight tool must be marked dry-run only"
Assert-Contains (Read-Text "tools/selected-iau-sofa-payload-materialization-preflight-dry-run.ps1") "DRY_RUN_ONLY" "Selected IAU SOFA payload materialization preflight tool must be marked dry-run only"
Assert-Contains (Read-Text "tools/artifact-writer-dry-run.ps1") "DRY_RUN_ONLY" "Artifact writer scaffold must be marked dry-run only"
Assert-Contains (Read-Text "tools/compare-astronomy-dry-run.ps1") "DRY_RUN_ONLY" "Comparison scaffold must be marked dry-run only"
Assert-Contains (Read-Text "tools/golden-cases-dry-run.ps1") "DRY_RUN_ONLY" "Golden-case scaffold must be marked dry-run only"
Assert-Contains (Read-Text "tools/replay-policy-dry-run.ps1") "DRY_RUN_ONLY" "Replay-policy scaffold must be marked dry-run only"

if ($sourcePolicy.source_policy_id -ne "m9-astronomy-source-stack-v1") {
    throw "Unexpected astronomy source policy id"
}

foreach ($sourceId in @(
    "gb-t-33661-2017",
    "jpl-horizons-api",
    "iau-sofa-ansi-c",
    "naif-cspice"
)) {
    $json = $sourcePolicy | ConvertTo-Json -Depth 8
    Assert-Contains $json $sourceId "Source policy missing source id: $sourceId"
}

if ($sourcePolicy.first_generated_range.start_year -ne 1901 -or $sourcePolicy.first_generated_range.end_year -ne 2100) {
    throw "First generated range must stay aligned to V1 1901-2100 validation range"
}

if ($draftManifest.source_policy_id -ne $sourcePolicy.source_policy_id) {
    throw "Draft manifest must reference the active astronomy source policy"
}

if ($draftManifest.acceptance_status -ne "not_accepted") {
    throw "Draft manifest must remain not_accepted until generated evidence exists"
}

if ($draftManifest.generation_command.status -ne "not_run") {
    throw "Draft manifest must not claim a generation command has run"
}

if ($draftManifest.artifact_hashes.status -ne "missing" -or $draftManifest.artifact_hashes.items.Count -ne 0) {
    throw "Draft manifest must not claim artifact hashes"
}

if ($draftManifest.comparison_report.status -ne "template_only") {
    throw "Draft manifest must not claim a completed comparison report"
}

foreach ($blocker in @(
    "generation command not selected",
    "no generated artifact exists",
    "no artifact hashes exist",
    "comparison report is template only",
    "golden cases are not generated",
    "runtime engine is not integrated"
)) {
    if ($draftManifest.acceptance_blockers -notcontains $blocker) {
        throw "Draft manifest missing acceptance blocker: $blocker"
    }
}

if ($generationPlan.status -ne "draft_not_runnable") {
    throw "Generation plan must remain draft_not_runnable"
}

if ($implementationPlan.status -ne "planning_only") {
    throw "M9 implementation plan must remain planning_only"
}

if ($implementationPlan.decision -ne "continue_m9_generated_data_planning") {
    throw "M9 implementation plan must continue generated-data planning inside M9"
}

if ($implementationPlan.capability_status -ne "target") {
    throw "M9 implementation plan must keep astronomy-engine target"
}

$generatorStage = @($implementationPlan.stages | Where-Object { $_.id -eq "generator-contract" })
if ($generatorStage.Count -ne 1 -or $generatorStage[0].status -ne "contract_defined") {
    throw "M9 implementation plan must mark generator-contract as contract_defined"
}

$sourceAdapterStage = @($implementationPlan.stages | Where-Object { $_.id -eq "source-adapter-contract" })
if ($sourceAdapterStage.Count -ne 1 -or $sourceAdapterStage[0].status -ne "contract_defined") {
    throw "M9 implementation plan must mark source-adapter-contract as contract_defined"
}

$artifactWriterStage = @($implementationPlan.stages | Where-Object { $_.id -eq "artifact-writer-dry-run" })
if ($artifactWriterStage.Count -ne 1 -or $artifactWriterStage[0].status -ne "dry_run_defined") {
    throw "M9 implementation plan must mark artifact-writer-dry-run as dry_run_defined"
}

$comparisonRunnerStage = @($implementationPlan.stages | Where-Object { $_.id -eq "comparison-runner-dry-run" })
if ($comparisonRunnerStage.Count -ne 1 -or $comparisonRunnerStage[0].status -ne "dry_run_defined") {
    throw "M9 implementation plan must mark comparison-runner-dry-run as dry_run_defined"
}

$goldenMaterializationStage = @($implementationPlan.stages | Where-Object { $_.id -eq "golden-row-materialization" })
if ($goldenMaterializationStage.Count -ne 1 -or $goldenMaterializationStage[0].status -ne "readiness_defined") {
    throw "M9 implementation plan must mark golden-row-materialization as readiness_defined"
}

$replayMaterializationStage = @($implementationPlan.stages | Where-Object { $_.id -eq "replay-test-materialization" })
if ($replayMaterializationStage.Count -ne 1 -or $replayMaterializationStage[0].status -ne "readiness_defined") {
    throw "M9 implementation plan must mark replay-test-materialization as readiness_defined"
}

foreach ($stageId in @(
    "generator-contract",
    "source-adapter-contract",
    "artifact-writer-dry-run",
    "comparison-runner-dry-run",
    "golden-row-materialization",
    "replay-test-materialization"
)) {
    $match = @($implementationPlan.stages | Where-Object { $_.id -eq $stageId })
    if ($match.Count -ne 1) {
        throw "M9 implementation plan missing stage: $stageId"
    }
}

foreach ($forbidden in @(
    "generated artifact acceptance",
    "android baseline replacement",
    "calendar-date-query runtime change",
    "chart-create runtime change",
    "astronomy-engine supported"
)) {
    if ($implementationPlan.forbidden_until_later_acceptance -notcontains $forbidden) {
        throw "M9 implementation plan missing forbidden item: $forbidden"
    }
}

if ($generatorContract.status -ne "contract_only") {
    throw "M9 generator contract must remain contract_only"
}

if ($generatorContract.hash_algorithm -ne "sha256") {
    throw "M9 generator contract must require sha256"
}

if ($generatorContract.source_policy_id -ne $sourcePolicy.source_policy_id) {
    throw "M9 generator contract must reference the active source policy"
}

if ($generatorContract.manifest_id -ne $draftManifest.manifest_id) {
    throw "M9 generator contract must reference the active draft manifest"
}

if (@($generatorContract.planned_outputs).Count -ne @($generationPlan.planned_artifacts).Count) {
    throw "M9 generator contract output count must match generation plan"
}

foreach ($inputId in @(
    "source-policy",
    "draft-manifest",
    "generation-plan",
    "implementation-plan"
)) {
    $match = @($generatorContract.required_inputs | Where-Object { $_.id -eq $inputId })
    if ($match.Count -ne 1 -or $match[0].required -ne $true) {
        throw "M9 generator contract missing required input: $inputId"
    }
}

foreach ($output in $generatorContract.planned_outputs) {
    if ($output.status -ne "not_generated") {
        throw "M9 generator contract output must remain not_generated: $($output.path)"
    }
    if ($output.hash_required -ne $true) {
        throw "M9 generator contract output must require hash: $($output.path)"
    }
}

foreach ($forbidden in @(
    "write planned output files",
    "mark artifact hash present",
    "mark manifest accepted",
    "change runtime date-layer behavior",
    "replace android-date-layer-v1",
    "claim astronomy-engine supported"
)) {
    if ($generatorContract.forbidden_in_contract_stage -notcontains $forbidden) {
        throw "M9 generator contract missing forbidden item: $forbidden"
    }
}

if ($generatorImplementationEntry.status -ne "guarded_entrypoint_only") {
    throw "M10 generator implementation entry must remain guarded_entrypoint_only"
}

if ($generatorImplementationEntry.capability_status -ne "target") {
    throw "M10 generator implementation entry must keep astronomy-engine target"
}

if ($generatorImplementationEntry.generator_contract_id -ne $generatorContract.generator_contract_id) {
    throw "M10 generator implementation entry must reference the active generator contract"
}

if ($generatorImplementationEntry.manifest_id -ne $draftManifest.manifest_id) {
    throw "M10 generator implementation entry must reference the active draft manifest"
}

if ($generatorImplementationEntry.non_dry_run_entrypoint_available -ne $true) {
    throw "M10 generator implementation entry must expose a guarded non-dry-run entrypoint"
}

foreach ($flag in @(
    "AllowSourceSnapshotUse",
    "AllowArtifactWrite",
    "AllowManifestUpdate"
)) {
    if ($generatorImplementationEntry.guard_flags -notcontains $flag) {
        throw "M10 generator implementation entry missing guard flag: $flag"
    }
}

if ($generatorImplementationEntry.source_snapshot_manifest.status -ne "metadata_only_no_source_payload") {
    throw "M10 generator implementation entry must keep source snapshot manifest as metadata_only_no_source_payload"
}

foreach ($blocker in @(
    "source snapshot payloads are not materialized",
    "local generation adapter is not implemented",
    "planned artifacts do not exist",
    "sha256 hashes do not exist",
    "comparison report is not completed",
    "golden rows are not generated",
    "replay tests are not executed",
    "replacement ADR is missing"
)) {
    if ($generatorImplementationEntry.block_reasons -notcontains $blocker) {
        throw "M10 generator implementation entry missing blocker: $blocker"
    }
}

if ($generatorImplementationEntry.entrypoint_result_policy.writes_performed -ne $false -or
    $generatorImplementationEntry.entrypoint_result_policy.hashes_computed -ne 0 -or
    $generatorImplementationEntry.entrypoint_result_policy.acceptance_status_changed -ne $false -or
    $generatorImplementationEntry.entrypoint_result_policy.runtime_behavior_changed -ne $false -or
    $generatorImplementationEntry.entrypoint_result_policy.generated_artifacts_accepted -ne $false) {
    throw "M10 generator implementation entry result policy must forbid writes, hashes, acceptance, runtime changes, and accepted artifacts"
}

foreach ($forbidden in @(
    "write generated artifact files",
    "compute hashes for generated artifacts",
    "mark manifest accepted",
    "change calendar-date-query runtime behavior",
    "change chart-create runtime behavior",
    "replace android-date-layer-v1",
    "claim astronomy-engine supported"
)) {
    if ($generatorImplementationEntry.forbidden_until_later_evidence -notcontains $forbidden) {
        throw "M10 generator implementation entry missing forbidden item: $forbidden"
    }
}

if ($sourceAdapterContract.status -ne "contract_only") {
    throw "M9 source adapter contract must remain contract_only"
}

if ($sourceAdapterContract.source_policy_id -ne $sourcePolicy.source_policy_id) {
    throw "M9 source adapter contract must reference the active source policy"
}

if ($sourceAdapterContract.generator_contract_id -ne $generatorContract.generator_contract_id) {
    throw "M9 source adapter contract must reference the active generator contract"
}

foreach ($sourceId in @(
    "gb-t-33661-2017",
    "jpl-horizons-api",
    "iau-sofa-ansi-c",
    "naif-cspice"
)) {
    $match = @($sourceAdapterContract.adapters | Where-Object { $_.source_id -eq $sourceId })
    if ($match.Count -ne 1) {
        throw "M9 source adapter contract missing adapter: $sourceId"
    }
    if ($match[0].runtime_dependency -ne $false -or $match[0].output_claim_allowed -ne $false) {
        throw "M9 source adapter contract must forbid runtime dependency and output claim: $sourceId"
    }
}

foreach ($required in @(
    "adapter input snapshots or local routine versions recorded",
    "adapter provenance recorded in manifest source_references",
    "validation sample timestamps recorded",
    "offline dependency versions recorded when used",
    "no runtime network dependency"
)) {
    if ($sourceAdapterContract.required_before_generated_acceptance -notcontains $required) {
        throw "M9 source adapter contract missing acceptance requirement: $required"
    }
}

foreach ($forbidden in @(
    "call external API during full project gate",
    "treat source availability as generated data",
    "make Horizons a runtime dependency",
    "claim SOFA or SPICE integration",
    "replace android-date-layer-v1",
    "claim astronomy-engine supported"
)) {
    if ($sourceAdapterContract.forbidden_in_contract_stage -notcontains $forbidden) {
        throw "M9 source adapter contract missing forbidden item: $forbidden"
    }
}

foreach ($field in @(
    "source_snapshot_manifest_id",
    "status",
    "source_policy_id",
    "generator_contract_id",
    "source_adapter_contract_id",
    "snapshot_range",
    "sources",
    "acceptance_requirements",
    "forbidden_until_acceptance",
    "created_at_utc"
)) {
    if ($sourceSnapshotManifestSchema.required_fields -notcontains $field) {
        throw "M10 source snapshot manifest schema missing required field: $field"
    }
}

foreach ($field in @(
    "source_id",
    "adapter_role",
    "snapshot_mode",
    "provenance",
    "local_materialization_status",
    "runtime_dependency",
    "output_claim_allowed"
)) {
    if ($sourceSnapshotManifestSchema.source_required_fields -notcontains $field) {
        throw "M10 source snapshot manifest schema missing source field: $field"
    }
}

if ($sourceSnapshotManifestSchema.status -ne "schema_only") {
    throw "M10 source snapshot manifest schema must remain schema_only"
}

if ($sourceSnapshotManifestPlan.status -ne "manifest_materialized_metadata_only") {
    throw "M10 source snapshot manifest plan must remain manifest_materialized_metadata_only"
}

if ($sourceSnapshotManifestPlan.schema_id -ne $sourceSnapshotManifestSchema.schema_id) {
    throw "M10 source snapshot manifest plan must reference the active schema"
}

if ($sourceSnapshotManifestPlan.manifest_status -ne "metadata_only_no_source_payload") {
    throw "M10 source snapshot manifest plan must keep the manifest metadata_only_no_source_payload"
}

if ($sourceSnapshotManifestPlan.source_policy_id -ne $sourcePolicy.source_policy_id -or
    $sourceSnapshotManifestPlan.generator_contract_id -ne $generatorContract.generator_contract_id -or
    $sourceSnapshotManifestPlan.source_adapter_contract_id -ne $sourceAdapterContract.source_adapter_contract_id) {
    throw "M10 source snapshot manifest plan must reference active source, generator, and adapter contracts"
}

if ($sourceSnapshotManifestPlan.snapshot_range.start_year -ne $sourcePolicy.first_generated_range.start_year -or
    $sourceSnapshotManifestPlan.snapshot_range.end_year -ne $sourcePolicy.first_generated_range.end_year) {
    throw "M10 source snapshot manifest plan range must match source policy first generated range"
}

$plannedSourceIds = @($sourceSnapshotManifestPlan.planned_sources | ForEach-Object { $_.source_id })
$manifestSourceIds = @($sourceSnapshotManifest.sources | ForEach-Object { $_.source_id })
foreach ($sourceId in $sourceSnapshotManifestSchema.allowed_source_ids) {
    if ($plannedSourceIds -notcontains $sourceId) {
        throw "M10 source snapshot manifest plan missing source id: $sourceId"
    }
    if ($manifestSourceIds -notcontains $sourceId) {
        throw "M10 source snapshot manifest missing source id: $sourceId"
    }
    $adapterMatch = @($sourceAdapterContract.adapters | Where-Object { $_.source_id -eq $sourceId })
    if ($adapterMatch.Count -ne 1) {
        throw "M10 source snapshot manifest plan source is not in source adapter contract: $sourceId"
    }
}

if ($sourceSnapshotManifest.status -ne "selected_source_payload_materialized") {
    throw "M10 source snapshot manifest must record selected_source_payload_materialized"
}

if ($sourceSnapshotManifest.source_policy_id -ne $sourcePolicy.source_policy_id -or
    $sourceSnapshotManifest.generator_contract_id -ne $generatorContract.generator_contract_id -or
    $sourceSnapshotManifest.source_adapter_contract_id -ne $sourceAdapterContract.source_adapter_contract_id) {
    throw "M10 source snapshot manifest must reference active source, generator, and adapter contracts"
}

if ($sourceSnapshotManifest.snapshot_range.start_year -ne $sourcePolicy.first_generated_range.start_year -or
    $sourceSnapshotManifest.snapshot_range.end_year -ne $sourcePolicy.first_generated_range.end_year) {
    throw "M10 source snapshot manifest range must match source policy first generated range"
}

foreach ($source in $sourceSnapshotManifest.sources) {
    if ($source.source_id -eq $selectedMaterializedSourceId) {
        if ($source.local_materialization_status -ne "source_boundary_payload_materialized" -or
            $source.source_payload_path -ne "data/generated/astronomy/source-snapshots/payloads/naif-cspice-kernel-boundary.json" -or
            $source.source_payload_hash.algorithm -ne "sha256" -or
            $source.source_payload_hash.value -ne $selectedSourcePayloadHash) {
            throw "M10 selected source manifest entry must record selected payload path and hash"
        }
    } elseif ($source.source_id -eq $nextRemainingSourceId) {
        if ($source.local_materialization_status -ne "routine_version_payload_materialized" -or
            $source.source_payload_path -ne "data/generated/astronomy/source-snapshots/payloads/iau-sofa-routine-version.json" -or
            $source.source_payload_hash.algorithm -ne "sha256" -or
            $source.source_payload_hash.value -ne $selectedIauSofaPayloadHash) {
            throw "M10 selected IAU SOFA manifest entry must record selected payload path and hash"
        }
    } elseif ($source.source_id -eq $jplHorizonsSourceId) {
        if ($source.local_materialization_status -ne "validation_query_snapshot_payload_materialized" -or
            $source.source_payload_path -ne "data/generated/astronomy/source-snapshots/payloads/jpl-horizons-validation-samples.json" -or
            $source.source_payload_hash.algorithm -ne "sha256" -or
            $source.source_payload_hash.value -ne $selectedJplHorizonsPayloadHash) {
            throw "M10 selected JPL Horizons manifest entry must record selected payload path and hash"
        }
    } elseif ($source.source_id -eq $gbtSourceId) {
        if ($source.local_materialization_status -ne "rule_reference_payload_materialized" -or
            $source.source_payload_path -ne "data/generated/astronomy/source-snapshots/payloads/gb-t-33661-2017-rule-reference.json" -or
            $source.source_payload_hash.algorithm -ne "sha256" -or
            $source.source_payload_hash.value -ne $selectedGbTPayloadHash) {
            throw "M10 selected GB/T manifest entry must record selected rule-reference payload path and hash"
        }
    } elseif ($source.local_materialization_status -ne "not_materialized") {
        throw "M10 unselected source snapshot manifest sources must remain not_materialized: $($source.source_id)"
    }
    if ($source.runtime_dependency -ne $false -or $source.output_claim_allowed -ne $false) {
        throw "M10 source snapshot manifest sources must forbid runtime dependency and output claim: $($source.source_id)"
    }
}

foreach ($plannedSource in $sourceSnapshotManifestPlan.planned_sources) {
    if ($plannedSource.local_materialization_status -ne "not_materialized") {
        throw "M10 source snapshot planned source must remain not_materialized: $($plannedSource.source_id)"
    }
    if ($plannedSource.runtime_dependency -ne $false -or $plannedSource.output_claim_allowed -ne $false) {
        throw "M10 source snapshot planned source must forbid runtime dependency and output claim: $($plannedSource.source_id)"
    }
}

if ($sourcePayloadMaterializationPolicy.status -ne "selected_source_payload_materialized") {
    throw "M10 source payload materialization policy must record selected_source_payload_materialized"
}

if ($sourcePayloadMaterializationPolicy.source_snapshot_manifest_id -ne $sourceSnapshotManifest.source_snapshot_manifest_id -or
    $sourcePayloadMaterializationPolicy.source_policy_id -ne $sourcePolicy.source_policy_id -or
    $sourcePayloadMaterializationPolicy.source_adapter_contract_id -ne $sourceAdapterContract.source_adapter_contract_id) {
    throw "M10 source payload materialization policy must reference active source manifest, source policy, and adapter contract"
}

if ($sourcePayloadMaterializationPolicy.payload_directory.status -ne "exists_selected_source_only" -or
    $sourcePayloadMaterializationPolicy.payload_directory.create_allowed -ne $true -or
    $sourcePayloadMaterializationPolicy.payload_directory.allowed_materialized_sources -notcontains $selectedMaterializedSourceId -or
    $sourcePayloadMaterializationPolicy.payload_directory.allowed_materialized_sources -notcontains $nextRemainingSourceId -or
    $sourcePayloadMaterializationPolicy.payload_directory.allowed_materialized_sources -notcontains $jplHorizonsSourceId -or
    $sourcePayloadMaterializationPolicy.payload_directory.allowed_materialized_sources -notcontains $gbtSourceId) {
    throw "M10 source payload materialization policy must allow selected NAIF, IAU SOFA, JPL Horizons, and GB/T source payload materialization"
}

$sourcePayloadDirectoryPath = Join-Path $projectPath $sourcePayloadMaterializationPolicy.payload_directory.path
if (-not (Test-Path -LiteralPath $sourcePayloadDirectoryPath)) {
    throw "M10 source payload directory must exist after selected source materialization"
}

$existingSourcePayloadFiles = @()

foreach ($source in $sourceSnapshotManifest.sources) {
    $payloadMatch = @($sourcePayloadMaterializationPolicy.planned_payloads | Where-Object { $_.source_id -eq $source.source_id })
    if ($payloadMatch.Count -ne 1) {
        throw "M10 source payload materialization policy must include one payload for source: $($source.source_id)"
    }
    $payload = $payloadMatch[0]
    if ($payload.schema_status -ne "schema_only") {
        throw "M10 source payload schema status must remain schema_only: $($payload.source_id)"
    }
    $sourcePayloadSchemaPath = Join-Path $projectPath $payload.schema_path
    if (-not (Test-Path -LiteralPath $sourcePayloadSchemaPath)) {
        throw "M10 source payload schema missing: $($payload.schema_path)"
    }
    $sourcePayloadSchema = Get-Content -LiteralPath $sourcePayloadSchemaPath -Encoding UTF8 -Raw | ConvertFrom-Json
    if ($sourcePayloadSchema.status -ne "schema_only") {
        throw "M10 source payload schema must remain schema_only: $($payload.schema_path)"
    }
    if ($sourcePayloadSchema.source_id -ne $payload.source_id -or $sourcePayloadSchema.payload_kind -ne $payload.payload_kind) {
        throw "M10 source payload schema must match payload source and kind: $($payload.source_id)"
    }
    foreach ($field in @(
        "payload_id",
        "source_id",
        "payload_kind",
        "provenance",
        "hash_algorithm",
        "created_at_utc"
    )) {
        if ($sourcePayloadSchema.required_fields -notcontains $field) {
            throw "M10 source payload schema missing required common field: $field"
        }
    }
    if ($sourcePayloadSchema.forbidden_claims -notcontains "generated astronomy artifact" -or
        $sourcePayloadSchema.forbidden_claims -notcontains "astronomy-engine supported") {
        throw "M10 source payload schema must forbid generated artifact and supported claims: $($payload.source_id)"
    }
    if ($payload.runtime_dependency -ne $false -or $payload.output_claim_allowed -ne $false) {
        throw "M10 source payload must forbid runtime dependency and output claim: $($payload.source_id)"
    }
    $payloadPath = Join-Path $projectPath $payload.path
    if ($payload.source_id -eq $selectedMaterializedSourceId) {
        if ($payload.payload_status -ne "materialized" -or
            $payload.hash_status -ne "computed" -or
            $payload.sha256 -ne $selectedSourcePayloadHash) {
            throw "M10 selected source payload policy must record materialized/computed/hash"
        }
        if (-not (Test-Path -LiteralPath $payloadPath)) {
            throw "M10 selected source payload file must exist: $($payload.path)"
        }
        $actualSourcePayloadHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $payloadPath).Hash.ToLowerInvariant()
        if ($actualSourcePayloadHash -ne $selectedSourcePayloadHash) {
            throw "M10 selected source payload file hash mismatch: $actualSourcePayloadHash"
        }
        $existingSourcePayloadFiles += $payload.path
    } elseif ($payload.source_id -eq $nextRemainingSourceId) {
        if ($payload.payload_status -ne "materialized" -or
            $payload.hash_status -ne "computed" -or
            $payload.sha256 -ne $selectedIauSofaPayloadHash) {
            throw "M10 selected IAU SOFA payload policy must record materialized/computed/hash"
        }
        if (-not (Test-Path -LiteralPath $payloadPath)) {
            throw "M10 selected IAU SOFA payload file must exist: $($payload.path)"
        }
        $actualIauSofaPayloadHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $payloadPath).Hash.ToLowerInvariant()
        if ($actualIauSofaPayloadHash -ne $selectedIauSofaPayloadHash) {
            throw "M10 selected IAU SOFA payload file hash mismatch: $actualIauSofaPayloadHash"
        }
        $existingSourcePayloadFiles += $payload.path
    } elseif ($payload.source_id -eq $jplHorizonsSourceId) {
        if ($payload.payload_status -ne "materialized" -or
            $payload.hash_status -ne "computed" -or
            $payload.sha256 -ne $selectedJplHorizonsPayloadHash) {
            throw "M10 selected JPL Horizons payload policy must record materialized/computed/hash"
        }
        if (-not (Test-Path -LiteralPath $payloadPath)) {
            throw "M10 selected JPL Horizons payload file must exist: $($payload.path)"
        }
        $actualJplHorizonsPayloadHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $payloadPath).Hash.ToLowerInvariant()
        if ($actualJplHorizonsPayloadHash -ne $selectedJplHorizonsPayloadHash) {
            throw "M10 selected JPL Horizons payload file hash mismatch: $actualJplHorizonsPayloadHash"
        }
        $existingSourcePayloadFiles += $payload.path
    } elseif ($payload.source_id -eq $gbtSourceId) {
        if ($payload.payload_status -ne "materialized" -or
            $payload.hash_status -ne "computed" -or
            $payload.sha256 -ne $selectedGbTPayloadHash) {
            throw "M10 selected GB/T payload policy must record materialized/computed/hash"
        }
        if (-not (Test-Path -LiteralPath $payloadPath)) {
            throw "M10 selected GB/T payload file must exist: $($payload.path)"
        }
        $actualGbTPayloadHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $payloadPath).Hash.ToLowerInvariant()
        if ($actualGbTPayloadHash -ne $selectedGbTPayloadHash) {
            throw "M10 selected GB/T payload file hash mismatch: $actualGbTPayloadHash"
        }
        $existingSourcePayloadFiles += $payload.path
    } else {
        if ($payload.payload_status -ne "not_materialized" -or $payload.hash_status -ne "not_computed") {
            throw "M10 unselected source payload must remain not_materialized/not_computed: $($payload.source_id)"
        }
        if (Test-Path -LiteralPath $payloadPath) {
            throw "M10 unselected source payload file must not exist: $($payload.path)"
        }
    }
}

if ($existingSourcePayloadFiles.Count -ne 4) {
    throw "M10 must have exactly four selected source payload files"
}

foreach ($required in @(
    "selected-source-only payload directory policy approved",
    "per-source payload schema files defined",
    "manual or external source capture procedure documented",
    "no external API call in full project gate policy preserved",
    "selected source payload hash recorded"
)) {
    if ($sourcePayloadMaterializationPolicy.required_before_payload_materialization -notcontains $required) {
        throw "M10 source payload materialization policy missing requirement: $required"
    }
}

foreach ($forbidden in @(
    "write unselected source payload files",
    "compute unselected source payload hashes",
    "write generated astronomy artifacts",
    "compute generated artifact hashes",
    "mark draft manifest accepted",
    "change calendar-date-query runtime behavior",
    "change chart-create runtime behavior",
    "claim astronomy-engine supported"
)) {
    if ($sourcePayloadMaterializationPolicy.forbidden_in_policy_stage -notcontains $forbidden) {
        throw "M10 source payload materialization policy missing forbidden item: $forbidden"
    }
}

if ($sourceCaptureProcedure.status -ne "selected_source_payload_materialized") {
    throw "M10 source capture procedure must record selected_source_payload_materialized"
}

if ($sourceCaptureProcedure.source_snapshot_manifest_id -ne $sourceSnapshotManifest.source_snapshot_manifest_id -or
    $sourceCaptureProcedure.source_payload_materialization_policy_id -ne $sourcePayloadMaterializationPolicy.source_payload_materialization_policy_id) {
    throw "M10 source capture procedure must reference active source snapshot manifest and payload policy"
}

if ($sourceCaptureProcedure.full_gate_network_policy -ne "no_external_calls") {
    throw "M10 source capture procedure must preserve no external calls in full gate"
}

if ($sourceCaptureProcedure.payload_directory.status -ne "exists_selected_source_only" -or
    $sourceCaptureProcedure.payload_directory.create_allowed -ne $true -or
    $sourceCaptureProcedure.payload_directory.path -ne $sourcePayloadMaterializationPolicy.payload_directory.path -or
    $sourceCaptureProcedure.payload_directory.allowed_materialized_sources -notcontains $selectedMaterializedSourceId -or
    $sourceCaptureProcedure.payload_directory.allowed_materialized_sources -notcontains $nextRemainingSourceId -or
    $sourceCaptureProcedure.payload_directory.allowed_materialized_sources -notcontains $jplHorizonsSourceId -or
    $sourceCaptureProcedure.payload_directory.allowed_materialized_sources -notcontains $gbtSourceId) {
    throw "M10 source capture procedure must keep selected-source-only payload directory policy for NAIF, IAU SOFA, JPL Horizons, and GB/T"
}

if ($sourceCaptureProcedure.payload_materialization_allowed -ne $true -or
    $sourceCaptureProcedure.payload_materialization_scope -ne "selected_source_only" -or
    $sourceCaptureProcedure.payload_hash_computation_allowed -ne $true -or
    $sourceCaptureProcedure.payload_hash_computation_scope -ne "selected_source_payload_only" -or
    $sourceCaptureProcedure.generated_artifact_allowed -ne $false -or
    $sourceCaptureProcedure.generated_artifact_hash_allowed -ne $false -or
    $sourceCaptureProcedure.manifest_acceptance_change_allowed -ne $false -or
    $sourceCaptureProcedure.runtime_behavior_change_allowed -ne $false -or
    $sourceCaptureProcedure.capability_status -ne "target") {
    throw "M10 source capture procedure must allow only selected source payload/hash and forbid artifacts, acceptance changes, runtime changes, or capability promotion"
}

foreach ($payload in $sourcePayloadMaterializationPolicy.planned_payloads) {
    $procedureMatch = @($sourceCaptureProcedure.procedures | Where-Object { $_.source_id -eq $payload.source_id })
    if ($procedureMatch.Count -ne 1) {
        throw "M10 source capture procedure must include one procedure for source: $($payload.source_id)"
    }
    $procedure = $procedureMatch[0]
    if ($procedure.payload_kind -ne $payload.payload_kind -or
        $procedure.schema_path -ne $payload.schema_path -or
        $procedure.payload_path -ne $payload.path) {
        throw "M10 source capture procedure must match payload policy source/kind/schema/path: $($payload.source_id)"
    }
    if ($payload.source_id -eq $selectedMaterializedSourceId) {
        if ($procedure.capture_status -ne "completed_for_boundary_payload" -or
            $procedure.materialization_status -ne "source_boundary_payload_materialized" -or
            $procedure.hash_status -ne "computed" -or
            $procedure.sha256 -ne $selectedSourcePayloadHash) {
            throw "M10 selected source capture procedure must record materialization/hash: $($payload.source_id)"
        }
    } elseif ($payload.source_id -eq $nextRemainingSourceId) {
        if ($procedure.capture_status -ne "completed_for_routine_version_payload" -or
            $procedure.materialization_status -ne "routine_version_payload_materialized" -or
            $procedure.hash_status -ne "computed" -or
            $procedure.sha256 -ne $selectedIauSofaPayloadHash) {
            throw "M10 selected IAU SOFA capture procedure must record materialization/hash"
        }
    } elseif ($payload.source_id -eq $jplHorizonsSourceId) {
        if ($procedure.capture_status -ne "completed_for_validation_query_snapshot_boundary" -or
            $procedure.materialization_status -ne "validation_query_snapshot_payload_materialized" -or
            $procedure.hash_status -ne "computed" -or
            $procedure.sha256 -ne $selectedJplHorizonsPayloadHash) {
            throw "M10 selected JPL Horizons capture procedure must record materialization/hash"
        }
    } elseif ($payload.source_id -eq $gbtSourceId) {
        if ($procedure.capture_status -ne "completed_for_rule_reference_boundary" -or
            $procedure.materialization_status -ne "rule_reference_payload_materialized" -or
            $procedure.hash_status -ne "computed" -or
            $procedure.sha256 -ne $selectedGbTPayloadHash) {
            throw "M10 selected GB/T capture procedure must record rule-reference materialization/hash"
        }
    } elseif ($procedure.capture_status -ne "not_started" -or
        $procedure.materialization_status -ne "not_materialized" -or
        $procedure.hash_status -ne "not_computed") {
        throw "M10 unselected source capture procedure must keep not_started/not_materialized/not_computed: $($payload.source_id)"
    }
    if (@($procedure.capture_steps).Count -lt 5 -or @($procedure.required_evidence_fields).Count -lt 5) {
        throw "M10 source capture procedure must define capture steps and evidence fields: $($payload.source_id)"
    }
}

foreach ($required in @(
    "procedure dry-run passes",
    "payload directory creation policy approved",
    "one source-specific materialization decision recorded",
    "manual reviewer confirms provenance handling",
    "payload hash recording policy defined",
    "no external API call in full project gate policy preserved"
)) {
    if ($sourceCaptureProcedure.required_before_first_payload -notcontains $required) {
        throw "M10 source capture procedure missing requirement: $required"
    }
}

foreach ($forbidden in @(
    "write unselected source payload files",
    "compute unselected source payload hashes",
    "perform external API call in full project gate",
    "write generated astronomy artifacts",
    "compute generated artifact hashes",
    "mark draft manifest accepted",
    "change calendar-date-query runtime behavior",
    "change chart-create runtime behavior",
    "claim astronomy-engine supported"
)) {
    if ($sourceCaptureProcedure.forbidden_in_procedure_stage -notcontains $forbidden) {
        throw "M10 source capture procedure missing forbidden item: $forbidden"
    }
}

if ($sourcePayloadMaterializationDecision.status -ne "decision_only") {
    throw "M10 source payload materialization decision must remain decision_only"
}

if ($sourcePayloadMaterializationDecision.decision -ne "select_first_source_payload_candidate") {
    throw "M10 source payload materialization decision must select the first source payload candidate"
}

if ($sourcePayloadMaterializationDecision.source_snapshot_manifest_id -ne $sourceSnapshotManifest.source_snapshot_manifest_id -or
    $sourcePayloadMaterializationDecision.source_payload_materialization_policy_id -ne $sourcePayloadMaterializationPolicy.source_payload_materialization_policy_id -or
    $sourcePayloadMaterializationDecision.source_capture_procedure_id -ne $sourceCaptureProcedure.source_capture_procedure_id) {
    throw "M10 source payload materialization decision must reference active source manifest, payload policy, and capture procedure"
}

if ($sourcePayloadMaterializationDecision.decision_scope -ne "single_source_only") {
    throw "M10 source payload materialization decision must remain single_source_only"
}

if ($sourcePayloadMaterializationDecision.payload_materialization_allowed_in_this_loop -ne $false -or
    $sourcePayloadMaterializationDecision.payload_directory_creation_allowed_in_this_loop -ne $false -or
    $sourcePayloadMaterializationDecision.payload_hash_computation_allowed_in_this_loop -ne $false -or
    $sourcePayloadMaterializationDecision.generated_artifact_allowed_in_this_loop -ne $false -or
    $sourcePayloadMaterializationDecision.generated_artifact_hash_allowed_in_this_loop -ne $false -or
    $sourcePayloadMaterializationDecision.manifest_acceptance_change_allowed_in_this_loop -ne $false -or
    $sourcePayloadMaterializationDecision.runtime_behavior_change_allowed_in_this_loop -ne $false -or
    $sourcePayloadMaterializationDecision.capability_status -ne "target") {
    throw "M10 source payload materialization decision must not allow payload writes, hashes, artifacts, acceptance changes, runtime changes, or capability promotion"
}

$selectedDecisionSourceId = $sourcePayloadMaterializationDecision.selected_source.source_id
$selectedPayload = @($sourcePayloadMaterializationPolicy.planned_payloads | Where-Object { $_.source_id -eq $selectedDecisionSourceId })
$selectedProcedure = @($sourceCaptureProcedure.procedures | Where-Object { $_.source_id -eq $selectedDecisionSourceId })
$selectedManifestSource = @($sourceSnapshotManifest.sources | Where-Object { $_.source_id -eq $selectedDecisionSourceId })
if ($selectedPayload.Count -ne 1 -or $selectedProcedure.Count -ne 1 -or $selectedManifestSource.Count -ne 1) {
    throw "M10 source payload materialization decision selected source must exist in policy, procedure, and manifest: $selectedDecisionSourceId"
}

if ($sourcePayloadMaterializationDecision.selected_source.payload_kind -ne $selectedPayload[0].payload_kind -or
    $sourcePayloadMaterializationDecision.selected_source.schema_path -ne $selectedPayload[0].schema_path -or
    $sourcePayloadMaterializationDecision.selected_source.payload_path -ne $selectedPayload[0].path -or
    $sourcePayloadMaterializationDecision.selected_source.payload_format -ne $selectedPayload[0].payload_format) {
    throw "M10 source payload materialization decision selected source must match payload policy: $selectedDecisionSourceId"
}

foreach ($required in @(
    "decision dry-run passes",
    "payload directory creation policy changes from missing to approved for selected source only",
    "selected payload canonical JSON writer or manual file procedure is defined",
    "selected payload hash recording policy is defined",
    "selected payload remains source evidence only, not generated astronomy artifact",
    "no external API call in full project gate policy preserved"
)) {
    if ($sourcePayloadMaterializationDecision.required_before_selected_payload_materialization -notcontains $required) {
        throw "M10 source payload materialization decision missing requirement: $required"
    }
}

foreach ($forbidden in @(
    "create payload directory",
    "write selected source payload file",
    "write any other source payload file",
    "compute source payload hash",
    "perform external API call in full project gate",
    "write generated astronomy artifacts",
    "compute generated artifact hashes",
    "mark draft manifest accepted",
    "change calendar-date-query runtime behavior",
    "change chart-create runtime behavior",
    "claim astronomy-engine supported"
)) {
    if ($sourcePayloadMaterializationDecision.forbidden_in_decision_stage -notcontains $forbidden) {
        throw "M10 source payload materialization decision missing forbidden item: $forbidden"
    }
}

if ($selectedSourcePayloadMaterializationPreflight.status -ne "preflight_only") {
    throw "M10 selected source payload materialization preflight must remain preflight_only"
}

if ($selectedSourcePayloadMaterializationPreflight.source_payload_materialization_decision_id -ne $sourcePayloadMaterializationDecision.source_payload_materialization_decision_id -or
    $selectedSourcePayloadMaterializationPreflight.source_payload_materialization_policy_id -ne $sourcePayloadMaterializationPolicy.source_payload_materialization_policy_id -or
    $selectedSourcePayloadMaterializationPreflight.source_capture_procedure_id -ne $sourceCaptureProcedure.source_capture_procedure_id -or
    $selectedSourcePayloadMaterializationPreflight.source_snapshot_manifest_id -ne $sourceSnapshotManifest.source_snapshot_manifest_id) {
    throw "M10 selected source payload materialization preflight must reference active decision, policy, procedure, and manifest"
}

if ($selectedSourcePayloadMaterializationPreflight.selected_source.source_id -ne $sourcePayloadMaterializationDecision.selected_source.source_id -or
    $selectedSourcePayloadMaterializationPreflight.selected_source.source_id -ne "naif-cspice" -or
    $selectedSourcePayloadMaterializationPreflight.selected_source.payload_kind -ne $sourcePayloadMaterializationDecision.selected_source.payload_kind -or
    $selectedSourcePayloadMaterializationPreflight.selected_source.schema_path -ne $sourcePayloadMaterializationDecision.selected_source.schema_path -or
    $selectedSourcePayloadMaterializationPreflight.selected_source.payload_path -ne $sourcePayloadMaterializationDecision.selected_source.payload_path) {
    throw "M10 selected source payload materialization preflight must stay aligned to selected naif-cspice decision"
}

if ($selectedSourcePayloadMaterializationPreflight.payload_directory_policy.create_allowed_in_this_loop -ne $false -or
    $selectedSourcePayloadMaterializationPreflight.payload_directory_policy.next_loop_create_scope -ne "selected_source_only" -or
    $selectedSourcePayloadMaterializationPreflight.payload_directory_policy.path -ne $sourcePayloadMaterializationPolicy.payload_directory.path) {
    throw "M10 selected source payload materialization preflight must keep directory creation blocked this loop and selected-source-only next loop"
}

if ($selectedSourcePayloadMaterializationPreflight.selected_payload_write_policy.write_allowed_in_this_loop -ne $false -or
    $selectedSourcePayloadMaterializationPreflight.selected_payload_write_policy.next_loop_write_scope -ne "selected_source_only" -or
    $selectedSourcePayloadMaterializationPreflight.selected_payload_write_policy.canonical_json_required -ne $true -or
    $selectedSourcePayloadMaterializationPreflight.selected_payload_write_policy.allowed_payload_claim -ne "source-boundary-evidence-only") {
    throw "M10 selected source payload materialization preflight must keep writes blocked this loop and source-only next loop"
}

if ($selectedSourcePayloadMaterializationPreflight.selected_payload_hash_policy.hash_algorithm -ne "sha256" -or
    $selectedSourcePayloadMaterializationPreflight.selected_payload_hash_policy.hash_allowed_in_this_loop -ne $false -or
    $selectedSourcePayloadMaterializationPreflight.selected_payload_hash_policy.next_loop_hash_scope -ne "selected_source_payload_only") {
    throw "M10 selected source payload materialization preflight must keep hashes blocked this loop and selected-payload-only next loop"
}

foreach ($check in @(
    "decision dry-run passes",
    "selected source remains naif-cspice",
    "selected schema remains schema_only",
    "payload directory is absent before materialization",
    "selected payload is absent before materialization",
    "no other planned payload exists",
    "no external API call in full project gate",
    "generated artifact paths remain absent",
    "draft manifest remains not_accepted",
    "runtime behavior unchanged",
    "astronomy-engine remains target"
)) {
    if ($selectedSourcePayloadMaterializationPreflight.preflight_checks -notcontains $check) {
        throw "M10 selected source payload materialization preflight missing check: $check"
    }
}

if ($selectedSourcePayloadMaterializationPreflight.materialization_allowed_after_preflight.selected_source_payload -ne $true -or
    $selectedSourcePayloadMaterializationPreflight.materialization_allowed_after_preflight.other_source_payloads -ne $false -or
    $selectedSourcePayloadMaterializationPreflight.materialization_allowed_after_preflight.generated_astronomy_artifacts -ne $false -or
    $selectedSourcePayloadMaterializationPreflight.materialization_allowed_after_preflight.generated_artifact_hashes -ne $false -or
    $selectedSourcePayloadMaterializationPreflight.materialization_allowed_after_preflight.draft_manifest_acceptance_change -ne $false -or
    $selectedSourcePayloadMaterializationPreflight.materialization_allowed_after_preflight.runtime_behavior_change -ne $false -or
    $selectedSourcePayloadMaterializationPreflight.materialization_allowed_after_preflight.capability_promotion -ne $false) {
    throw "M10 selected source payload materialization preflight must allow only selected source payload after preflight"
}

foreach ($forbidden in @(
    "create payload directory",
    "write selected source payload file",
    "write any other source payload file",
    "compute source payload hash",
    "perform external API call in full project gate",
    "write generated astronomy artifacts",
    "compute generated artifact hashes",
    "mark draft manifest accepted",
    "change calendar-date-query runtime behavior",
    "change chart-create runtime behavior",
    "claim astronomy-engine supported"
)) {
    if ($selectedSourcePayloadMaterializationPreflight.forbidden_in_preflight_stage -notcontains $forbidden) {
        throw "M10 selected source payload materialization preflight missing forbidden item: $forbidden"
    }
}

if ($selectedSourcePayloadMaterialization.status -ne "selected_source_payload_materialized") {
    throw "M10 selected source payload materialization evidence must record selected_source_payload_materialized"
}

if ($selectedSourcePayloadMaterialization.source_payload_materialization_decision_id -ne $sourcePayloadMaterializationDecision.source_payload_materialization_decision_id -or
    $selectedSourcePayloadMaterialization.selected_source_payload_materialization_preflight_id -ne $selectedSourcePayloadMaterializationPreflight.selected_source_payload_materialization_preflight_id -or
    $selectedSourcePayloadMaterialization.source_payload_materialization_policy_id -ne $sourcePayloadMaterializationPolicy.source_payload_materialization_policy_id -or
    $selectedSourcePayloadMaterialization.source_snapshot_manifest_id -ne $sourceSnapshotManifest.source_snapshot_manifest_id) {
    throw "M10 selected source payload materialization evidence must reference active decision, preflight, policy, and manifest"
}

if ($selectedSourcePayloadMaterialization.selected_source.source_id -ne $selectedMaterializedSourceId -or
    $selectedSourcePayloadMaterialization.selected_source.payload_kind -ne "offline-kernel-toolkit-boundary" -or
    $selectedSourcePayloadMaterialization.selected_source.payload_path -ne "data/generated/astronomy/source-snapshots/payloads/naif-cspice-kernel-boundary.json" -or
    $selectedSourcePayloadMaterialization.selected_source.payload_status -ne "materialized" -or
    $selectedSourcePayloadMaterialization.selected_source.hash_algorithm -ne "sha256" -or
    $selectedSourcePayloadMaterialization.selected_source.sha256 -ne $selectedSourcePayloadHash) {
    throw "M10 selected source payload materialization evidence must record selected payload path/status/hash"
}

if ($selectedSourcePayloadMaterialization.materialized_payload_count -ne 1 -or
    $selectedSourcePayloadMaterialization.allowed_materialized_sources -notcontains $selectedMaterializedSourceId) {
    throw "M10 selected source payload materialization evidence must allow exactly the selected source"
}

foreach ($unselectedSourceId in @("gb-t-33661-2017", "jpl-horizons-api", "iau-sofa-ansi-c")) {
    if ($selectedSourcePayloadMaterialization.forbidden_materialized_sources -notcontains $unselectedSourceId) {
        throw "M10 selected source payload materialization evidence must forbid unselected source: $unselectedSourceId"
    }
}

if ($selectedSourcePayloadMaterialization.source_payload_claim -ne "source-boundary-evidence-only" -or
    $selectedSourcePayloadMaterialization.generated_artifact_allowed -ne $false -or
    $selectedSourcePayloadMaterialization.generated_artifact_hash_allowed -ne $false -or
    $selectedSourcePayloadMaterialization.manifest_acceptance_change_allowed -ne $false -or
    $selectedSourcePayloadMaterialization.runtime_behavior_change_allowed -ne $false -or
    $selectedSourcePayloadMaterialization.capability_status -ne "target") {
    throw "M10 selected source payload materialization evidence must forbid generated artifacts, acceptance changes, runtime changes, and capability promotion"
}

$selectedMaterializedPayloadPath = Join-Path $projectPath $selectedSourcePayloadMaterialization.selected_source.payload_path
if (-not (Test-Path -LiteralPath $selectedMaterializedPayloadPath)) {
    throw "M10 selected source payload materialization evidence references missing payload"
}
$selectedMaterializedPayloadHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $selectedMaterializedPayloadPath).Hash.ToLowerInvariant()
if ($selectedMaterializedPayloadHash -ne $selectedSourcePayloadHash) {
    throw "M10 selected source payload materialization evidence hash mismatch: $selectedMaterializedPayloadHash"
}

foreach ($forbidden in @(
    "write any other source payload file",
    "write generated astronomy artifacts",
    "compute generated artifact hashes",
    "mark draft manifest accepted",
    "change calendar-date-query runtime behavior",
    "change chart-create runtime behavior",
    "replace android-date-layer-v1",
    "claim astronomy-engine supported",
    "claim CSPICE toolkit integrated",
    "claim SPICE kernel materialized"
)) {
    if ($selectedSourcePayloadMaterialization.forbidden_after_materialization -notcontains $forbidden) {
        throw "M10 selected source payload materialization evidence missing forbidden item: $forbidden"
    }
}

if ($remainingSourcePayloadStrategy.status -ne "strategy_decision_only") {
    throw "M10 remaining source payload strategy must remain strategy_decision_only"
}

if ($remainingSourcePayloadStrategy.source_payload_materialization_policy_id -ne $sourcePayloadMaterializationPolicy.source_payload_materialization_policy_id -or
    $remainingSourcePayloadStrategy.source_capture_procedure_id -ne $sourceCaptureProcedure.source_capture_procedure_id -or
    $remainingSourcePayloadStrategy.source_snapshot_manifest_id -ne $sourceSnapshotManifest.source_snapshot_manifest_id -or
    $remainingSourcePayloadStrategy.selected_source_payload_materialization_id -ne $selectedSourcePayloadMaterialization.selected_source_payload_materialization_id) {
    throw "M10 remaining source payload strategy must reference active policy, procedure, manifest, and selected materialization"
}

if (@($remainingSourcePayloadStrategy.currently_materialized_sources).Count -ne 1 -or
    $remainingSourcePayloadStrategy.currently_materialized_sources[0].source_id -ne $selectedMaterializedSourceId -or
    $remainingSourcePayloadStrategy.currently_materialized_sources[0].sha256 -ne $selectedSourcePayloadHash) {
    throw "M10 remaining source payload strategy must record exactly one selected naif-cspice materialization"
}

if (@($remainingSourcePayloadStrategy.remaining_source_sequence).Count -ne 3) {
    throw "M10 remaining source payload strategy must order the three unmaterialized sources"
}

$remainingSequenceIds = @($remainingSourcePayloadStrategy.remaining_source_sequence | Sort-Object order | ForEach-Object { $_.source_id })
if ($remainingSequenceIds[0] -ne $nextRemainingSourceId -or
    $remainingSequenceIds[1] -ne "jpl-horizons-api" -or
    $remainingSequenceIds[2] -ne "gb-t-33661-2017") {
    throw "M10 remaining source payload strategy must choose IAU SOFA, then JPL Horizons, then GB/T"
}

$nextRemainingPayload = @($sourcePayloadMaterializationPolicy.planned_payloads | Where-Object { $_.source_id -eq $nextRemainingSourceId })
$nextRemainingProcedure = @($sourceCaptureProcedure.procedures | Where-Object { $_.source_id -eq $nextRemainingSourceId })
$nextRemainingManifest = @($sourceSnapshotManifest.sources | Where-Object { $_.source_id -eq $nextRemainingSourceId })
if ($nextRemainingPayload.Count -ne 1 -or $nextRemainingProcedure.Count -ne 1 -or $nextRemainingManifest.Count -ne 1) {
    throw "M10 remaining source payload strategy next source must exist in policy, procedure, and manifest"
}

if ($remainingSourcePayloadStrategy.next_selected_source.source_id -ne $nextRemainingSourceId -or
    $remainingSourcePayloadStrategy.next_selected_source.payload_kind -ne $nextRemainingPayload[0].payload_kind -or
    $remainingSourcePayloadStrategy.next_selected_source.schema_path -ne $nextRemainingPayload[0].schema_path -or
    $remainingSourcePayloadStrategy.next_selected_source.payload_path -ne $nextRemainingPayload[0].path -or
    $remainingSourcePayloadStrategy.next_selected_source.payload_format -ne $nextRemainingPayload[0].payload_format) {
    throw "M10 remaining source payload strategy next selected source must match payload policy"
}

$gbtMaterializedPayload = @($sourcePayloadMaterializationPolicy.planned_payloads | Where-Object { $_.source_id -eq $gbtSourceId })
if ($gbtMaterializedPayload.Count -ne 1 -or
    $gbtMaterializedPayload[0].payload_status -ne "materialized" -or
    $gbtMaterializedPayload[0].hash_status -ne "computed" -or
    $gbtMaterializedPayload[0].sha256 -ne $selectedGbTPayloadHash) {
    throw "M10 GB/T payload must record LOOP-054 materialized/computed/hash"
}
$gbtMaterializedPayloadPath = Join-Path $projectPath $gbtMaterializedPayload[0].path
if (-not (Test-Path -LiteralPath $gbtMaterializedPayloadPath)) {
    throw "M10 GB/T materialized source payload file must exist: $($gbtMaterializedPayload[0].path)"
}
$actualGbTMaterializedPayloadHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $gbtMaterializedPayloadPath).Hash.ToLowerInvariant()
if ($actualGbTMaterializedPayloadHash -ne $selectedGbTPayloadHash) {
    throw "M10 GB/T materialized source payload hash mismatch: $actualGbTMaterializedPayloadHash"
}

if ($remainingSourcePayloadStrategy.allowed_next_loop.selected_source_payload_preflight -ne $true -or
    $remainingSourcePayloadStrategy.allowed_next_loop.selected_source_id -ne $nextRemainingSourceId -or
    $remainingSourcePayloadStrategy.allowed_next_loop.payload_materialization -ne $false -or
    $remainingSourcePayloadStrategy.allowed_next_loop.payload_hash_computation -ne $false -or
    $remainingSourcePayloadStrategy.allowed_next_loop.generated_astronomy_artifacts -ne $false -or
    $remainingSourcePayloadStrategy.allowed_next_loop.generated_artifact_hashes -ne $false -or
    $remainingSourcePayloadStrategy.allowed_next_loop.draft_manifest_acceptance_change -ne $false -or
    $remainingSourcePayloadStrategy.allowed_next_loop.runtime_behavior_change -ne $false -or
    $remainingSourcePayloadStrategy.allowed_next_loop.capability_promotion -ne $false) {
    throw "M10 remaining source payload strategy must allow only next-loop IAU SOFA preflight"
}

foreach ($check in @(
    "exactly one source payload is materialized before strategy decision",
    "naif-cspice payload hash remains unchanged",
    "remaining sources are not_materialized",
    "next source is selected from remaining sources",
    "next loop is preflight-only",
    "no external API call in full project gate",
    "generated artifact paths remain absent",
    "draft manifest remains not_accepted",
    "runtime behavior unchanged",
    "astronomy-engine remains target"
)) {
    if ($remainingSourcePayloadStrategy.strategy_checks -notcontains $check) {
        throw "M10 remaining source payload strategy missing check: $check"
    }
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
    if ($remainingSourcePayloadStrategy.forbidden_in_strategy_stage -notcontains $forbidden) {
        throw "M10 remaining source payload strategy missing forbidden item: $forbidden"
    }
}

if ($selectedIauSofaPayloadMaterializationPreflight.status -ne "preflight_only") {
    throw "M10 selected IAU SOFA payload materialization preflight must remain preflight_only"
}

if ($selectedIauSofaPayloadMaterializationPreflight.remaining_source_payload_strategy_id -ne $remainingSourcePayloadStrategy.remaining_source_payload_strategy_id -or
    $selectedIauSofaPayloadMaterializationPreflight.source_payload_materialization_policy_id -ne $sourcePayloadMaterializationPolicy.source_payload_materialization_policy_id -or
    $selectedIauSofaPayloadMaterializationPreflight.source_capture_procedure_id -ne $sourceCaptureProcedure.source_capture_procedure_id -or
    $selectedIauSofaPayloadMaterializationPreflight.source_snapshot_manifest_id -ne $sourceSnapshotManifest.source_snapshot_manifest_id) {
    throw "M10 selected IAU SOFA preflight must reference active strategy, policy, procedure, and manifest"
}

if ($selectedIauSofaPayloadMaterializationPreflight.selected_source.source_id -ne $nextRemainingSourceId -or
    $selectedIauSofaPayloadMaterializationPreflight.selected_source.payload_kind -ne $nextRemainingPayload[0].payload_kind -or
    $selectedIauSofaPayloadMaterializationPreflight.selected_source.schema_path -ne $nextRemainingPayload[0].schema_path -or
    $selectedIauSofaPayloadMaterializationPreflight.selected_source.payload_path -ne $nextRemainingPayload[0].path -or
    $selectedIauSofaPayloadMaterializationPreflight.selected_source.payload_format -ne $nextRemainingPayload[0].payload_format) {
    throw "M10 selected IAU SOFA preflight selected source must match the remaining source strategy and payload policy"
}

if (@($selectedIauSofaPayloadMaterializationPreflight.current_materialized_payloads).Count -ne 1 -or
    $selectedIauSofaPayloadMaterializationPreflight.current_materialized_payloads[0].source_id -ne $selectedMaterializedSourceId -or
    $selectedIauSofaPayloadMaterializationPreflight.current_materialized_payloads[0].sha256 -ne $selectedSourcePayloadHash) {
    throw "M10 selected IAU SOFA preflight must preserve the existing naif-cspice materialization record"
}

if ($selectedIauSofaPayloadMaterializationPreflight.payload_directory_policy.path -ne $sourcePayloadMaterializationPolicy.payload_directory.path -or
    $selectedIauSofaPayloadMaterializationPreflight.payload_directory_policy.current_status -ne "exists_selected_source_only" -or
    $selectedIauSofaPayloadMaterializationPreflight.payload_directory_policy.existing_materialized_source_count -ne 1 -or
    $selectedIauSofaPayloadMaterializationPreflight.payload_directory_policy.create_allowed_in_this_loop -ne $false -or
    $selectedIauSofaPayloadMaterializationPreflight.payload_directory_policy.next_loop_write_scope -ne "selected_source_only") {
    throw "M10 selected IAU SOFA preflight must preserve selected-source-only payload directory scope"
}

if ($selectedIauSofaPayloadMaterializationPreflight.selected_payload_write_policy.write_allowed_in_this_loop -ne $false -or
    $selectedIauSofaPayloadMaterializationPreflight.selected_payload_write_policy.next_loop_write_scope -ne "selected_source_only" -or
    $selectedIauSofaPayloadMaterializationPreflight.selected_payload_write_policy.canonical_json_required -ne $true -or
    $selectedIauSofaPayloadMaterializationPreflight.selected_payload_write_policy.allowed_payload_claim -ne "local-routine-version-boundary-only") {
    throw "M10 selected IAU SOFA preflight must block writes this loop and allow only selected-source writes next loop"
}

if ($selectedIauSofaPayloadMaterializationPreflight.selected_payload_hash_policy.hash_algorithm -ne "sha256" -or
    $selectedIauSofaPayloadMaterializationPreflight.selected_payload_hash_policy.hash_allowed_in_this_loop -ne $false -or
    $selectedIauSofaPayloadMaterializationPreflight.selected_payload_hash_policy.next_loop_hash_scope -ne "selected_source_payload_only") {
    throw "M10 selected IAU SOFA preflight must block hashes this loop and scope next-loop hash to selected payload"
}

$selectedIauSofaSchemaPath = Join-Path $projectPath $selectedIauSofaPayloadMaterializationPreflight.selected_source.schema_path
if (-not (Test-Path -LiteralPath $selectedIauSofaSchemaPath)) {
    throw "M10 selected IAU SOFA preflight schema missing"
}
$selectedIauSofaSchema = Get-Content -LiteralPath $selectedIauSofaSchemaPath -Encoding UTF8 -Raw | ConvertFrom-Json
if ($selectedIauSofaSchema.status -ne "schema_only" -or
    $selectedIauSofaSchema.source_id -ne $nextRemainingSourceId -or
    $selectedIauSofaSchema.payload_kind -ne $selectedIauSofaPayloadMaterializationPreflight.selected_source.payload_kind) {
    throw "M10 selected IAU SOFA preflight schema must remain schema_only and match selected source"
}

if (-not (Test-Path -LiteralPath $gbtMaterializedPayloadPath)) {
    throw "M10 selected IAU SOFA closed preflight must observe GB/T payload after LOOP-054: $($gbtMaterializedPayload[0].path)"
}

$materializedIauSofaPayload = @($sourcePayloadMaterializationPolicy.planned_payloads | Where-Object { $_.source_id -eq $nextRemainingSourceId })
$materializedIauSofaPayloadPath = Join-Path $projectPath $materializedIauSofaPayload[0].path
if ($materializedIauSofaPayload.Count -ne 1 -or
    $materializedIauSofaPayload[0].payload_status -ne "materialized" -or
    $materializedIauSofaPayload[0].hash_status -ne "computed" -or
    $materializedIauSofaPayload[0].sha256 -ne $selectedIauSofaPayloadHash -or
    -not (Test-Path -LiteralPath $materializedIauSofaPayloadPath)) {
    throw "M10 selected IAU SOFA payload must be materialized after preflight closes"
}

foreach ($claim in $selectedIauSofaPayloadMaterializationPreflight.selected_payload_write_policy.forbidden_payload_claims) {
    if ($selectedIauSofaSchema.forbidden_claims -contains $claim) {
        continue
    }
    if ($claim -eq "Android baseline replaced") {
        continue
    }
    throw "M10 selected IAU SOFA schema missing forbidden payload claim: $claim"
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
    if ($selectedIauSofaPayloadMaterializationPreflight.preflight_checks -notcontains $check) {
        throw "M10 selected IAU SOFA preflight missing check: $check"
    }
}

if ($selectedIauSofaPayloadMaterializationPreflight.materialization_allowed_after_preflight.selected_source_payload -ne $true -or
    $selectedIauSofaPayloadMaterializationPreflight.materialization_allowed_after_preflight.selected_source_id -ne $nextRemainingSourceId -or
    $selectedIauSofaPayloadMaterializationPreflight.materialization_allowed_after_preflight.other_remaining_source_payloads -ne $false -or
    $selectedIauSofaPayloadMaterializationPreflight.materialization_allowed_after_preflight.generated_astronomy_artifacts -ne $false -or
    $selectedIauSofaPayloadMaterializationPreflight.materialization_allowed_after_preflight.generated_artifact_hashes -ne $false -or
    $selectedIauSofaPayloadMaterializationPreflight.materialization_allowed_after_preflight.draft_manifest_acceptance_change -ne $false -or
    $selectedIauSofaPayloadMaterializationPreflight.materialization_allowed_after_preflight.runtime_behavior_change -ne $false -or
    $selectedIauSofaPayloadMaterializationPreflight.materialization_allowed_after_preflight.capability_promotion -ne $false) {
    throw "M10 selected IAU SOFA preflight must allow only selected source payload after preflight"
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
    if ($selectedIauSofaPayloadMaterializationPreflight.forbidden_in_preflight_stage -notcontains $forbidden) {
        throw "M10 selected IAU SOFA preflight missing forbidden item: $forbidden"
    }
}

if ($selectedIauSofaPayloadMaterialization.status -ne "selected_source_payload_materialized") {
    throw "M10 selected IAU SOFA payload materialization evidence must record selected_source_payload_materialized"
}

if ($selectedIauSofaPayloadMaterialization.remaining_source_payload_strategy_id -ne $remainingSourcePayloadStrategy.remaining_source_payload_strategy_id -or
    $selectedIauSofaPayloadMaterialization.selected_iau_sofa_payload_materialization_preflight_id -ne $selectedIauSofaPayloadMaterializationPreflight.selected_source_payload_materialization_preflight_id -or
    $selectedIauSofaPayloadMaterialization.source_payload_materialization_policy_id -ne $sourcePayloadMaterializationPolicy.source_payload_materialization_policy_id -or
    $selectedIauSofaPayloadMaterialization.source_capture_procedure_id -ne $sourceCaptureProcedure.source_capture_procedure_id -or
    $selectedIauSofaPayloadMaterialization.source_snapshot_manifest_id -ne $sourceSnapshotManifest.source_snapshot_manifest_id) {
    throw "M10 selected IAU SOFA materialization evidence must reference active strategy, preflight, policy, procedure, and manifest"
}

if ($selectedIauSofaPayloadMaterialization.selected_source.source_id -ne $nextRemainingSourceId -or
    $selectedIauSofaPayloadMaterialization.selected_source.payload_kind -ne "local-routine-version-record" -or
    $selectedIauSofaPayloadMaterialization.selected_source.payload_path -ne "data/generated/astronomy/source-snapshots/payloads/iau-sofa-routine-version.json" -or
    $selectedIauSofaPayloadMaterialization.selected_source.payload_status -ne "materialized" -or
    $selectedIauSofaPayloadMaterialization.selected_source.hash_algorithm -ne "sha256" -or
    $selectedIauSofaPayloadMaterialization.selected_source.sha256 -ne $selectedIauSofaPayloadHash) {
    throw "M10 selected IAU SOFA materialization evidence must record selected payload path/status/hash"
}

if ($selectedIauSofaPayloadMaterialization.materialized_payload_count -ne 2 -or
    $selectedIauSofaPayloadMaterialization.allowed_materialized_sources -notcontains $selectedMaterializedSourceId -or
    $selectedIauSofaPayloadMaterialization.allowed_materialized_sources -notcontains $nextRemainingSourceId) {
    throw "M10 selected IAU SOFA materialization evidence must allow exactly NAIF and IAU SOFA selected sources"
}

foreach ($unselectedSourceId in @("jpl-horizons-api", "gb-t-33661-2017")) {
    if ($selectedIauSofaPayloadMaterialization.forbidden_materialized_sources -notcontains $unselectedSourceId) {
        throw "M10 selected IAU SOFA materialization evidence must forbid unselected source: $unselectedSourceId"
    }
}

if ($selectedIauSofaPayloadMaterialization.source_payload_claim -ne "source-boundary-evidence-only" -or
    $selectedIauSofaPayloadMaterialization.generated_artifact_allowed -ne $false -or
    $selectedIauSofaPayloadMaterialization.generated_artifact_hash_allowed -ne $false -or
    $selectedIauSofaPayloadMaterialization.manifest_acceptance_change_allowed -ne $false -or
    $selectedIauSofaPayloadMaterialization.runtime_behavior_change_allowed -ne $false -or
    $selectedIauSofaPayloadMaterialization.capability_status -ne "target") {
    throw "M10 selected IAU SOFA materialization evidence must forbid generated artifacts, acceptance changes, runtime changes, and capability promotion"
}

$selectedIauSofaMaterializedPayloadPath = Join-Path $projectPath $selectedIauSofaPayloadMaterialization.selected_source.payload_path
if (-not (Test-Path -LiteralPath $selectedIauSofaMaterializedPayloadPath)) {
    throw "M10 selected IAU SOFA materialization evidence references missing payload"
}
$selectedIauSofaMaterializedPayloadHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $selectedIauSofaMaterializedPayloadPath).Hash.ToLowerInvariant()
if ($selectedIauSofaMaterializedPayloadHash -ne $selectedIauSofaPayloadHash) {
    throw "M10 selected IAU SOFA materialization evidence hash mismatch: $selectedIauSofaMaterializedPayloadHash"
}

foreach ($forbidden in @(
    "write jpl-horizons payload file",
    "write gb-t payload file",
    "write generated astronomy artifacts",
    "compute generated artifact hashes",
    "mark draft manifest accepted",
    "change calendar-date-query runtime behavior",
    "change chart-create runtime behavior",
    "replace android-date-layer-v1",
    "claim astronomy-engine supported",
    "claim SOFA routine integrated",
    "claim runtime dependency enabled"
)) {
    if ($selectedIauSofaPayloadMaterialization.forbidden_after_materialization -notcontains $forbidden) {
        throw "M10 selected IAU SOFA materialization evidence missing forbidden item: $forbidden"
    }
}

if ($postIauRemainingSourcePayloadStrategy.status -ne "strategy_decision_only") {
    throw "M10 post-IAU remaining source payload strategy must remain strategy_decision_only"
}

if ($postIauRemainingSourcePayloadStrategy.source_payload_materialization_policy_id -ne $sourcePayloadMaterializationPolicy.source_payload_materialization_policy_id -or
    $postIauRemainingSourcePayloadStrategy.source_capture_procedure_id -ne $sourceCaptureProcedure.source_capture_procedure_id -or
    $postIauRemainingSourcePayloadStrategy.source_snapshot_manifest_id -ne $sourceSnapshotManifest.source_snapshot_manifest_id -or
    $postIauRemainingSourcePayloadStrategy.selected_iau_sofa_payload_materialization_id -ne $selectedIauSofaPayloadMaterialization.selected_source_payload_materialization_id -or
    $postIauRemainingSourcePayloadStrategy.previous_remaining_source_payload_strategy_id -ne $remainingSourcePayloadStrategy.remaining_source_payload_strategy_id) {
    throw "M10 post-IAU remaining source payload strategy must reference active policy, procedure, manifest, IAU materialization, and previous strategy"
}

if (@($postIauRemainingSourcePayloadStrategy.currently_materialized_sources).Count -ne 2) {
    throw "M10 post-IAU remaining source payload strategy must record exactly two materialized sources"
}

$postIauNaifSource = @($postIauRemainingSourcePayloadStrategy.currently_materialized_sources | Where-Object { $_.source_id -eq $selectedMaterializedSourceId })
$postIauSofaSource = @($postIauRemainingSourcePayloadStrategy.currently_materialized_sources | Where-Object { $_.source_id -eq $nextRemainingSourceId })
if ($postIauNaifSource.Count -ne 1 -or $postIauNaifSource[0].sha256 -ne $selectedSourcePayloadHash -or
    $postIauSofaSource.Count -ne 1 -or $postIauSofaSource[0].sha256 -ne $selectedIauSofaPayloadHash) {
    throw "M10 post-IAU remaining source payload strategy must preserve NAIF and IAU SOFA hashes"
}

if (@($postIauRemainingSourcePayloadStrategy.remaining_source_sequence).Count -ne 2) {
    throw "M10 post-IAU remaining source payload strategy must order the two remaining sources"
}

$postIauSequenceIds = @($postIauRemainingSourcePayloadStrategy.remaining_source_sequence | Sort-Object order | ForEach-Object { $_.source_id })
if ($postIauSequenceIds[0] -ne "jpl-horizons-api" -or $postIauSequenceIds[1] -ne "gb-t-33661-2017") {
    throw "M10 post-IAU remaining source payload strategy must choose JPL Horizons, then GB/T"
}

$postIauNextPayload = @($sourcePayloadMaterializationPolicy.planned_payloads | Where-Object { $_.source_id -eq "jpl-horizons-api" })
$postIauNextProcedure = @($sourceCaptureProcedure.procedures | Where-Object { $_.source_id -eq "jpl-horizons-api" })
$postIauNextManifest = @($sourceSnapshotManifest.sources | Where-Object { $_.source_id -eq "jpl-horizons-api" })
if ($postIauNextPayload.Count -ne 1 -or $postIauNextProcedure.Count -ne 1 -or $postIauNextManifest.Count -ne 1) {
    throw "M10 post-IAU next selected source must exist in policy, procedure, and manifest"
}

if ($postIauRemainingSourcePayloadStrategy.next_selected_source.source_id -ne "jpl-horizons-api" -or
    $postIauRemainingSourcePayloadStrategy.next_selected_source.payload_kind -ne $postIauNextPayload[0].payload_kind -or
    $postIauRemainingSourcePayloadStrategy.next_selected_source.schema_path -ne $postIauNextPayload[0].schema_path -or
    $postIauRemainingSourcePayloadStrategy.next_selected_source.payload_path -ne $postIauNextPayload[0].path -or
    $postIauRemainingSourcePayloadStrategy.next_selected_source.payload_format -ne $postIauNextPayload[0].payload_format) {
    throw "M10 post-IAU next selected source must match payload policy"
}

$postIauNextSchemaPath = Join-Path $projectPath $postIauRemainingSourcePayloadStrategy.next_selected_source.schema_path
if (-not (Test-Path -LiteralPath $postIauNextSchemaPath)) {
    throw "M10 post-IAU next selected source schema is missing"
}
$postIauNextSchema = Get-Content -LiteralPath $postIauNextSchemaPath -Encoding UTF8 -Raw | ConvertFrom-Json
if ($postIauNextSchema.status -ne "schema_only" -or
    $postIauNextSchema.source_id -ne "jpl-horizons-api" -or
    $postIauNextSchema.payload_kind -ne "validation-query-snapshot-set") {
    throw "M10 post-IAU next selected source schema must remain schema_only for JPL Horizons"
}

$postIauGbTPayload = @($sourcePayloadMaterializationPolicy.planned_payloads | Where-Object { $_.source_id -eq $gbtSourceId })
$postIauGbTProcedure = @($sourceCaptureProcedure.procedures | Where-Object { $_.source_id -eq $gbtSourceId })
if ($postIauGbTPayload.Count -ne 1 -or $postIauGbTProcedure.Count -ne 1) {
    throw "M10 post-IAU GB/T source missing policy/procedure entry"
}
if ($postIauGbTPayload[0].payload_status -ne "materialized" -or
    $postIauGbTPayload[0].hash_status -ne "computed" -or
    $postIauGbTPayload[0].sha256 -ne $selectedGbTPayloadHash -or
    $postIauGbTProcedure[0].capture_status -ne "completed_for_rule_reference_boundary" -or
    $postIauGbTProcedure[0].materialization_status -ne "rule_reference_payload_materialized" -or
    $postIauGbTProcedure[0].hash_status -ne "computed" -or
    $postIauGbTProcedure[0].sha256 -ne $selectedGbTPayloadHash) {
    throw "M10 post-IAU GB/T source must record LOOP-054 materialized/computed/hash"
}
if (-not (Test-Path -LiteralPath (Join-Path $projectPath $postIauGbTPayload[0].path))) {
    throw "M10 post-IAU GB/T source payload file must exist after LOOP-054: $($postIauGbTPayload[0].path)"
}

if ($postIauRemainingSourcePayloadStrategy.allowed_next_loop.selected_source_payload_preflight -ne $true -or
    $postIauRemainingSourcePayloadStrategy.allowed_next_loop.selected_source_id -ne "jpl-horizons-api" -or
    $postIauRemainingSourcePayloadStrategy.allowed_next_loop.payload_materialization -ne $false -or
    $postIauRemainingSourcePayloadStrategy.allowed_next_loop.payload_hash_computation -ne $false -or
    $postIauRemainingSourcePayloadStrategy.allowed_next_loop.generated_astronomy_artifacts -ne $false -or
    $postIauRemainingSourcePayloadStrategy.allowed_next_loop.generated_artifact_hashes -ne $false -or
    $postIauRemainingSourcePayloadStrategy.allowed_next_loop.draft_manifest_acceptance_change -ne $false -or
    $postIauRemainingSourcePayloadStrategy.allowed_next_loop.runtime_behavior_change -ne $false -or
    $postIauRemainingSourcePayloadStrategy.allowed_next_loop.capability_promotion -ne $false) {
    throw "M10 post-IAU remaining source payload strategy must allow only next-loop JPL preflight"
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
    if ($postIauRemainingSourcePayloadStrategy.strategy_checks -notcontains $check) {
        throw "M10 post-IAU remaining source payload strategy missing check: $check"
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
    if ($postIauRemainingSourcePayloadStrategy.forbidden_in_strategy_stage -notcontains $forbidden) {
        throw "M10 post-IAU remaining source payload strategy missing forbidden item: $forbidden"
    }
}

if ($selectedJplHorizonsPayloadMaterializationPreflight.status -ne "preflight_only") {
    throw "M10 selected JPL Horizons payload materialization preflight must remain preflight_only"
}

if ($selectedJplHorizonsPayloadMaterializationPreflight.post_iau_remaining_source_payload_strategy_id -ne $postIauRemainingSourcePayloadStrategy.post_iau_remaining_source_payload_strategy_id -or
    $selectedJplHorizonsPayloadMaterializationPreflight.source_payload_materialization_policy_id -ne $sourcePayloadMaterializationPolicy.source_payload_materialization_policy_id -or
    $selectedJplHorizonsPayloadMaterializationPreflight.source_capture_procedure_id -ne $sourceCaptureProcedure.source_capture_procedure_id -or
    $selectedJplHorizonsPayloadMaterializationPreflight.source_snapshot_manifest_id -ne $sourceSnapshotManifest.source_snapshot_manifest_id) {
    throw "M10 selected JPL Horizons preflight must reference active post-IAU strategy, policy, procedure, and manifest"
}

if ($selectedJplHorizonsPayloadMaterializationPreflight.selected_source.source_id -ne $jplHorizonsSourceId) {
    throw "M10 selected JPL Horizons preflight must remain scoped to jpl-horizons-api"
}

$selectedJplPayload = @($sourcePayloadMaterializationPolicy.planned_payloads | Where-Object { $_.source_id -eq $jplHorizonsSourceId })
$selectedJplProcedure = @($sourceCaptureProcedure.procedures | Where-Object { $_.source_id -eq $jplHorizonsSourceId })
$selectedJplManifest = @($sourceSnapshotManifest.sources | Where-Object { $_.source_id -eq $jplHorizonsSourceId })
if ($selectedJplPayload.Count -ne 1 -or $selectedJplProcedure.Count -ne 1 -or $selectedJplManifest.Count -ne 1) {
    throw "M10 selected JPL Horizons source must exist in policy, procedure, and manifest"
}

if ($selectedJplHorizonsPayloadMaterializationPreflight.selected_source.payload_kind -ne $selectedJplPayload[0].payload_kind -or
    $selectedJplHorizonsPayloadMaterializationPreflight.selected_source.schema_path -ne $selectedJplPayload[0].schema_path -or
    $selectedJplHorizonsPayloadMaterializationPreflight.selected_source.payload_path -ne $selectedJplPayload[0].path -or
    $selectedJplHorizonsPayloadMaterializationPreflight.selected_source.payload_format -ne $selectedJplPayload[0].payload_format) {
    throw "M10 selected JPL Horizons preflight source must match payload policy"
}

if ($selectedJplPayload[0].payload_status -ne "materialized" -or
    $selectedJplPayload[0].hash_status -ne "computed" -or
    $selectedJplPayload[0].sha256 -ne $selectedJplHorizonsPayloadHash -or
    $selectedJplProcedure[0].capture_status -ne "completed_for_validation_query_snapshot_boundary" -or
    $selectedJplProcedure[0].materialization_status -ne "validation_query_snapshot_payload_materialized" -or
    $selectedJplProcedure[0].hash_status -ne "computed" -or
    $selectedJplProcedure[0].sha256 -ne $selectedJplHorizonsPayloadHash) {
    throw "M10 selected JPL Horizons payload/procedure must record materialization/hash after LOOP-052"
}

$selectedJplSchemaPath = Join-Path $projectPath $selectedJplHorizonsPayloadMaterializationPreflight.selected_source.schema_path
if (-not (Test-Path -LiteralPath $selectedJplSchemaPath)) {
    throw "M10 selected JPL Horizons schema is missing"
}
$selectedJplSchema = Get-Content -LiteralPath $selectedJplSchemaPath -Encoding UTF8 -Raw | ConvertFrom-Json
if ($selectedJplSchema.status -ne "schema_only" -or
    $selectedJplSchema.source_id -ne $jplHorizonsSourceId -or
    $selectedJplSchema.payload_kind -ne "validation-query-snapshot-set") {
    throw "M10 selected JPL Horizons schema must remain schema_only and match payload kind"
}

foreach ($field in $selectedJplSchema.required_fields) {
    if ($selectedJplHorizonsPayloadMaterializationPreflight.offline_query_boundary_policy.required_payload_fields -notcontains $field) {
        throw "M10 selected JPL Horizons preflight missing payload field: $field"
    }
}

foreach ($field in $selectedJplSchema.required_query_snapshot_fields) {
    if ($selectedJplHorizonsPayloadMaterializationPreflight.offline_query_boundary_policy.required_query_snapshot_fields -notcontains $field) {
        throw "M10 selected JPL Horizons preflight missing query snapshot field: $field"
    }
}

if ($selectedJplHorizonsPayloadMaterializationPreflight.payload_directory_policy.path -ne $sourcePayloadMaterializationPolicy.payload_directory.path -or
    $selectedJplHorizonsPayloadMaterializationPreflight.payload_directory_policy.current_status -ne "exists_selected_source_only" -or
    $selectedJplHorizonsPayloadMaterializationPreflight.payload_directory_policy.existing_materialized_source_count -ne 2 -or
    $selectedJplHorizonsPayloadMaterializationPreflight.payload_directory_policy.create_allowed_in_this_loop -ne $false -or
    $selectedJplHorizonsPayloadMaterializationPreflight.payload_directory_policy.next_loop_write_scope -ne "selected_source_only") {
    throw "M10 selected JPL Horizons preflight must preserve selected-source-only payload directory policy"
}

if ($selectedJplHorizonsPayloadMaterializationPreflight.selected_payload_write_policy.write_allowed_in_this_loop -ne $false -or
    $selectedJplHorizonsPayloadMaterializationPreflight.selected_payload_write_policy.next_loop_write_scope -ne "selected_source_only" -or
    $selectedJplHorizonsPayloadMaterializationPreflight.selected_payload_write_policy.canonical_json_required -ne $true -or
    $selectedJplHorizonsPayloadMaterializationPreflight.selected_payload_write_policy.allowed_payload_claim -ne "offline-validation-query-snapshot-boundary-only") {
    throw "M10 selected JPL Horizons preflight must keep writes blocked this loop and source-only next loop"
}

if ($selectedJplHorizonsPayloadMaterializationPreflight.selected_payload_hash_policy.hash_algorithm -ne "sha256" -or
    $selectedJplHorizonsPayloadMaterializationPreflight.selected_payload_hash_policy.hash_allowed_in_this_loop -ne $false -or
    $selectedJplHorizonsPayloadMaterializationPreflight.selected_payload_hash_policy.next_loop_hash_scope -ne "selected_source_payload_only") {
    throw "M10 selected JPL Horizons preflight must keep hashes blocked this loop and scoped next loop"
}

if ($selectedJplHorizonsPayloadMaterializationPreflight.offline_query_boundary_policy.full_gate_network_policy -ne "no_external_calls" -or
    $selectedJplHorizonsPayloadMaterializationPreflight.offline_query_boundary_policy.query_execution_allowed_in_this_loop -ne $false -or
    $selectedJplHorizonsPayloadMaterializationPreflight.offline_query_boundary_policy.query_execution_allowed_in_full_gate -ne $false -or
    $selectedJplHorizonsPayloadMaterializationPreflight.offline_query_boundary_policy.sample_set_scope -ne "validation-query-snapshot-set") {
    throw "M10 selected JPL Horizons preflight must keep JPL query execution out of this loop and out of full gate"
}

$selectedJplMaterializedPayloadPath = Join-Path $projectPath $selectedJplHorizonsPayloadMaterializationPreflight.selected_source.payload_path
if (-not (Test-Path -LiteralPath $selectedJplMaterializedPayloadPath)) {
    throw "M10 selected JPL Horizons payload must exist after preflight closes"
}
$selectedJplMaterializedPayloadHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $selectedJplMaterializedPayloadPath).Hash.ToLowerInvariant()
if ($selectedJplMaterializedPayloadHash -ne $selectedJplHorizonsPayloadHash) {
    throw "M10 selected JPL Horizons payload hash mismatch after preflight closes: $selectedJplMaterializedPayloadHash"
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
    if ($selectedJplHorizonsPayloadMaterializationPreflight.preflight_checks -notcontains $check) {
        throw "M10 selected JPL Horizons preflight missing check: $check"
    }
}

if ($selectedJplHorizonsPayloadMaterializationPreflight.materialization_allowed_after_preflight.selected_source_payload -ne $true -or
    $selectedJplHorizonsPayloadMaterializationPreflight.materialization_allowed_after_preflight.selected_source_id -ne $jplHorizonsSourceId -or
    $selectedJplHorizonsPayloadMaterializationPreflight.materialization_allowed_after_preflight.other_remaining_source_payloads -ne $false -or
    $selectedJplHorizonsPayloadMaterializationPreflight.materialization_allowed_after_preflight.generated_astronomy_artifacts -ne $false -or
    $selectedJplHorizonsPayloadMaterializationPreflight.materialization_allowed_after_preflight.generated_artifact_hashes -ne $false -or
    $selectedJplHorizonsPayloadMaterializationPreflight.materialization_allowed_after_preflight.draft_manifest_acceptance_change -ne $false -or
    $selectedJplHorizonsPayloadMaterializationPreflight.materialization_allowed_after_preflight.runtime_behavior_change -ne $false -or
    $selectedJplHorizonsPayloadMaterializationPreflight.materialization_allowed_after_preflight.capability_promotion -ne $false) {
    throw "M10 selected JPL Horizons preflight must allow only selected source payload after preflight"
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
    if ($selectedJplHorizonsPayloadMaterializationPreflight.forbidden_in_preflight_stage -notcontains $forbidden) {
        throw "M10 selected JPL Horizons preflight missing forbidden item: $forbidden"
    }
}

if ($selectedJplHorizonsPayloadMaterialization.status -ne "selected_source_payload_materialized") {
    throw "M10 selected JPL Horizons payload materialization evidence must record selected_source_payload_materialized"
}

if ($selectedJplHorizonsPayloadMaterialization.post_iau_remaining_source_payload_strategy_id -ne $postIauRemainingSourcePayloadStrategy.post_iau_remaining_source_payload_strategy_id -or
    $selectedJplHorizonsPayloadMaterialization.selected_jpl_horizons_payload_materialization_preflight_id -ne $selectedJplHorizonsPayloadMaterializationPreflight.selected_source_payload_materialization_preflight_id -or
    $selectedJplHorizonsPayloadMaterialization.source_payload_materialization_policy_id -ne $sourcePayloadMaterializationPolicy.source_payload_materialization_policy_id -or
    $selectedJplHorizonsPayloadMaterialization.source_capture_procedure_id -ne $sourceCaptureProcedure.source_capture_procedure_id -or
    $selectedJplHorizonsPayloadMaterialization.source_snapshot_manifest_id -ne $sourceSnapshotManifest.source_snapshot_manifest_id) {
    throw "M10 selected JPL Horizons materialization evidence must reference active post-IAU strategy, preflight, policy, procedure, and manifest"
}

if ($selectedJplHorizonsPayloadMaterialization.selected_source.source_id -ne $jplHorizonsSourceId -or
    $selectedJplHorizonsPayloadMaterialization.selected_source.payload_kind -ne "validation-query-snapshot-set" -or
    $selectedJplHorizonsPayloadMaterialization.selected_source.payload_path -ne "data/generated/astronomy/source-snapshots/payloads/jpl-horizons-validation-samples.json" -or
    $selectedJplHorizonsPayloadMaterialization.selected_source.payload_status -ne "materialized" -or
    $selectedJplHorizonsPayloadMaterialization.selected_source.hash_algorithm -ne "sha256" -or
    $selectedJplHorizonsPayloadMaterialization.selected_source.sha256 -ne $selectedJplHorizonsPayloadHash) {
    throw "M10 selected JPL Horizons materialization evidence must record selected payload path/status/hash"
}

if ($selectedJplHorizonsPayloadMaterialization.materialized_payload_count -ne 3 -or
    $selectedJplHorizonsPayloadMaterialization.allowed_materialized_sources -notcontains $selectedMaterializedSourceId -or
    $selectedJplHorizonsPayloadMaterialization.allowed_materialized_sources -notcontains $nextRemainingSourceId -or
    $selectedJplHorizonsPayloadMaterialization.allowed_materialized_sources -notcontains $jplHorizonsSourceId -or
    $selectedJplHorizonsPayloadMaterialization.forbidden_materialized_sources -notcontains "gb-t-33661-2017") {
    throw "M10 selected JPL Horizons materialization evidence must allow exactly NAIF, IAU SOFA, and JPL Horizons selected sources"
}

if ($selectedJplHorizonsPayloadMaterialization.source_payload_claim -ne "offline-validation-query-snapshot-boundary-only" -or
    $selectedJplHorizonsPayloadMaterialization.online_query_executed_in_full_gate -ne $false -or
    $selectedJplHorizonsPayloadMaterialization.external_calls_performed -ne $false -or
    $selectedJplHorizonsPayloadMaterialization.response_bodies_materialized -ne $false -or
    $selectedJplHorizonsPayloadMaterialization.generated_artifact_allowed -ne $false -or
    $selectedJplHorizonsPayloadMaterialization.generated_artifact_hash_allowed -ne $false -or
    $selectedJplHorizonsPayloadMaterialization.manifest_acceptance_change_allowed -ne $false -or
    $selectedJplHorizonsPayloadMaterialization.runtime_behavior_change_allowed -ne $false -or
    $selectedJplHorizonsPayloadMaterialization.capability_status -ne "target") {
    throw "M10 selected JPL Horizons materialization evidence must forbid full-gate online query, response-body claims, generated artifacts, acceptance changes, runtime changes, and capability promotion"
}

foreach ($forbidden in @(
    "write gb-t payload file",
    "write generated astronomy artifacts",
    "compute generated artifact hashes",
    "execute online JPL Horizons query in full project gate",
    "mark draft manifest accepted",
    "change calendar-date-query runtime behavior",
    "change chart-create runtime behavior",
    "replace android-date-layer-v1",
    "claim astronomy-engine supported",
    "claim JPL Horizons runtime dependency enabled",
    "claim response bodies captured"
)) {
    if ($selectedJplHorizonsPayloadMaterialization.forbidden_after_materialization -notcontains $forbidden) {
        throw "M10 selected JPL Horizons materialization evidence missing forbidden item: $forbidden"
    }
}

if ($selectedGbTPayloadMaterializationPreflight.status -ne "preflight_only") {
    throw "M10 selected GB/T payload materialization preflight must remain preflight_only"
}

if ($selectedGbTPayloadMaterializationPreflight.selected_jpl_horizons_payload_materialization_id -ne $selectedJplHorizonsPayloadMaterialization.selected_source_payload_materialization_id -or
    $selectedGbTPayloadMaterializationPreflight.source_payload_materialization_policy_id -ne $sourcePayloadMaterializationPolicy.source_payload_materialization_policy_id -or
    $selectedGbTPayloadMaterializationPreflight.source_capture_procedure_id -ne $sourceCaptureProcedure.source_capture_procedure_id -or
    $selectedGbTPayloadMaterializationPreflight.source_snapshot_manifest_id -ne $sourceSnapshotManifest.source_snapshot_manifest_id) {
    throw "M10 selected GB/T preflight must reference active JPL materialization, policy, procedure, and manifest"
}

if ($selectedGbTPayloadMaterializationPreflight.selected_source.source_id -ne $gbtSourceId) {
    throw "M10 selected GB/T preflight must remain scoped to gb-t-33661-2017"
}

$selectedGbTPayload = @($sourcePayloadMaterializationPolicy.planned_payloads | Where-Object { $_.source_id -eq $gbtSourceId })
$selectedGbTProcedure = @($sourceCaptureProcedure.procedures | Where-Object { $_.source_id -eq $gbtSourceId })
$selectedGbTManifest = @($sourceSnapshotManifest.sources | Where-Object { $_.source_id -eq $gbtSourceId })
if ($selectedGbTPayload.Count -ne 1 -or $selectedGbTProcedure.Count -ne 1 -or $selectedGbTManifest.Count -ne 1) {
    throw "M10 selected GB/T source must exist in policy, procedure, and manifest"
}

if ($selectedGbTPayloadMaterializationPreflight.selected_source.payload_kind -ne $selectedGbTPayload[0].payload_kind -or
    $selectedGbTPayloadMaterializationPreflight.selected_source.schema_path -ne $selectedGbTPayload[0].schema_path -or
    $selectedGbTPayloadMaterializationPreflight.selected_source.payload_path -ne $selectedGbTPayload[0].path -or
    $selectedGbTPayloadMaterializationPreflight.selected_source.payload_format -ne $selectedGbTPayload[0].payload_format) {
    throw "M10 selected GB/T preflight source must match payload policy"
}

if ($selectedGbTPayload[0].payload_status -ne "materialized" -or
    $selectedGbTPayload[0].hash_status -ne "computed" -or
    $selectedGbTPayload[0].sha256 -ne $selectedGbTPayloadHash -or
    $selectedGbTProcedure[0].capture_status -ne "completed_for_rule_reference_boundary" -or
    $selectedGbTProcedure[0].materialization_status -ne "rule_reference_payload_materialized" -or
    $selectedGbTProcedure[0].hash_status -ne "computed" -or
    $selectedGbTProcedure[0].sha256 -ne $selectedGbTPayloadHash -or
    $selectedGbTManifest[0].local_materialization_status -ne "rule_reference_payload_materialized" -or
    $selectedGbTManifest[0].source_payload_hash.value -ne $selectedGbTPayloadHash) {
    throw "M10 selected GB/T payload/procedure/manifest must record LOOP-054 materialized/computed/hash"
}

$selectedGbTSchemaPath = Join-Path $projectPath $selectedGbTPayloadMaterializationPreflight.selected_source.schema_path
if (-not (Test-Path -LiteralPath $selectedGbTSchemaPath)) {
    throw "M10 selected GB/T schema is missing"
}
$selectedGbTSchema = Get-Content -LiteralPath $selectedGbTSchemaPath -Encoding UTF8 -Raw | ConvertFrom-Json
if ($selectedGbTSchema.status -ne "schema_only" -or
    $selectedGbTSchema.source_id -ne $gbtSourceId -or
    $selectedGbTSchema.payload_kind -ne "calendar-rule-reference") {
    throw "M10 selected GB/T schema must remain schema_only and match payload kind"
}

foreach ($field in $selectedGbTSchema.required_fields) {
    if ($selectedGbTPayloadMaterializationPreflight.rule_reference_boundary_policy.required_payload_fields -notcontains $field) {
        throw "M10 selected GB/T preflight missing payload field: $field"
    }
}

foreach ($field in $selectedGbTSchema.required_rule_scope_fields) {
    if ($selectedGbTPayloadMaterializationPreflight.rule_reference_boundary_policy.required_rule_scope_fields -notcontains $field) {
        throw "M10 selected GB/T preflight missing rule scope field: $field"
    }
}

if ($selectedGbTPayloadMaterializationPreflight.payload_directory_policy.path -ne $sourcePayloadMaterializationPolicy.payload_directory.path -or
    $selectedGbTPayloadMaterializationPreflight.payload_directory_policy.current_status -ne "exists_selected_source_only" -or
    $selectedGbTPayloadMaterializationPreflight.payload_directory_policy.existing_materialized_source_count -ne 3 -or
    $selectedGbTPayloadMaterializationPreflight.payload_directory_policy.create_allowed_in_this_loop -ne $false -or
    $selectedGbTPayloadMaterializationPreflight.payload_directory_policy.next_loop_write_scope -ne "selected_source_only") {
    throw "M10 selected GB/T preflight must preserve selected-source-only payload directory policy"
}

if ($selectedGbTPayloadMaterializationPreflight.selected_payload_write_policy.write_allowed_in_this_loop -ne $false -or
    $selectedGbTPayloadMaterializationPreflight.selected_payload_write_policy.next_loop_write_scope -ne "selected_source_only" -or
    $selectedGbTPayloadMaterializationPreflight.selected_payload_write_policy.canonical_json_required -ne $true -or
    $selectedGbTPayloadMaterializationPreflight.selected_payload_write_policy.allowed_payload_claim -ne "calendar-rule-reference-boundary-only") {
    throw "M10 selected GB/T preflight must keep writes blocked this loop and source-only next loop"
}

if ($selectedGbTPayloadMaterializationPreflight.selected_payload_hash_policy.hash_algorithm -ne "sha256" -or
    $selectedGbTPayloadMaterializationPreflight.selected_payload_hash_policy.hash_allowed_in_this_loop -ne $false -or
    $selectedGbTPayloadMaterializationPreflight.selected_payload_hash_policy.next_loop_hash_scope -ne "selected_source_payload_only") {
    throw "M10 selected GB/T preflight must keep hashes blocked this loop and scoped next loop"
}

if ($selectedGbTPayloadMaterializationPreflight.rule_reference_boundary_policy.full_gate_network_policy -ne "no_external_calls" -or
    $selectedGbTPayloadMaterializationPreflight.rule_reference_boundary_policy.source_reference_capture_allowed_in_this_loop -ne $false -or
    $selectedGbTPayloadMaterializationPreflight.rule_reference_boundary_policy.payload_materialization_allowed_in_this_loop -ne $false -or
    $selectedGbTPayloadMaterializationPreflight.rule_reference_boundary_policy.sample_set_scope -ne "calendar-rule-reference") {
    throw "M10 selected GB/T preflight must keep rule-reference capture and payload materialization outside this loop"
}

$selectedGbTPayloadPath = Join-Path $projectPath $selectedGbTPayloadMaterializationPreflight.selected_source.payload_path
if (-not (Test-Path -LiteralPath $selectedGbTPayloadPath)) {
    throw "M10 selected GB/T payload must exist after LOOP-054"
}
$actualSelectedGbTPayloadHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $selectedGbTPayloadPath).Hash.ToLowerInvariant()
if ($actualSelectedGbTPayloadHash -ne $selectedGbTPayloadHash) {
    throw "M10 selected GB/T payload hash mismatch after LOOP-054: $actualSelectedGbTPayloadHash"
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
    if ($selectedGbTPayloadMaterializationPreflight.preflight_checks -notcontains $check) {
        throw "M10 selected GB/T preflight missing check: $check"
    }
}

if ($selectedGbTPayloadMaterializationPreflight.materialization_allowed_after_preflight.selected_source_payload -ne $true -or
    $selectedGbTPayloadMaterializationPreflight.materialization_allowed_after_preflight.selected_source_id -ne $gbtSourceId -or
    $selectedGbTPayloadMaterializationPreflight.materialization_allowed_after_preflight.other_remaining_source_payloads -ne $false -or
    $selectedGbTPayloadMaterializationPreflight.materialization_allowed_after_preflight.generated_astronomy_artifacts -ne $false -or
    $selectedGbTPayloadMaterializationPreflight.materialization_allowed_after_preflight.generated_artifact_hashes -ne $false -or
    $selectedGbTPayloadMaterializationPreflight.materialization_allowed_after_preflight.draft_manifest_acceptance_change -ne $false -or
    $selectedGbTPayloadMaterializationPreflight.materialization_allowed_after_preflight.runtime_behavior_change -ne $false -or
    $selectedGbTPayloadMaterializationPreflight.materialization_allowed_after_preflight.android_baseline_replacement -ne $false -or
    $selectedGbTPayloadMaterializationPreflight.materialization_allowed_after_preflight.capability_promotion -ne $false) {
    throw "M10 selected GB/T preflight must allow only selected source payload after preflight"
}

if ($selectedGbTPayloadMaterialization.status -ne "selected_source_payload_materialized" -or
    $selectedGbTPayloadMaterialization.selected_gb_t_payload_materialization_preflight_id -ne $selectedGbTPayloadMaterializationPreflight.selected_source_payload_materialization_preflight_id -or
    $selectedGbTPayloadMaterialization.source_payload_materialization_policy_id -ne $sourcePayloadMaterializationPolicy.source_payload_materialization_policy_id -or
    $selectedGbTPayloadMaterialization.source_capture_procedure_id -ne $sourceCaptureProcedure.source_capture_procedure_id -or
    $selectedGbTPayloadMaterialization.source_snapshot_manifest_id -ne $sourceSnapshotManifest.source_snapshot_manifest_id) {
    throw "M10 selected GB/T materialization evidence must reference active preflight, policy, procedure, and manifest"
}

if ($selectedGbTPayloadMaterialization.selected_source.source_id -ne $gbtSourceId -or
    $selectedGbTPayloadMaterialization.selected_source.payload_kind -ne "calendar-rule-reference" -or
    $selectedGbTPayloadMaterialization.selected_source.payload_path -ne "data/generated/astronomy/source-snapshots/payloads/gb-t-33661-2017-rule-reference.json" -or
    $selectedGbTPayloadMaterialization.selected_source.payload_status -ne "materialized" -or
    $selectedGbTPayloadMaterialization.selected_source.hash_algorithm -ne "sha256" -or
    $selectedGbTPayloadMaterialization.selected_source.sha256 -ne $selectedGbTPayloadHash) {
    throw "M10 selected GB/T materialization evidence must record selected payload path/status/hash"
}

if ($selectedGbTPayloadMaterialization.materialized_payload_count -ne 4 -or
    $selectedGbTPayloadMaterialization.allowed_materialized_sources -notcontains $selectedMaterializedSourceId -or
    $selectedGbTPayloadMaterialization.allowed_materialized_sources -notcontains $nextRemainingSourceId -or
    $selectedGbTPayloadMaterialization.allowed_materialized_sources -notcontains $jplHorizonsSourceId -or
    $selectedGbTPayloadMaterialization.allowed_materialized_sources -notcontains $gbtSourceId) {
    throw "M10 selected GB/T materialization evidence must allow exactly the four selected source-boundary payloads"
}

if ($selectedGbTPayloadMaterialization.source_payload_claim -ne "calendar-rule-reference-boundary-only" -or
    $selectedGbTPayloadMaterialization.standard_text_copied -ne $false -or
    $selectedGbTPayloadMaterialization.external_calls_performed -ne $false -or
    $selectedGbTPayloadMaterialization.generated_artifact_allowed -ne $false -or
    $selectedGbTPayloadMaterialization.generated_artifact_hash_allowed -ne $false -or
    $selectedGbTPayloadMaterialization.manifest_acceptance_change_allowed -ne $false -or
    $selectedGbTPayloadMaterialization.runtime_behavior_change_allowed -ne $false -or
    $selectedGbTPayloadMaterialization.android_baseline_replacement_allowed -ne $false -or
    $selectedGbTPayloadMaterialization.capability_status -ne "target") {
    throw "M10 selected GB/T materialization evidence must forbid standard-text copy, external calls, generated artifacts, acceptance changes, runtime changes, Android replacement, and capability promotion"
}

foreach ($forbidden in @(
    "copy GB/T standard text into repository",
    "treat GB/T rule-reference boundary as implemented calendar algorithm",
    "write generated astronomy artifacts",
    "compute generated artifact hashes",
    "perform external API call in full project gate",
    "mark draft manifest accepted",
    "change calendar-date-query runtime behavior",
    "change chart-create runtime behavior",
    "replace android-date-layer-v1",
    "claim astronomy-engine supported"
)) {
    if ($selectedGbTPayloadMaterialization.forbidden_after_materialization -notcontains $forbidden) {
        throw "M10 selected GB/T materialization evidence missing forbidden item: $forbidden"
    }
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
    if ($selectedGbTPayloadMaterializationPreflight.forbidden_in_preflight_stage -notcontains $forbidden) {
        throw "M10 selected GB/T preflight missing forbidden item: $forbidden"
    }
}

foreach ($required in @(
    "all selected source ids represented",
    "local snapshot or pinned routine version recorded for every represented source",
    "provenance recorded for every represented source",
    "runtime_dependency false for every represented source",
    "output_claim_allowed false for every represented source",
    "no external API call in full project gate"
)) {
    if ($sourceSnapshotManifestSchema.required_acceptance_requirements -notcontains $required) {
        throw "M10 source snapshot manifest schema missing acceptance requirement: $required"
    }
}

foreach ($forbidden in @(
    "write source snapshot files",
    "write generated astronomy artifacts",
    "compute generated artifact hashes",
    "mark draft manifest accepted",
    "claim astronomy-engine supported"
)) {
    if ($sourceSnapshotManifestPlan.forbidden_in_manifest_materialization_stage -notcontains $forbidden) {
        throw "M10 source snapshot manifest plan missing forbidden item: $forbidden"
    }
}

if (-not (Test-Path -LiteralPath $sourceSnapshotManifestPath)) {
    throw "M10 source snapshot manifest must exist during metadata-only materialization stage"
}

if ($artifactWriterPlan.status -ne "dry_run_only") {
    throw "M9 artifact writer plan must remain dry_run_only"
}

if ($artifactWriterPlan.write_mode -ne "no_write_preview") {
    throw "M9 artifact writer plan must remain no_write_preview"
}

if ($artifactWriterPlan.hash_algorithm -ne "sha256") {
    throw "M9 artifact writer plan must use sha256"
}

if ($artifactWriterPlan.generator_contract_id -ne $generatorContract.generator_contract_id) {
    throw "M9 artifact writer plan must reference active generator contract"
}

if ($artifactWriterPlan.source_adapter_contract_id -ne $sourceAdapterContract.source_adapter_contract_id) {
    throw "M9 artifact writer plan must reference active source adapter contract"
}

if (@($artifactWriterPlan.planned_artifacts).Count -ne @($generatorContract.planned_outputs).Count) {
    throw "M9 artifact writer planned count must match generator contract"
}

foreach ($artifact in $artifactWriterPlan.planned_artifacts) {
    if ($artifact.hash_status -ne "not_computed" -or $artifact.write_status -ne "not_written") {
        throw "M9 artifact writer artifacts must remain not_computed/not_written: $($artifact.path)"
    }
    $match = @($generatorContract.planned_outputs | Where-Object { $_.path -eq $artifact.path -and $_.kind -eq $artifact.kind })
    if ($match.Count -ne 1) {
        throw "M9 artifact writer artifact must match generator contract: $($artifact.path)"
    }
}

foreach ($forbidden in @(
    "create output directory",
    "write artifact file",
    "compute hash for nonexistent artifact",
    "mark manifest artifact_hashes present",
    "mark manifest accepted",
    "claim generated rows exist"
)) {
    if ($artifactWriterPlan.forbidden_in_dry_run -notcontains $forbidden) {
        throw "M9 artifact writer plan missing forbidden item: $forbidden"
    }
}

if ($comparisonRunnerPlan.status -ne "dry_run_only") {
    throw "M9 comparison runner plan must remain dry_run_only"
}

if ($comparisonRunnerPlan.comparison_schema_id -ne $comparisonSchema.schema_id) {
    throw "M9 comparison runner plan must reference active comparison schema"
}

if ($comparisonRunnerPlan.artifact_writer_plan_id -ne $artifactWriterPlan.artifact_writer_plan_id) {
    throw "M9 comparison runner plan must reference active artifact writer plan"
}

if ($comparisonRunnerPlan.android_algorithm_version -ne "android-date-layer-v1") {
    throw "M9 comparison runner plan must bind android-date-layer-v1"
}

if ($comparisonRunnerPlan.dry_run_result_policy.rows_compared -ne 0 -or $comparisonRunnerPlan.dry_run_result_policy.difference_rows -ne 0) {
    throw "M9 comparison runner dry-run policy must keep zero rows"
}

foreach ($binding in @(
    "android_algorithm_version",
    "android_ruleset_id",
    "manifest_id",
    "source_policy_id",
    "generated_range",
    "difference_taxonomy"
)) {
    if ($comparisonRunnerPlan.required_bindings -notcontains $binding) {
        throw "M9 comparison runner plan missing binding: $binding"
    }
}

foreach ($forbidden in @(
    "read generated astronomy rows",
    "write comparison artifact",
    "classify real differences",
    "mark comparison report completed",
    "replace android-date-layer-v1",
    "claim astronomy-engine supported"
)) {
    if ($comparisonRunnerPlan.forbidden_in_dry_run -notcontains $forbidden) {
        throw "M9 comparison runner plan missing forbidden item: $forbidden"
    }
}

if ($generationPlan.source_policy_id -ne $sourcePolicy.source_policy_id) {
    throw "Generation plan must reference the active astronomy source policy"
}

if ($generationPlan.manifest_id -ne $draftManifest.manifest_id) {
    throw "Generation plan must reference the active draft manifest"
}

if ($generationPlan.intended_command.script_status -ne "dry_run_only") {
    throw "Generation plan must declare only a dry-run script"
}

foreach ($artifact in $generationPlan.planned_artifacts) {
    if ($artifact.status -ne "not_generated") {
        throw "Planned artifact must remain not_generated: $($artifact.path)"
    }
    if (-not $artifact.required_hash) {
        throw "Planned artifact must require a hash: $($artifact.path)"
    }
}

foreach ($claim in @(
    "astronomy-engine supported",
    "android baseline replaced",
    "wider date range supported",
    "true solar time supported",
    "timezone history supported"
)) {
    if ($generationPlan.forbidden_runtime_claims -notcontains $claim) {
        throw "Generation plan missing forbidden runtime claim: $claim"
    }
}

foreach ($field in @(
    "comparison_id",
    "manifest_id",
    "source_policy_id",
    "android_algorithm_version",
    "astronomy_engine_version",
    "generated_range",
    "rows_compared",
    "difference_summary",
    "difference_rows",
    "created_at_utc"
)) {
    if ($comparisonSchema.required_fields -notcontains $field) {
        throw "Comparison schema missing required field: $field"
    }
}

foreach ($category in $schema.required_difference_taxonomy) {
    if ($comparisonSchema.allowed_categories -notcontains $category) {
        throw "Comparison schema missing allowed category: $category"
    }
}

if ($goldenPlan.status -ne "planned_not_generated") {
    throw "Golden cases plan must remain planned_not_generated"
}

if ($goldenReadinessPlan.status -ne "readiness_only") {
    throw "M9 golden row readiness plan must remain readiness_only"
}

if ($goldenReadinessPlan.golden_plan_id -ne $goldenPlan.golden_plan_id) {
    throw "M9 golden row readiness plan must reference active golden plan"
}

if ($goldenReadinessPlan.generator_contract_id -ne $generatorContract.generator_contract_id) {
    throw "M9 golden row readiness plan must reference active generator contract"
}

if ($goldenReadinessPlan.artifact_writer_plan_id -ne $artifactWriterPlan.artifact_writer_plan_id) {
    throw "M9 golden row readiness plan must reference active artifact writer plan"
}

if ($goldenReadinessPlan.comparison_runner_plan_id -ne $comparisonRunnerPlan.comparison_runner_plan_id) {
    throw "M9 golden row readiness plan must reference active comparison runner plan"
}

foreach ($required in @(
    "generator contract remains contract_only",
    "artifact writer dry-run remains no_write_preview",
    "comparison runner dry-run remains zero-row",
    "source references are available for every category",
    "expected Android and astronomy values are specified before row acceptance",
    "difference classification is required before manifest acceptance"
)) {
    if ($goldenReadinessPlan.required_before_materialization -notcontains $required) {
        throw "M9 golden row readiness plan missing requirement: $required"
    }
}

foreach ($forbidden in @(
    "generate golden rows",
    "write golden fixture file",
    "mark golden category generated",
    "claim boundary tests passed",
    "mark manifest accepted",
    "claim astronomy-engine supported"
)) {
    if ($goldenReadinessPlan.forbidden_in_readiness_stage -notcontains $forbidden) {
        throw "M9 golden row readiness plan missing forbidden item: $forbidden"
    }
}

if ($precloseoutAudit.status -ne "full_m9_closeout_blocked_preflight_ready") {
    throw "M9 pre-closeout audit must block full M9 closeout while preflight is ready"
}

if ($precloseoutAudit.capability_status -ne "target") {
    throw "M9 pre-closeout audit must keep astronomy-engine target"
}

if ($precloseoutAudit.preflight_closeout_allowed -ne $true -or $precloseoutAudit.full_closeout_allowed -ne $false) {
    throw "M9 pre-closeout audit must allow only preflight closeout"
}

if ($precloseoutAudit.generated_artifacts_accepted -ne $false -or $precloseoutAudit.android_baseline_replacement_allowed -ne $false) {
    throw "M9 pre-closeout audit must not accept generated artifacts or replacement"
}

if ($preflightCloseoutDecision.decision -ne "close_m9_as_preflight_only") {
    throw "M9 preflight closeout decision must close M9 only as preflight"
}

if ($preflightCloseoutDecision.m9_preflight_closed -ne $true -or $preflightCloseoutDecision.m9_full_astronomy_engine_closed -ne $false) {
    throw "M9 preflight closeout decision must not close the full astronomy engine"
}

if ($preflightCloseoutDecision.capability_status -ne "target") {
    throw "M9 preflight closeout decision must keep astronomy-engine target"
}

if ($preflightCloseoutDecision.generated_artifacts_accepted -ne $false -or $preflightCloseoutDecision.android_baseline_replacement_allowed -ne $false) {
    throw "M9 preflight closeout decision must not accept generated artifacts or replacement"
}

if ($preflightCloseoutDecision.calendar_date_query_runtime_change_allowed -ne $false -or $preflightCloseoutDecision.chart_create_runtime_change_allowed -ne $false) {
    throw "M9 preflight closeout decision must not allow runtime route behavior changes"
}

if ($preflightCloseoutDecision.next_milestone.id -ne "M10" -or $preflightCloseoutDecision.next_milestone.entry_loop -ne "LOOP-038") {
    throw "M9 preflight closeout decision must route the next work to M10 / LOOP-038"
}

foreach ($blocker in @(
    "generated astronomy artifacts",
    "sha256 artifact hashes",
    "completed Android-vs-astronomy comparison report",
    "generated golden rows",
    "executed replay tests",
    "runtime astronomy integration",
    "replacement ADR"
)) {
    if ($preflightCloseoutDecision.blocking_full_closeout_items -notcontains $blocker) {
        throw "M9 preflight closeout decision missing full-closeout blocker: $blocker"
    }
}

foreach ($forbidden in @(
    "treat M9 preflight closeout as generated-data acceptance",
    "mark astronomy-engine supported",
    "replace android-date-layer-v1",
    "change calendar-date-query default runtime behavior",
    "change chart-create default runtime behavior"
)) {
    if ($preflightCloseoutDecision.forbidden_after_closeout -notcontains $forbidden) {
        throw "M9 preflight closeout decision missing forbidden after-closeout item: $forbidden"
    }
}

foreach ($blockerId in @(
    "generated-astronomy-table",
    "artifact-hashes",
    "android-comparison-report",
    "golden-case-rows",
    "replay-tests",
    "runtime-integration"
)) {
    $match = @($precloseoutAudit.blocking_acceptance_items | Where-Object { $_.id -eq $blockerId })
    if ($match.Count -ne 1) {
        throw "M9 pre-closeout audit missing blocker: $blockerId"
    }
    if ($match[0].status -ne "missing") {
        throw "M9 pre-closeout blocker must remain missing until full evidence exists: $blockerId"
    }
}

foreach ($category in @(
    "1901-2100-boundary",
    "2033-anomaly",
    "lichun-boundary",
    "qingming-boundary",
    "jiazi-day-anchor",
    "near-midnight-solar-lunar-event"
)) {
    $match = @($goldenPlan.required_categories | Where-Object { $_.id -eq $category })
    if ($match.Count -ne 1) {
        throw "Golden cases plan missing category: $category"
    }
    if ($match[0].status -ne "not_generated") {
        throw "Golden category must remain not_generated: $category"
    }
    $readinessMatch = @($goldenReadinessPlan.category_readiness | Where-Object { $_.id -eq $category })
    if ($readinessMatch.Count -ne 1) {
        throw "Golden row readiness plan missing category: $category"
    }
    if ($readinessMatch[0].status -ne "not_generated" -or $readinessMatch[0].readiness_status -ne "blocked_until_generated_rows") {
        throw "Golden row readiness category must remain blocked/not_generated: $category"
    }
}

$dryRunOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $generatorScript -ProjectRoot $projectPath -Manifest "data/generated/astronomy/manifests/astronomy-engine-v0-draft.json" -DryRun
$dryRun = $dryRunOutput | ConvertFrom-Json

if ($dryRun.mode -ne "dry_run_only") {
    throw "Generator dry-run did not report dry_run_only mode"
}

if ($dryRun.generator_contract_id -ne $generatorContract.generator_contract_id -or $dryRun.hash_algorithm -ne "sha256") {
    throw "Generator dry-run must report the active generator contract and sha256 hash algorithm"
}

if ($dryRun.writes_performed -ne $false -or $dryRun.acceptance_status_changed -ne $false) {
    throw "Generator dry-run must not write artifacts or change acceptance status"
}

if ($dryRun.planned_artifact_count -ne @($generationPlan.planned_artifacts).Count) {
    throw "Generator dry-run planned artifact count mismatch"
}

if (@($dryRun.existing_planned_artifacts).Count -ne 0) {
    throw "Generator dry-run found generated artifacts before acceptance"
}

$entryOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $generatorScript -ProjectRoot $projectPath -Manifest "data/generated/astronomy/manifests/astronomy-engine-v0-draft.json" -PrepareImplementation
$entryRun = $entryOutput | ConvertFrom-Json

if ($entryRun.mode -ne "implementation_entry_guarded") {
    throw "M10 generator entry did not report implementation_entry_guarded mode"
}

if ($entryRun.dry_run -ne $false) {
    throw "M10 generator entry must be non-dry-run in shape"
}

if ($entryRun.implementation_entry_id -ne $generatorImplementationEntry.implementation_entry_id) {
    throw "M10 generator entry must report the active implementation entry id"
}

if ($entryRun.capability_status -ne "target") {
    throw "M10 generator entry must keep astronomy-engine target"
}

if ($entryRun.source_snapshot_manifest_exists -ne $true -or $entryRun.generation_blocked -ne $true) {
    throw "M10 generator entry must remain blocked while only metadata source manifest exists"
}

if ($entryRun.writes_performed -ne $false -or
    $entryRun.hashes_computed -ne 0 -or
    $entryRun.acceptance_status_changed -ne $false -or
    $entryRun.runtime_behavior_changed -ne $false) {
    throw "M10 generator entry must not write files, compute hashes, change acceptance, or change runtime behavior"
}

$sourceSnapshotDryRunOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $sourceSnapshotManifestDryRunScript -ProjectRoot $projectPath
$sourceSnapshotDryRun = $sourceSnapshotDryRunOutput | ConvertFrom-Json

if ($sourceSnapshotDryRun.mode -ne "source_snapshot_manifest_selected_payload_dry_run") {
    throw "M10 source snapshot manifest dry-run did not report selected payload mode"
}

if ($sourceSnapshotDryRun.schema_id -ne $sourceSnapshotManifestSchema.schema_id -or
    $sourceSnapshotDryRun.plan_id -ne $sourceSnapshotManifestPlan.source_snapshot_manifest_plan_id -or
    $sourceSnapshotDryRun.manifest_id -ne $sourceSnapshotManifest.source_snapshot_manifest_id) {
    throw "M10 source snapshot manifest dry-run must report active schema, plan, and manifest"
}

if ($sourceSnapshotDryRun.manifest_exists -ne $true -or $sourceSnapshotDryRun.manifest_status -ne "selected_source_payload_materialized") {
    throw "M10 source snapshot manifest dry-run must keep selected payload manifest present"
}

if ($sourceSnapshotDryRun.planned_source_count -ne @($sourceSnapshotManifestPlan.planned_sources).Count -or
    $sourceSnapshotDryRun.manifest_source_count -ne @($sourceSnapshotManifest.sources).Count) {
    throw "M10 source snapshot manifest dry-run source count mismatch"
}

if ($sourceSnapshotDryRun.writes_performed -ne $false -or
    $sourceSnapshotDryRun.source_snapshots_materialized -ne 4 -or
    $sourceSnapshotDryRun.source_payload_hashes_computed -ne 4 -or
    $sourceSnapshotDryRun.generated_artifacts_written -ne 0 -or
    $sourceSnapshotDryRun.acceptance_status_changed -ne $false -or
    $sourceSnapshotDryRun.runtime_behavior_changed -ne $false) {
    throw "M10 source snapshot manifest dry-run must inspect exactly four selected source payloads/hashes and avoid generated artifacts, acceptance changes, or runtime changes"
}

$sourcePayloadDryRunOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $sourcePayloadMaterializationDryRunScript -ProjectRoot $projectPath
$sourcePayloadDryRun = $sourcePayloadDryRunOutput | ConvertFrom-Json

if ($sourcePayloadDryRun.mode -ne "source_payload_materialization_selected_sources_dry_run") {
    throw "M10 source payload materialization dry-run did not report selected sources mode"
}

if ($sourcePayloadDryRun.policy_id -ne $sourcePayloadMaterializationPolicy.source_payload_materialization_policy_id -or
    $sourcePayloadDryRun.manifest_id -ne $sourceSnapshotManifest.source_snapshot_manifest_id) {
    throw "M10 source payload materialization dry-run must report active policy and manifest"
}

if ($sourcePayloadDryRun.payload_directory_exists -ne $true -or
    $sourcePayloadDryRun.selected_source_ids -notcontains $selectedMaterializedSourceId -or
    $sourcePayloadDryRun.selected_source_ids -notcontains $nextRemainingSourceId -or
    $sourcePayloadDryRun.selected_source_ids -notcontains $jplHorizonsSourceId -or
    $sourcePayloadDryRun.selected_source_ids -notcontains $gbtSourceId) {
    throw "M10 source payload materialization dry-run must inspect selected source payload directory"
}

if ($sourcePayloadDryRun.planned_payload_count -ne @($sourcePayloadMaterializationPolicy.planned_payloads).Count -or
    $sourcePayloadDryRun.schema_file_count -ne @($sourcePayloadMaterializationPolicy.planned_payloads).Count -or
    $sourcePayloadDryRun.manifest_source_count -ne @($sourceSnapshotManifest.sources).Count) {
    throw "M10 source payload materialization dry-run payload/source count mismatch"
}

if (@($sourcePayloadDryRun.existing_payload_files).Count -ne 4 -or
    @($sourcePayloadDryRun.materialized_payload_files).Count -ne 4) {
    throw "M10 source payload materialization dry-run must find exactly four selected payload files"
}

if ($sourcePayloadDryRun.writes_performed -ne $false -or
    $sourcePayloadDryRun.source_payloads_materialized -ne 4 -or
    $sourcePayloadDryRun.payload_hashes_computed -ne 4 -or
    $sourcePayloadDryRun.generated_artifacts_written -ne 0 -or
    $sourcePayloadDryRun.generated_artifact_hashes_computed -ne 0 -or
    $sourcePayloadDryRun.acceptance_status_changed -ne $false -or
    $sourcePayloadDryRun.runtime_behavior_changed -ne $false) {
    throw "M10 source payload materialization dry-run must inspect exactly four selected payloads/hashes and avoid generated artifacts, acceptance changes, or runtime behavior"
}

$sourceCaptureDryRunOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $sourceCaptureProcedureDryRunScript -ProjectRoot $projectPath
$sourceCaptureDryRun = $sourceCaptureDryRunOutput | ConvertFrom-Json

if ($sourceCaptureDryRun.mode -ne "source_capture_procedure_selected_payloads_dry_run") {
    throw "M10 source capture procedure dry-run did not report selected payloads mode"
}

if ($sourceCaptureDryRun.procedure_id -ne $sourceCaptureProcedure.source_capture_procedure_id -or
    $sourceCaptureDryRun.policy_id -ne $sourcePayloadMaterializationPolicy.source_payload_materialization_policy_id -or
    $sourceCaptureDryRun.manifest_id -ne $sourceSnapshotManifest.source_snapshot_manifest_id) {
    throw "M10 source capture procedure dry-run must report active procedure, policy, and manifest"
}

if ($sourceCaptureDryRun.payload_directory_exists -ne $true -or
    $sourceCaptureDryRun.selected_source_ids -notcontains $selectedMaterializedSourceId -or
    $sourceCaptureDryRun.selected_source_ids -notcontains $nextRemainingSourceId -or
    $sourceCaptureDryRun.selected_source_ids -notcontains $jplHorizonsSourceId -or
    $sourceCaptureDryRun.selected_source_ids -notcontains $gbtSourceId) {
    throw "M10 source capture procedure dry-run must inspect selected payload directory"
}

if ($sourceCaptureDryRun.planned_source_count -ne @($sourcePayloadMaterializationPolicy.planned_payloads).Count -or
    $sourceCaptureDryRun.procedure_source_count -ne @($sourceCaptureProcedure.procedures).Count -or
    $sourceCaptureDryRun.schema_file_count -ne @($sourcePayloadMaterializationPolicy.planned_payloads).Count) {
    throw "M10 source capture procedure dry-run source/schema count mismatch"
}

if (@($sourceCaptureDryRun.existing_payload_files).Count -ne 4 -or
    @($sourceCaptureDryRun.materialized_payload_files).Count -ne 4) {
    throw "M10 source capture procedure dry-run must find exactly four selected payload files"
}

if ($sourceCaptureDryRun.writes_performed -ne $false -or
    $sourceCaptureDryRun.source_payloads_materialized -ne 4 -or
    $sourceCaptureDryRun.payload_hashes_computed -ne 4 -or
    $sourceCaptureDryRun.external_calls_performed -ne $false -or
    $sourceCaptureDryRun.generated_artifacts_written -ne 0 -or
    $sourceCaptureDryRun.generated_artifact_hashes_computed -ne 0 -or
    $sourceCaptureDryRun.acceptance_status_changed -ne $false -or
    $sourceCaptureDryRun.runtime_behavior_changed -ne $false) {
    throw "M10 source capture procedure dry-run must inspect exactly four selected payloads/hashes and avoid external calls, generated artifacts, acceptance changes, or runtime behavior"
}

$sourcePayloadDecisionDryRunOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $sourcePayloadMaterializationDecisionDryRunScript -ProjectRoot $projectPath
$sourcePayloadDecisionDryRun = $sourcePayloadDecisionDryRunOutput | ConvertFrom-Json

if ($sourcePayloadDecisionDryRun.mode -ne "source_payload_materialization_decision_selected_payload_dry_run") {
    throw "M10 source payload materialization decision dry-run did not report selected payload decision mode"
}

if ($sourcePayloadDecisionDryRun.decision_id -ne $sourcePayloadMaterializationDecision.source_payload_materialization_decision_id -or
    $sourcePayloadDecisionDryRun.policy_id -ne $sourcePayloadMaterializationPolicy.source_payload_materialization_policy_id -or
    $sourcePayloadDecisionDryRun.procedure_id -ne $sourceCaptureProcedure.source_capture_procedure_id -or
    $sourcePayloadDecisionDryRun.manifest_id -ne $sourceSnapshotManifest.source_snapshot_manifest_id) {
    throw "M10 source payload materialization decision dry-run must report active decision, policy, procedure, and manifest"
}

if ($sourcePayloadDecisionDryRun.selected_source_id -ne $sourcePayloadMaterializationDecision.selected_source.source_id -or
    $sourcePayloadDecisionDryRun.selected_payload_kind -ne $sourcePayloadMaterializationDecision.selected_source.payload_kind) {
    throw "M10 source payload materialization decision dry-run must report selected source and payload kind"
}

if ($sourcePayloadDecisionDryRun.payload_directory_exists -ne $true -or
    $sourcePayloadDecisionDryRun.selected_payload_exists -ne $true) {
    throw "M10 source payload materialization decision dry-run must inspect selected payload existence"
}

if ($sourcePayloadDecisionDryRun.planned_payload_count -ne @($sourcePayloadMaterializationPolicy.planned_payloads).Count -or
    @($sourcePayloadDecisionDryRun.existing_payload_files).Count -ne 4) {
    throw "M10 source payload materialization decision dry-run payload count/state mismatch"
}

if ($sourcePayloadDecisionDryRun.writes_performed -ne $false -or
    $sourcePayloadDecisionDryRun.source_payloads_materialized -ne 4 -or
    $sourcePayloadDecisionDryRun.payload_hashes_computed -ne 4 -or
    $sourcePayloadDecisionDryRun.external_calls_performed -ne $false -or
    $sourcePayloadDecisionDryRun.generated_artifacts_written -ne 0 -or
    $sourcePayloadDecisionDryRun.generated_artifact_hashes_computed -ne 0 -or
    $sourcePayloadDecisionDryRun.acceptance_status_changed -ne $false -or
    $sourcePayloadDecisionDryRun.runtime_behavior_changed -ne $false) {
    throw "M10 source payload materialization decision dry-run must inspect exactly four selected payloads/hashes and avoid external calls, generated artifacts, acceptance changes, or runtime behavior"
}

$selectedSourcePreflightDryRunOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $selectedSourcePayloadMaterializationPreflightDryRunScript -ProjectRoot $projectPath
$selectedSourcePreflightDryRun = $selectedSourcePreflightDryRunOutput | ConvertFrom-Json

if ($selectedSourcePreflightDryRun.mode -ne "selected_source_payload_materialization_preflight_closed_dry_run") {
    throw "M10 selected source payload materialization preflight dry-run did not report closed preflight mode"
}

if ($selectedSourcePreflightDryRun.preflight_id -ne $selectedSourcePayloadMaterializationPreflight.selected_source_payload_materialization_preflight_id -or
    $selectedSourcePreflightDryRun.decision_id -ne $sourcePayloadMaterializationDecision.source_payload_materialization_decision_id) {
    throw "M10 selected source payload materialization preflight dry-run must report active preflight and decision"
}

if ($selectedSourcePreflightDryRun.selected_source_id -ne "naif-cspice" -or
    $selectedSourcePreflightDryRun.selected_payload_kind -ne "offline-kernel-toolkit-boundary") {
    throw "M10 selected source payload materialization preflight dry-run must remain scoped to naif-cspice"
}

if ($selectedSourcePreflightDryRun.payload_directory_exists -ne $true -or
    $selectedSourcePreflightDryRun.selected_payload_exists -ne $true -or
    @($selectedSourcePreflightDryRun.existing_payload_files).Count -ne 4) {
    throw "M10 selected source payload materialization preflight dry-run must inspect selected payload existence"
}

if ($selectedSourcePreflightDryRun.next_loop_create_scope -ne "selected_source_only" -or
    $selectedSourcePreflightDryRun.next_loop_write_scope -ne "selected_source_only" -or
    $selectedSourcePreflightDryRun.next_loop_hash_scope -ne "selected_source_payload_only") {
    throw "M10 selected source payload materialization preflight dry-run must keep next-loop scope selected-source only"
}

if ($selectedSourcePreflightDryRun.writes_performed -ne $false -or
    $selectedSourcePreflightDryRun.source_payloads_materialized -ne 4 -or
    $selectedSourcePreflightDryRun.payload_hashes_computed -ne 4 -or
    $selectedSourcePreflightDryRun.external_calls_performed -ne $false -or
    $selectedSourcePreflightDryRun.generated_artifacts_written -ne 0 -or
    $selectedSourcePreflightDryRun.generated_artifact_hashes_computed -ne 0 -or
    $selectedSourcePreflightDryRun.acceptance_status_changed -ne $false -or
    $selectedSourcePreflightDryRun.runtime_behavior_changed -ne $false) {
    throw "M10 selected source payload materialization preflight dry-run must inspect exactly four selected payloads/hashes and avoid external calls, generated artifacts, acceptance changes, or runtime behavior"
}

$remainingSourcePayloadStrategyDryRunOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $remainingSourcePayloadStrategyDryRunScript -ProjectRoot $projectPath
$remainingSourcePayloadStrategyDryRun = $remainingSourcePayloadStrategyDryRunOutput | ConvertFrom-Json

if ($remainingSourcePayloadStrategyDryRun.mode -ne "remaining_source_payload_strategy_closed_dry_run") {
    throw "M10 remaining source payload strategy dry-run did not report strategy mode"
}

if ($remainingSourcePayloadStrategyDryRun.strategy_id -ne $remainingSourcePayloadStrategy.remaining_source_payload_strategy_id -or
    $remainingSourcePayloadStrategyDryRun.selected_materialization_id -ne $selectedSourcePayloadMaterialization.selected_source_payload_materialization_id) {
    throw "M10 remaining source payload strategy dry-run must report active strategy and materialization ids"
}

if ($remainingSourcePayloadStrategyDryRun.iau_sofa_materialization_id -ne $selectedIauSofaPayloadMaterialization.selected_source_payload_materialization_id -or
    $remainingSourcePayloadStrategyDryRun.jpl_horizons_materialization_id -ne $selectedJplHorizonsPayloadMaterialization.selected_source_payload_materialization_id -or
    $remainingSourcePayloadStrategyDryRun.gbt_materialization_id -ne $selectedGbTPayloadMaterialization.selected_source_payload_materialization_id) {
    throw "M10 remaining source payload strategy dry-run must report IAU SOFA, JPL Horizons, and GB/T materialization ids after closure"
}

if ($remainingSourcePayloadStrategyDryRun.materialized_source_count -ne 4 -or
    $remainingSourcePayloadStrategyDryRun.materialized_source_ids -notcontains $selectedMaterializedSourceId -or
    $remainingSourcePayloadStrategyDryRun.materialized_source_ids -notcontains $nextRemainingSourceId -or
    $remainingSourcePayloadStrategyDryRun.materialized_source_ids -notcontains $jplHorizonsSourceId -or
    $remainingSourcePayloadStrategyDryRun.materialized_source_ids -notcontains $gbtSourceId) {
    throw "M10 remaining source payload strategy dry-run must report all four source payloads as materialized"
}

if ($remainingSourcePayloadStrategyDryRun.remaining_source_count -ne 0 -or
    $remainingSourcePayloadStrategyDryRun.next_selected_source_id -ne "jpl-horizons-api" -or
    $remainingSourcePayloadStrategyDryRun.next_selected_payload_kind -ne "validation-query-snapshot-set" -or
    $remainingSourcePayloadStrategyDryRun.next_loop_action -ne "remaining_source_payload_strategy_after_iau_sofa") {
    throw "M10 remaining source payload strategy dry-run must route next work to post-IAU remaining-source strategy"
}

if (@($remainingSourcePayloadStrategyDryRun.existing_payload_files).Count -ne 4) {
    throw "M10 remaining source payload strategy dry-run must observe exactly four existing payload files"
}

if ($remainingSourcePayloadStrategyDryRun.writes_performed -ne $false -or
    $remainingSourcePayloadStrategyDryRun.source_payloads_materialized -ne 4 -or
    $remainingSourcePayloadStrategyDryRun.new_source_payloads_written -ne 3 -or
    $remainingSourcePayloadStrategyDryRun.new_source_payload_hashes_computed -ne 3 -or
    $remainingSourcePayloadStrategyDryRun.external_calls_performed -ne $false -or
    $remainingSourcePayloadStrategyDryRun.generated_artifacts_written -ne 0 -or
    $remainingSourcePayloadStrategyDryRun.generated_artifact_hashes_computed -ne 0 -or
    $remainingSourcePayloadStrategyDryRun.acceptance_status_changed -ne $false -or
    $remainingSourcePayloadStrategyDryRun.runtime_behavior_changed -ne $false) {
    throw "M10 remaining source payload strategy dry-run must not write new payloads, compute new hashes, call external sources, write generated artifacts, change acceptance, or change runtime behavior"
}

$selectedIauSofaPreflightDryRunOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $selectedIauSofaPayloadMaterializationPreflightDryRunScript -ProjectRoot $projectPath
$selectedIauSofaPreflightDryRun = $selectedIauSofaPreflightDryRunOutput | ConvertFrom-Json

if ($selectedIauSofaPreflightDryRun.mode -ne "selected_iau_sofa_payload_materialization_preflight_closed_dry_run") {
    throw "M10 selected IAU SOFA payload materialization preflight dry-run did not report closed preflight mode"
}

if ($selectedIauSofaPreflightDryRun.preflight_id -ne $selectedIauSofaPayloadMaterializationPreflight.selected_source_payload_materialization_preflight_id -or
    $selectedIauSofaPreflightDryRun.strategy_id -ne $remainingSourcePayloadStrategy.remaining_source_payload_strategy_id -or
    $selectedIauSofaPreflightDryRun.materialization_id -ne $selectedIauSofaPayloadMaterialization.selected_source_payload_materialization_id) {
    throw "M10 selected IAU SOFA preflight dry-run must report active preflight, strategy, and materialization ids"
}

if ($selectedIauSofaPreflightDryRun.selected_source_id -ne $nextRemainingSourceId -or
    $selectedIauSofaPreflightDryRun.selected_payload_kind -ne "local-routine-version-record" -or
    $selectedIauSofaPreflightDryRun.selected_payload_exists -ne $true) {
    throw "M10 selected IAU SOFA preflight dry-run must keep selected payload present after closure and scoped to IAU SOFA"
}

if ($selectedIauSofaPreflightDryRun.payload_directory_exists -ne $true -or
    $selectedIauSofaPreflightDryRun.existing_payload_count -ne 4 -or
    @($selectedIauSofaPreflightDryRun.existing_payload_files).Count -ne 4) {
    throw "M10 selected IAU SOFA preflight dry-run must observe exactly four existing payload files"
}

if ($selectedIauSofaPreflightDryRun.next_loop_write_scope -ne "selected_source_only" -or
    $selectedIauSofaPreflightDryRun.next_loop_hash_scope -ne "selected_source_payload_only") {
    throw "M10 selected IAU SOFA preflight dry-run must scope next-loop writes and hashes to the selected source only"
}

if ($selectedIauSofaPreflightDryRun.writes_performed -ne $false -or
    $selectedIauSofaPreflightDryRun.source_payloads_materialized -ne 4 -or
    $selectedIauSofaPreflightDryRun.new_source_payloads_written -ne 1 -or
    $selectedIauSofaPreflightDryRun.new_source_payload_hashes_computed -ne 1 -or
    $selectedIauSofaPreflightDryRun.external_calls_performed -ne $false -or
    $selectedIauSofaPreflightDryRun.generated_artifacts_written -ne 0 -or
    $selectedIauSofaPreflightDryRun.generated_artifact_hashes_computed -ne 0 -or
    $selectedIauSofaPreflightDryRun.acceptance_status_changed -ne $false -or
    $selectedIauSofaPreflightDryRun.runtime_behavior_changed -ne $false) {
    throw "M10 selected IAU SOFA preflight dry-run must not write payloads, compute hashes, call external sources, write generated artifacts, change acceptance, or change runtime behavior"
}

$postIauRemainingSourcePayloadStrategyDryRunOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $postIauRemainingSourcePayloadStrategyDryRunScript -ProjectRoot $projectPath
$postIauRemainingSourcePayloadStrategyDryRun = $postIauRemainingSourcePayloadStrategyDryRunOutput | ConvertFrom-Json

if ($postIauRemainingSourcePayloadStrategyDryRun.mode -ne "post_iau_remaining_source_payload_strategy_closed_dry_run") {
    throw "M10 post-IAU remaining source payload strategy dry-run did not report post-IAU strategy mode"
}

if ($postIauRemainingSourcePayloadStrategyDryRun.strategy_id -ne $postIauRemainingSourcePayloadStrategy.post_iau_remaining_source_payload_strategy_id -or
    $postIauRemainingSourcePayloadStrategyDryRun.previous_strategy_id -ne $remainingSourcePayloadStrategy.remaining_source_payload_strategy_id -or
    $postIauRemainingSourcePayloadStrategyDryRun.iau_sofa_materialization_id -ne $selectedIauSofaPayloadMaterialization.selected_source_payload_materialization_id) {
    throw "M10 post-IAU remaining source payload strategy dry-run must report active strategy, previous strategy, and IAU materialization ids"
}

if ($postIauRemainingSourcePayloadStrategyDryRun.materialized_source_count -ne 4 -or
    $postIauRemainingSourcePayloadStrategyDryRun.materialized_source_ids -notcontains $selectedMaterializedSourceId -or
    $postIauRemainingSourcePayloadStrategyDryRun.materialized_source_ids -notcontains $nextRemainingSourceId -or
    $postIauRemainingSourcePayloadStrategyDryRun.materialized_source_ids -notcontains $jplHorizonsSourceId -or
    $postIauRemainingSourcePayloadStrategyDryRun.materialized_source_ids -notcontains $gbtSourceId) {
    throw "M10 post-IAU remaining source payload strategy dry-run must report all four source payloads as materialized"
}

if ($postIauRemainingSourcePayloadStrategyDryRun.remaining_source_count -ne 0 -or
    $postIauRemainingSourcePayloadStrategyDryRun.next_selected_source_id -ne "jpl-horizons-api" -or
    $postIauRemainingSourcePayloadStrategyDryRun.next_selected_payload_kind -ne "validation-query-snapshot-set" -or
    $postIauRemainingSourcePayloadStrategyDryRun.next_loop_action -ne "selected_source_payload_materialized_in_loop_052") {
    throw "M10 post-IAU remaining source payload strategy dry-run must report JPL materialized in LOOP-052"
}

if ($postIauRemainingSourcePayloadStrategyDryRun.existing_payload_count -ne 4 -or
    @($postIauRemainingSourcePayloadStrategyDryRun.existing_payload_files).Count -ne 4) {
    throw "M10 post-IAU remaining source payload strategy dry-run must observe exactly four existing payload files"
}

if ($postIauRemainingSourcePayloadStrategyDryRun.writes_performed -ne $false -or
    $postIauRemainingSourcePayloadStrategyDryRun.source_payloads_materialized -ne 4 -or
    $postIauRemainingSourcePayloadStrategyDryRun.new_source_payloads_written -ne 2 -or
    $postIauRemainingSourcePayloadStrategyDryRun.new_source_payload_hashes_computed -ne 2 -or
    $postIauRemainingSourcePayloadStrategyDryRun.external_calls_performed -ne $false -or
    $postIauRemainingSourcePayloadStrategyDryRun.generated_artifacts_written -ne 0 -or
    $postIauRemainingSourcePayloadStrategyDryRun.generated_artifact_hashes_computed -ne 0 -or
    $postIauRemainingSourcePayloadStrategyDryRun.acceptance_status_changed -ne $false -or
    $postIauRemainingSourcePayloadStrategyDryRun.runtime_behavior_changed -ne $false) {
    throw "M10 post-IAU remaining source payload strategy dry-run must not write payloads, compute new hashes, call external sources, write generated artifacts, change acceptance, or change runtime behavior"
}

$selectedJplHorizonsPreflightDryRunOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $selectedJplHorizonsPayloadMaterializationPreflightDryRunScript -ProjectRoot $projectPath
$selectedJplHorizonsPreflightDryRun = $selectedJplHorizonsPreflightDryRunOutput | ConvertFrom-Json

if ($selectedJplHorizonsPreflightDryRun.mode -ne "selected_jpl_horizons_payload_materialization_preflight_closed_dry_run") {
    throw "M10 selected JPL Horizons payload materialization preflight dry-run did not report JPL preflight mode"
}

if ($selectedJplHorizonsPreflightDryRun.preflight_id -ne $selectedJplHorizonsPayloadMaterializationPreflight.selected_source_payload_materialization_preflight_id -or
    $selectedJplHorizonsPreflightDryRun.strategy_id -ne $postIauRemainingSourcePayloadStrategy.post_iau_remaining_source_payload_strategy_id) {
    throw "M10 selected JPL Horizons preflight dry-run must report active preflight and post-IAU strategy ids"
}

if ($selectedJplHorizonsPreflightDryRun.selected_source_id -ne $jplHorizonsSourceId -or
    $selectedJplHorizonsPreflightDryRun.selected_payload_kind -ne "validation-query-snapshot-set" -or
    $selectedJplHorizonsPreflightDryRun.selected_payload_exists -ne $true) {
    throw "M10 selected JPL Horizons preflight dry-run must keep selected JPL payload present after closure"
}

if ($selectedJplHorizonsPreflightDryRun.payload_directory_exists -ne $true -or
    $selectedJplHorizonsPreflightDryRun.existing_payload_count -ne 4 -or
    @($selectedJplHorizonsPreflightDryRun.existing_payload_files).Count -ne 4) {
    throw "M10 selected JPL Horizons preflight dry-run must observe exactly four existing payload files"
}

if ($selectedJplHorizonsPreflightDryRun.next_loop_write_scope -ne "selected_source_only" -or
    $selectedJplHorizonsPreflightDryRun.next_loop_hash_scope -ne "selected_source_payload_only" -or
    $selectedJplHorizonsPreflightDryRun.query_execution_allowed_in_full_gate -ne $false) {
    throw "M10 selected JPL Horizons preflight dry-run must scope next-loop writes/hashes and forbid full-gate queries"
}

if ($selectedJplHorizonsPreflightDryRun.writes_performed -ne $false -or
    $selectedJplHorizonsPreflightDryRun.source_payloads_materialized -ne 4 -or
    $selectedJplHorizonsPreflightDryRun.new_source_payloads_written -ne 1 -or
    $selectedJplHorizonsPreflightDryRun.new_source_payload_hashes_computed -ne 1 -or
    $selectedJplHorizonsPreflightDryRun.external_calls_performed -ne $false -or
    $selectedJplHorizonsPreflightDryRun.generated_artifacts_written -ne 0 -or
    $selectedJplHorizonsPreflightDryRun.generated_artifact_hashes_computed -ne 0 -or
    $selectedJplHorizonsPreflightDryRun.acceptance_status_changed -ne $false -or
    $selectedJplHorizonsPreflightDryRun.runtime_behavior_changed -ne $false) {
    throw "M10 selected JPL Horizons preflight dry-run must not write payloads, compute hashes, call external sources, write generated artifacts, change acceptance, or change runtime behavior"
}

$selectedGbTPreflightDryRunOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $selectedGbTPayloadMaterializationPreflightDryRunScript -ProjectRoot $projectPath
$selectedGbTPreflightDryRun = $selectedGbTPreflightDryRunOutput | ConvertFrom-Json

if ($selectedGbTPreflightDryRun.mode -ne "selected_gb_t_payload_materialization_preflight_closed_dry_run") {
    throw "M10 selected GB/T payload materialization preflight dry-run did not report closed GB/T preflight mode"
}

if ($selectedGbTPreflightDryRun.preflight_id -ne $selectedGbTPayloadMaterializationPreflight.selected_source_payload_materialization_preflight_id -or
    $selectedGbTPreflightDryRun.jpl_materialization_id -ne $selectedJplHorizonsPayloadMaterialization.selected_source_payload_materialization_id -or
    $selectedGbTPreflightDryRun.materialization_id -ne $selectedGbTPayloadMaterialization.selected_source_payload_materialization_id) {
    throw "M10 selected GB/T preflight dry-run must report active preflight, JPL materialization, and GB/T materialization ids"
}

if ($selectedGbTPreflightDryRun.selected_source_id -ne $gbtSourceId -or
    $selectedGbTPreflightDryRun.selected_payload_kind -ne "calendar-rule-reference" -or
    $selectedGbTPreflightDryRun.selected_payload_exists -ne $true) {
    throw "M10 selected GB/T preflight dry-run must keep selected GB/T payload present after closure"
}

if ($selectedGbTPreflightDryRun.payload_directory_exists -ne $true -or
    $selectedGbTPreflightDryRun.existing_payload_count -ne 4 -or
    @($selectedGbTPreflightDryRun.existing_payload_files).Count -ne 4) {
    throw "M10 selected GB/T preflight dry-run must observe exactly four existing payload files"
}

if ($selectedGbTPreflightDryRun.next_loop_write_scope -ne "selected_source_only" -or
    $selectedGbTPreflightDryRun.next_loop_hash_scope -ne "selected_source_payload_only" -or
    $selectedGbTPreflightDryRun.source_reference_capture_allowed_in_this_loop -ne $false -or
    $selectedGbTPreflightDryRun.payload_materialization_allowed_in_this_loop -ne $false) {
    throw "M10 selected GB/T preflight dry-run must scope next-loop writes/hashes and block this-loop capture/materialization"
}

if ($selectedGbTPreflightDryRun.writes_performed -ne $false -or
    $selectedGbTPreflightDryRun.source_payloads_materialized -ne 4 -or
    $selectedGbTPreflightDryRun.new_source_payloads_written -ne 1 -or
    $selectedGbTPreflightDryRun.new_source_payload_hashes_computed -ne 1 -or
    $selectedGbTPreflightDryRun.external_calls_performed -ne $false -or
    $selectedGbTPreflightDryRun.generated_artifacts_written -ne 0 -or
    $selectedGbTPreflightDryRun.generated_artifact_hashes_computed -ne 0 -or
    $selectedGbTPreflightDryRun.acceptance_status_changed -ne $false -or
    $selectedGbTPreflightDryRun.runtime_behavior_changed -ne $false -or
    $selectedGbTPreflightDryRun.android_baseline_replaced -ne $false) {
    throw "M10 selected GB/T preflight dry-run must not write payloads, compute hashes, call external sources, write generated artifacts, change acceptance, runtime behavior, or Android baseline"
}

$artifactWriterDryRunOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $artifactWriterDryRunScript -ProjectRoot $projectPath
$artifactWriterDryRun = $artifactWriterDryRunOutput | ConvertFrom-Json

if ($artifactWriterDryRun.mode -ne "artifact_writer_dry_run_only") {
    throw "Artifact writer dry-run did not report artifact_writer_dry_run_only mode"
}

if ($artifactWriterDryRun.planned_artifact_count -ne @($artifactWriterPlan.planned_artifacts).Count) {
    throw "Artifact writer dry-run planned artifact count mismatch"
}

if ($artifactWriterDryRun.writes_performed -ne $false -or $artifactWriterDryRun.hashes_computed -ne 0 -or $artifactWriterDryRun.accepted_evidence -ne $false) {
    throw "Artifact writer dry-run must not write files, compute hashes, or claim accepted evidence"
}

if (@($artifactWriterDryRun.existing_planned_artifacts).Count -ne 0) {
    throw "Artifact writer dry-run found generated artifacts before acceptance"
}

$comparisonDryRunOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $comparisonDryRunScript -ProjectRoot $projectPath -Manifest "data/generated/astronomy/manifests/astronomy-engine-v0-draft.json"
$comparisonDryRun = $comparisonDryRunOutput | ConvertFrom-Json

if ($comparisonDryRun.mode -ne "comparison_dry_run_only") {
    throw "Comparison dry-run did not report comparison_dry_run_only mode"
}

if ($comparisonDryRun.comparison_runner_plan_id -ne $comparisonRunnerPlan.comparison_runner_plan_id) {
    throw "Comparison dry-run must report active comparison runner plan"
}

if ($comparisonDryRun.android_algorithm_version -ne $comparisonRunnerPlan.android_algorithm_version -or $comparisonDryRun.android_ruleset_id -ne $comparisonRunnerPlan.android_ruleset_id) {
    throw "Comparison dry-run must report Android baseline bindings"
}

if ($comparisonDryRun.rows_compared -ne 0 -or @($comparisonDryRun.difference_rows).Count -ne 0) {
    throw "Comparison dry-run must not claim generated comparison rows"
}

if ($comparisonDryRun.writes_performed -ne $false -or $comparisonDryRun.accepted_evidence -ne $false) {
    throw "Comparison dry-run must not write files or claim accepted evidence"
}

$goldenDryRunOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $goldenDryRunScript -ProjectRoot $projectPath
$goldenDryRun = $goldenDryRunOutput | ConvertFrom-Json

if ($goldenDryRun.mode -ne "golden_cases_dry_run_only") {
    throw "Golden-case dry-run did not report golden_cases_dry_run_only mode"
}

if ($goldenDryRun.golden_row_readiness_plan_id -ne $goldenReadinessPlan.golden_row_readiness_plan_id) {
    throw "Golden-case dry-run must report active golden row readiness plan"
}

if ($goldenDryRun.generated_rows -ne 0) {
    throw "Golden-case dry-run must not claim generated rows"
}

if ($goldenDryRun.writes_performed -ne $false -or $goldenDryRun.accepted_evidence -ne $false) {
    throw "Golden-case dry-run must not write files or claim accepted evidence"
}

if ($goldenDryRun.required_category_count -ne @($goldenPlan.required_categories).Count) {
    throw "Golden-case dry-run category count mismatch"
}

if ($goldenDryRun.readiness_category_count -ne @($goldenReadinessPlan.category_readiness).Count) {
    throw "Golden-case dry-run readiness category count mismatch"
}

if ($replayReadinessPlan.status -ne "readiness_only") {
    throw "M9 replay test readiness plan must remain readiness_only"
}

if ($replayReadinessPlan.android_algorithm_version -ne "android-date-layer-v1") {
    throw "M9 replay test readiness plan must bind android-date-layer-v1"
}

if ($replayReadinessPlan.comparison_runner_plan_id -ne $comparisonRunnerPlan.comparison_runner_plan_id) {
    throw "M9 replay test readiness plan must reference active comparison runner plan"
}

foreach ($required in @(
    "existing chart snapshots preserve algo_version",
    "existing chart snapshots preserve ruleset_id",
    "android-date-layer-v1 replay path remains available",
    "generated astronomy rows exist before comparison replay",
    "comparison runner produces classified differences",
    "replacement ADR exists before default behavior changes"
)) {
    if ($replayReadinessPlan.required_before_replay_tests -notcontains $required) {
        throw "M9 replay test readiness plan missing requirement: $required"
    }
}

foreach ($control in $replayReadinessPlan.readiness_controls) {
    if ($control.status -ne "required_not_executed" -and $control.status -ne "blocked_until_generated_rows") {
        throw "M9 replay readiness control must remain unexecuted or blocked: $($control.id)"
    }
}

foreach ($forbidden in @(
    "execute replay tests",
    "recompute old snapshots with astronomy engine",
    "change default runtime behavior",
    "replace android-date-layer-v1",
    "mark replay policy accepted",
    "claim astronomy-engine supported"
)) {
    if ($replayReadinessPlan.forbidden_in_readiness_stage -notcontains $forbidden) {
        throw "M9 replay test readiness plan missing forbidden item: $forbidden"
    }
}

$replayDryRunOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $replayDryRunScript -ProjectRoot $projectPath -Manifest "data/generated/astronomy/manifests/astronomy-engine-v0-draft.json"
$replayDryRun = $replayDryRunOutput | ConvertFrom-Json

if ($replayDryRun.mode -ne "replay_policy_dry_run_only") {
    throw "Replay-policy dry-run did not report replay_policy_dry_run_only mode"
}

if ($replayDryRun.replay_test_readiness_plan_id -ne $replayReadinessPlan.replay_test_readiness_plan_id) {
    throw "Replay-policy dry-run must report active replay test readiness plan"
}

if ($replayDryRun.replay_tests_executed -ne 0) {
    throw "Replay-policy dry-run must not claim executed replay tests"
}

if ($replayDryRun.writes_performed -ne $false -or $replayDryRun.accepted_evidence -ne $false) {
    throw "Replay-policy dry-run must not write files or claim accepted evidence"
}

if ($replayDryRun.replacement_allowed -ne $false) {
    throw "Replay-policy dry-run must not allow Android baseline replacement"
}

if ($replayDryRun.required_control_count -ne 5) {
    throw "Replay-policy dry-run required control count mismatch"
}

if ($replayDryRun.readiness_control_count -ne @($replayReadinessPlan.readiness_controls).Count) {
    throw "Replay-policy dry-run readiness control count mismatch"
}


# ---- Generated Artifact Materialization Preflight (LOOP-055) ----

$generatedArtifactMaterializationPreflightPath = Join-Path $projectPath "data/generated/astronomy/generated-artifact-materialization-preflight.json"
$generatedArtifactMaterializationPreflight = Get-Content -LiteralPath $generatedArtifactMaterializationPreflightPath -Encoding UTF8 -Raw | ConvertFrom-Json

if ($generatedArtifactMaterializationPreflight.status -ne "preflight_only") {
    throw "Generated artifact materialization preflight must remain preflight_only"
}

if ($generatedArtifactMaterializationPreflight.generated_artifact_materialization_preflight_id -ne "m10-generated-artifact-materialization-preflight-v1") {
    throw "Unexpected generated artifact materialization preflight id"
}

if ($generatedArtifactMaterializationPreflight.source_payload_prerequisites.Count -ne 4) {
    throw "Generated artifact materialization preflight must list exactly 4 source payload prerequisites"
}

foreach ($srcId in @("naif-cspice", "iau-sofa-ansi-c", "jpl-horizons-api", "gb-t-33661-2017")) {
    $prereq = $generatedArtifactMaterializationPreflight.source_payload_prerequisites | Where-Object { $_.source_id -eq $srcId }
    if (-not $prereq -or $prereq.status -ne "materialized") {
        throw "Generated artifact preflight prerequisite not materialized: $srcId"
    }
}

if ($generatedArtifactMaterializationPreflight.planned_generated_artifacts.Count -ne 4) {
    throw "Generated artifact preflight must list exactly 4 planned artifacts"
}

foreach ($planned in $generatedArtifactMaterializationPreflight.planned_generated_artifacts) {
    if ($planned.status -ne "not_generated") {
        throw "Planned artifact status must be not_generated: $($planned.path)"
    }
    if ($planned.hash_status -ne "not_computed") {
        throw "Planned artifact hash must be not_computed: $($planned.path)"
    }
}

$outDir = Join-Path $projectPath "data/generated/astronomy/out"
if (Test-Path -LiteralPath $outDir) {
    throw "Generated artifact output directory must not exist in preflight: $outDir"
}

$genPreflightDryRunScript = Join-Path $projectPath "tools/generated-artifact-materialization-preflight-dry-run.ps1"
Assert-Contains (Read-Text "tools/generated-artifact-materialization-preflight-dry-run.ps1") "DRY_RUN_ONLY" "Generated artifact materialization preflight tool must be marked dry-run only"

$genPreflightDryRunOutput = & powershell -NoProfile -ExecutionPolicy Bypass -File $genPreflightDryRunScript -ProjectRoot $projectPath
$genPreflightDryRun = $genPreflightDryRunOutput | ConvertFrom-Json

if ($genPreflightDryRun.status -ne "preflight_only") {
    throw "Generated artifact preflight dry-run must report preflight_only"
}

if ($genPreflightDryRun.generated_artifacts -ne 0) {
    throw "Generated artifact preflight dry-run must report 0 generated artifacts"
}

if ($genPreflightDryRun.generated_hashes -ne 0) {
    throw "Generated artifact preflight dry-run must report 0 generated hashes"
}

if ($genPreflightDryRun.writes -ne $false) {
    throw "Generated artifact preflight dry-run must not report writes"
}

Write-Host "Astronomy preflight check OK: $projectPath"
exit 0
