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
        throw "Missing product surface file: $RelativePath"
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

$rootReadme = Read-Text "README.md"
$packageReadme = Read-Text "docs/release/user-package-readme.md"
$workspaceHtml = Read-Text "frontend/index.html"
$reportHtml = Read-Text "frontend/report.html"
$topicReportHtml = Read-Text "frontend/topic-report.html"
$mainSource = Read-Text "frontend/src/main.js"
$topicReportSource = Read-Text "frontend/src/topic-report-page.js"
$renderSource = Read-Text "frontend/src/ui/render.js"
$apiClientSource = Read-Text "frontend/src/api/client.js"

$userDocs = $rootReadme + "`n" + $packageReadme
$publicUiSources = @(
    $workspaceHtml,
    $reportHtml,
    $topicReportHtml,
    $mainSource,
    $topicReportSource,
    $renderSource,
    $apiClientSource
) -join "`n"

foreach ($needle in @(
    "Fate Track",
    "Fate-Track-Windows-x64.zip",
    "minggui-desktop.exe",
    "SHA256"
)) {
    Assert-Contains $rootReadme $needle "Root README is missing user-facing release information: $needle"
}

foreach ($needle in @(
    "minggui-desktop.exe",
    "Fate-Track-Windows-x64.zip"
)) {
    Assert-Contains $packageReadme $needle "Package README is missing user startup information: $needle"
}

foreach ($forbidden in @(
    '## Developer',
    'cargo ',
    'cargo`r',
    'cargo`n',
    'npm.cmd',
    'node --',
    'tools\check-project.ps1',
    'tools/package-desktop-windows.ps1',
    'docs/release/',
    'v1-release-candidate',
    'v1-closeout',
    'desktop-packaging',
    'current-product-boundary',
    'BUILD-MANIFEST',
    'GET /api',
    'POST /api',
    'DTO',
    'score_internal',
    'backend',
    'frontend'
)) {
    Assert-NotContains $userDocs $forbidden "User docs leaked developer or governance wording: $forbidden"
}

foreach ($forbidden in @(
    ' AI ',
    'score_internal',
    '0-100',
    '404 Not Found',
    'Bad Request',
    'Method Not Allowed',
    'error.message',
    'String(error)'
)) {
    Assert-NotContains $publicUiSources $forbidden "Public UI source leaked mechanical or internal wording: $forbidden"
}

foreach ($forbiddenWindowPattern in @(
    'window.open',
    'target="_blank"',
    '<dialog',
    'showModal',
    '.close()'
)) {
    Assert-NotContains $publicUiSources $forbiddenWindowPattern "Public UI must not add window/dialog behavior: $forbiddenWindowPattern"
}

Write-Host "Product surface check OK: $projectPath"
exit 0
