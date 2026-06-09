[CmdletBinding()]
param(
    [string]$ProjectRoot,
    [string]$Manifest = "data/generated/astronomy/manifests/astronomy-engine-v0-draft.json"
)

$ErrorActionPreference = "Stop"

# DRY_RUN_ONLY: M9 LOOP-028 replay-policy scaffold. This script must not execute replay tests yet.

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

function Read-Text {
    param([string]$RelativePath)
    $path = Join-Path $projectPath $RelativePath
    if (-not (Test-Path -LiteralPath $path)) {
        throw "Missing astronomy planning file: $RelativePath"
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

$relativeManifest = $Manifest
if ([System.IO.Path]::IsPathRooted($Manifest)) {
    $manifestPath = Resolve-Path -LiteralPath $Manifest
    $relativeManifest = [System.IO.Path]::GetRelativePath($projectPath, $manifestPath.ProviderPath)
}

$manifestDoc = Read-Json $relativeManifest
$readinessPlan = Read-Json "data/generated/astronomy/replay-test-readiness-plan.json"
$policyText = Read-Text "data/generated/astronomy/replay-policy-draft.md"

if ($manifestDoc.acceptance_status -ne "not_accepted") {
    throw "Manifest must remain not_accepted during replay policy dry-run."
}

if ($readinessPlan.status -ne "readiness_only") {
    throw "Replay test readiness plan must remain readiness_only."
}

if ($readinessPlan.android_algorithm_version -ne "android-date-layer-v1") {
    throw "Replay test readiness plan must bind android-date-layer-v1."
}

foreach ($control in $readinessPlan.readiness_controls) {
    if ($control.status -ne "required_not_executed" -and $control.status -ne "blocked_until_generated_rows") {
        throw "Replay readiness controls must not be executed yet: $($control.id)"
    }
}

Assert-Contains $policyText "Existing V1 chart snapshots" "Replay policy must preserve existing V1 snapshots."
Assert-Contains $policyText "android-date-layer-v1" "Replay policy must name android-date-layer-v1."
Assert-Contains $policyText "Keep Android date-layer replay available" "Replay policy must require Android replay availability."
Assert-Contains $policyText "Add a replacement ADR" "Replay policy must require a replacement ADR."
Assert-Contains $policyText "Silent replacement" "Replay policy must forbid silent replacement."
Assert-Contains $policyText "replay tests exist" "Replay policy must not accept astronomy-engine before replay tests exist."

$requiredControls = @(
    [pscustomobject]@{
        id = "preserve-algo-version"
        status = "required_not_executed"
        evidence_required = "chart snapshots preserve algo_version"
    },
    [pscustomobject]@{
        id = "preserve-ruleset-id"
        status = "required_not_executed"
        evidence_required = "chart snapshots preserve ruleset_id"
    },
    [pscustomobject]@{
        id = "android-replay-available"
        status = "required_not_executed"
        evidence_required = "existing android-date-layer-v1 snapshots replay without astronomy replacement"
    },
    [pscustomobject]@{
        id = "replacement-adr"
        status = "required_not_executed"
        evidence_required = "later ADR explicitly accepts default runtime replacement"
    },
    [pscustomobject]@{
        id = "difference-classification"
        status = "required_not_executed"
        evidence_required = "android-vs-astronomy differences classified through taxonomy"
    }
)

$result = [pscustomobject]@{
    mode = "replay_policy_dry_run_only"
    replay_test_readiness_plan_id = $readinessPlan.replay_test_readiness_plan_id
    manifest_id = $manifestDoc.manifest_id
    source_policy_id = $manifestDoc.source_policy_id
    android_algorithm_version = "android-date-layer-v1"
    android_ruleset_id = $readinessPlan.android_ruleset_id
    required_control_count = $requiredControls.Count
    required_controls = $requiredControls
    readiness_control_count = @($readinessPlan.readiness_controls).Count
    readiness_controls = $readinessPlan.readiness_controls
    replay_tests_executed = 0
    writes_performed = $false
    accepted_evidence = $false
    replacement_allowed = $false
}

$result | ConvertTo-Json -Depth 8
exit 0
