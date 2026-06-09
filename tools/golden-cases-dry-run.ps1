[CmdletBinding()]
param(
    [string]$ProjectRoot
)

$ErrorActionPreference = "Stop"

# DRY_RUN_ONLY: M9 LOOP-027 golden-case scaffold. This script must not create golden rows yet.

if ([string]::IsNullOrWhiteSpace($ProjectRoot)) {
    $ProjectRoot = Join-Path $PSScriptRoot ".."
}

$resolvedProject = Resolve-Path -LiteralPath $ProjectRoot
$projectPath = $resolvedProject.ProviderPath
$goldenPlanPath = Join-Path $projectPath "data/generated/astronomy/golden-cases-plan.json"
$readinessPlanPath = Join-Path $projectPath "data/generated/astronomy/golden-row-readiness-plan.json"

if (-not (Test-Path -LiteralPath $goldenPlanPath)) {
    throw "Missing golden cases plan: data/generated/astronomy/golden-cases-plan.json"
}

if (-not (Test-Path -LiteralPath $readinessPlanPath)) {
    throw "Missing golden row readiness plan: data/generated/astronomy/golden-row-readiness-plan.json"
}

$goldenPlan = Get-Content -LiteralPath $goldenPlanPath -Encoding UTF8 -Raw | ConvertFrom-Json
$readinessPlan = Get-Content -LiteralPath $readinessPlanPath -Encoding UTF8 -Raw | ConvertFrom-Json

if ($goldenPlan.status -ne "planned_not_generated") {
    throw "Golden cases plan must remain planned_not_generated."
}

if ($readinessPlan.status -ne "readiness_only") {
    throw "Golden row readiness plan must remain readiness_only."
}

if ($readinessPlan.golden_plan_id -ne $goldenPlan.golden_plan_id) {
    throw "Golden row readiness plan must reference active golden plan."
}

foreach ($category in $readinessPlan.category_readiness) {
    if ($category.status -ne "not_generated" -or $category.readiness_status -ne "blocked_until_generated_rows") {
        throw "Golden row readiness must remain blocked/not_generated: $($category.id)"
    }
}

$result = [pscustomobject]@{
    mode = "golden_cases_dry_run_only"
    golden_plan_id = $goldenPlan.golden_plan_id
    golden_row_readiness_plan_id = $readinessPlan.golden_row_readiness_plan_id
    status = $goldenPlan.status
    required_category_count = @($goldenPlan.required_categories).Count
    required_categories = $goldenPlan.required_categories
    readiness_category_count = @($readinessPlan.category_readiness).Count
    readiness_categories = $readinessPlan.category_readiness
    generated_rows = 0
    writes_performed = $false
    accepted_evidence = $false
}

$result | ConvertTo-Json -Depth 8
exit 0
