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

function Text-FromCodepoints {
    param([int[]]$Codepoints)
    return -join ($Codepoints | ForEach-Object { [char]$_ })
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
$releaseZipPath = Join-Path $projectPath "release-artifacts/desktop-windows/latest/Fate-Track-Windows-x64.zip"
$releaseChecksumPath = Join-Path $projectPath "release-artifacts/desktop-windows/latest/SHA256SUMS.txt"

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
    'frontend',
    (Text-FromCodepoints @(0x751f, 0x6210, 0x547d, 0x76d8)),
    (Text-FromCodepoints @(0x7cfb, 0x7edf, 0x4f1a)),
    (Text-FromCodepoints @(0x6307, 0x5b9a, 0x5e74, 0x4efd)),
    (Text-FromCodepoints @(0x663e, 0x5f0f, 0x5e74, 0x4efd)),
    (Text-FromCodepoints @(0x663e, 0x5f0f, 0x5e74, 0x5ea6))
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

if (-not (Test-Path -LiteralPath $releaseZipPath)) {
    throw "Missing latest Windows release zip: $releaseZipPath"
}
if (-not (Test-Path -LiteralPath $releaseChecksumPath)) {
    throw "Missing latest Windows release checksum: $releaseChecksumPath"
}

Add-Type -AssemblyName System.IO.Compression.FileSystem
$zip = [System.IO.Compression.ZipFile]::OpenRead($releaseZipPath)
try {
    $entries = @($zip.Entries | ForEach-Object { $_.FullName.Replace('\', '/') })
    $entryText = $entries -join "`n"
    foreach ($requiredEntry in @(
        'Fate-Track-Windows-x64/minggui-desktop.exe',
        'Fate-Track-Windows-x64/README.md'
    )) {
        Assert-Contains $entryText $requiredEntry "Latest Windows release zip is missing: $requiredEntry"
    }
    foreach ($forbiddenEntry in @(
        '/docs/',
        'BUILD-MANIFEST',
        'v1-release-candidate',
        'v1-closeout',
        'desktop-packaging',
        'current-product-boundary'
    )) {
        Assert-NotContains $entryText $forbiddenEntry "Latest Windows release zip leaked internal file: $forbiddenEntry"
    }

    $readmeEntry = $zip.Entries | Where-Object { $_.FullName.Replace('\', '/') -eq 'Fate-Track-Windows-x64/README.md' } | Select-Object -First 1
    if ($null -eq $readmeEntry) {
        throw "Latest Windows release zip is missing packaged README.md"
    }
    $reader = [System.IO.StreamReader]::new($readmeEntry.Open(), [System.Text.Encoding]::UTF8)
    try {
        $packagedReadme = $reader.ReadToEnd()
    } finally {
        $reader.Dispose()
    }
    foreach ($forbidden in @(
        'docs/release/',
        'v1-release-candidate',
        'v1-closeout',
        'desktop-packaging',
        'current-product-boundary',
        'GET /api',
        'POST /api',
        'DTO',
        'score_internal',
        'backend',
        'frontend',
        ' AI ',
        (Text-FromCodepoints @(0x751f, 0x6210, 0x547d, 0x76d8)),
        (Text-FromCodepoints @(0x7cfb, 0x7edf, 0x4f1a)),
        (Text-FromCodepoints @(0x6307, 0x5b9a, 0x5e74, 0x4efd)),
        (Text-FromCodepoints @(0x663e, 0x5f0f, 0x5e74, 0x4efd)),
        (Text-FromCodepoints @(0x663e, 0x5f0f, 0x5e74, 0x5ea6))
    )) {
        Assert-NotContains $packagedReadme $forbidden "Packaged README leaked developer or governance wording: $forbidden"
    }
} finally {
    $zip.Dispose()
}

$checksumText = [System.IO.File]::ReadAllText($releaseChecksumPath, [System.Text.Encoding]::UTF8).Trim()
$checksum = ($checksumText -split '\s+')[0]
Assert-Contains $rootReadme $checksum "Root README SHA256 does not match latest Windows release checksum"

Write-Host "Product surface check OK: $projectPath"
exit 0
