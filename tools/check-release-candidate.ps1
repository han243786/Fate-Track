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
        throw "Missing release artifact: $RelativePath"
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

function Assert-NotContains {
    param(
        [string]$Text,
        [string]$Needle,
        [string]$Message
    )
    if ($Text.Contains($Needle)) {
        throw $Message
    }
}

$capabilities = Read-Text "backend/src/api/capabilities.rs"
$ledger = Read-Text "markdown/20-roadmap/93-capability-promotion-ledger.md"
$readme = Read-Text "README.md"
$release = Read-Text "docs/release/v1-release-candidate.md"
$releaseCloseout = Read-Text "docs/release/v1-closeout.md"
$productBoundary = Read-Text "docs/release/current-product-boundary.md"
$desktopPackaging = Read-Text "docs/release/desktop-packaging.md"
$desktopWorkflow = Read-Text ".github/workflows/release-desktop.yml"
$desktopPackageScript = Read-Text "tools/package-desktop-windows.ps1"
$appTests = Read-Text "backend/src/app.rs"
$frontendHtml = Read-Text "frontend/index.html"
$frontendTests = Read-Text "frontend/tests/workspace-markup.test.mjs"

foreach ($closeout in @(
    "markdown/20-roadmap/12-milestone-01-closeout.md",
    "markdown/20-roadmap/14-milestone-02-closeout.md",
    "markdown/20-roadmap/16-milestone-03-closeout.md",
    "markdown/20-roadmap/18-milestone-04-closeout.md",
    "markdown/20-roadmap/20-milestone-05-closeout.md",
    "markdown/20-roadmap/22-milestone-06-closeout.md",
    "markdown/20-roadmap/24-milestone-07-closeout.md",
    "markdown/20-roadmap/25-milestone-08-preflight.md",
    "markdown/20-roadmap/26-milestone-08-closeout.md"
)) {
    [void](Read-Text $closeout)
}

foreach ($pair in @(
    @("chart-create", "supported"),
    @("analysis-snapshot", "supported"),
    @("case-management", "restricted"),
    @("share-preview", "restricted"),
    @("settings", "restricted"),
    @("luck-cycles", "supported"),
    @("glossary", "supported"),
    @("chart-detail", "supported"),
    @("case-export", "restricted"),
    @("data-derivation", "restricted"),
    @("astronomy-engine", "supported"),
    @("chart-report", "restricted")
)) {
    $id = $pair[0]
    $status = $pair[1]
    Assert-Contains $capabilities "id: `"$id`"," "Capability catalog missing $id"
    Assert-Contains $capabilities "status: `"$status`"," "Capability $id is not frozen as $status"
    Assert-Contains $ledger "| ``$id`` |" "Capability ledger missing $id"
}

foreach ($needle in @(
    "All M0-M28 closed",
    "docs/release/current-product-boundary.md",
    "1901-2100",
    "Android date layer remains the accepted current baseline.",
    "luck-cycles",
    "astronomy-engine",
    "share-preview",
    "chart-report",
    "Rollback And Downgrade"
)) {
    Assert-Contains $release $needle "Release candidate document missing: $needle"
}

Assert-Contains $ledger "| ``release-candidate`` | supported" "Release candidate is not promoted in capability ledger"

foreach ($needle in @(
    "Fate Track"
)) {
    Assert-Contains $readme $needle "README missing user-facing product identity: $needle"
}

foreach ($needle in @(
    'current-product-boundary.md',
    'supported',
    'restricted',
    'planned',
    'score_internal',
    'GET /api/luck/cycles',
    'relationship',
    'wealth',
    'family',
    'career',
    '1901',
    '2100'
)) {
    Assert-Contains $productBoundary $needle "Current product boundary missing: $needle"
}

Assert-NotContains $readme "docs/release/" "README must stay user-facing and avoid governance document links"

foreach ($needle in @(
    "GET /api/charts?date=",
    "GET /api/analysis/snapshot",
    "GET /api/cases",
    "GET /api/share/preview",
    "GET /api/charts/report",
    "GET /api/glossary",
    "GET /api/luck/cycles",
    "GET /api/cases/export"
)) {
    Assert-Contains $releaseCloseout $needle "Release closeout missing API boundary: $needle"
}

foreach ($needle in @(
    "share_preview_returns_redacted_public_dto_without_private_case_state",
    "share_revoke_makes_token_unavailable_without_case_existence_leak",
    "snapshot_id",
    "private-note"
)) {
    Assert-Contains $appTests $needle "Backend privacy/release test evidence missing: $needle"
}

foreach ($needle in @(
    "chart-title",
    "analysis-title",
    "luck-title"
)) {
    Assert-Contains $frontendHtml $needle "Frontend workspace markup missing: $needle"
}

foreach ($needle in @(
    "Cloud Sync",
    "True Solar Time",
    "Astronomy Engine"
)) {
    Assert-Contains $frontendTests $needle "Frontend overclaim test missing forbidden term: $needle"
}

Assert-NotContains $release "full durable sharing supported" "Release document overclaims durable sharing"
Assert-NotContains $release "accounts supported" "Release document overclaims accounts"
Assert-NotContains $release "cloud sync supported" "Release document overclaims cloud sync"

foreach ($needle in @(
    "cargo clippy --all-targets -- -D warnings",
    "cargo test",
    "user-package-readme.md",
    "Fate-Track-Windows-x64.zip",
    "SHA256SUMS.txt"
)) {
    Assert-Contains $desktopPackaging $needle "Desktop packaging document missing: $needle"
}

foreach ($needle in @(
    "Run strict Rust lint",
    "cargo clippy --all-targets -- -D warnings",
    "Run governance release gate",
    "docs/release/user-package-readme.md",
    "Fate-Track-Desktop-All-Platforms.zip"
)) {
    Assert-Contains $desktopWorkflow $needle "Desktop release workflow missing: $needle"
}

foreach ($needle in @(
    "cargo",
    "clippy",
    "npm.cmd",
    "tools\check-project.ps1",
    "docs\release\user-package-readme.md",
    "cargo",
    "build",
    "Fate-Track-Windows-x64.zip",
    "SHA256SUMS.txt"
)) {
    Assert-Contains $desktopPackageScript $needle "Windows desktop packaging script missing: $needle"
}

foreach ($forbiddenPackageCopy in @(
    "Copy-Item -LiteralPath (Join-Path `$projectPath `"README.md`") -Destination `$stagePath",
    "Copy-Item -LiteralPath (Join-Path `$projectPath `"docs\release\v1-release-candidate.md`")",
    "Copy-Item -LiteralPath (Join-Path `$projectPath `"docs\release\v1-closeout.md`")",
    "Copy-Item -LiteralPath (Join-Path `$projectPath `"docs\release\desktop-packaging.md`")",
    "Copy-Item -LiteralPath (Join-Path `$projectPath `"docs\release\current-product-boundary.md`")"
)) {
    Assert-NotContains $desktopPackageScript $forbiddenPackageCopy "Windows user package must not include governance document copy: $forbiddenPackageCopy"
}

foreach ($forbiddenWorkflowCopy in @(
    "cp README.md dist/Fate-Track",
    "Copy-Item README.md dist/Fate-Track",
    "v1-release-candidate.md dist/Fate-Track",
    "v1-closeout.md dist/Fate-Track",
    "desktop-packaging.md dist/Fate-Track",
    "current-product-boundary.md dist/Fate-Track"
)) {
    Assert-NotContains $desktopWorkflow $forbiddenWorkflowCopy "Desktop workflow user artifact must not include governance document copy: $forbiddenWorkflowCopy"
}

Write-Host "Release candidate check OK: $projectPath"
exit 0
