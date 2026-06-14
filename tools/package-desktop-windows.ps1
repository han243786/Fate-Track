[CmdletBinding()]
param(
    [string]$ProjectRoot,
    [string]$OutputDir,
    [switch]$SkipQualityGate
)

$ErrorActionPreference = "Stop"

if ([string]::IsNullOrWhiteSpace($ProjectRoot)) {
    $ProjectRoot = Join-Path $PSScriptRoot ".."
}

$resolvedProject = Resolve-Path -LiteralPath $ProjectRoot
$projectPath = $resolvedProject.ProviderPath

if ([string]::IsNullOrWhiteSpace($OutputDir)) {
    $OutputDir = Join-Path $projectPath "dist"
}

function Invoke-Checked {
    param(
        [string]$Command,
        [string[]]$Arguments,
        [string]$WorkingDirectory
    )

    Write-Host ">> $Command $($Arguments -join ' ')"
    Push-Location -LiteralPath $WorkingDirectory
    try {
        & $Command @Arguments
        if ($LASTEXITCODE -ne 0) {
            throw "Command failed with exit code ${LASTEXITCODE}: $Command $($Arguments -join ' ')"
        }
    }
    finally {
        Pop-Location
    }
}

if (-not $SkipQualityGate) {
    Invoke-Checked "cargo" @("clippy", "--all-targets", "--", "-D", "warnings") $projectPath
    Invoke-Checked "cargo" @("test") $projectPath
    Invoke-Checked "npm.cmd" @("run", "check") (Join-Path $projectPath "frontend")
    Invoke-Checked "powershell" @("-NoProfile", "-ExecutionPolicy", "Bypass", "-File", "tools\check-project.ps1") $projectPath
}

Invoke-Checked "cargo" @("build", "-p", "minggui-desktop", "--release", "--locked") $projectPath

$outputPath = Join-Path $OutputDir "desktop-windows"
New-Item -ItemType Directory -Path $outputPath -Force | Out-Null

$stamp = Get-Date -Format "yyyyMMdd-HHmmss"
$packageName = "Fate-Track-Windows-x64"
$zipFileName = "Fate-Track-Windows-x64.zip"
$stageRoot = Join-Path ([System.IO.Path]::GetTempPath()) "fate-track-desktop-package-$stamp"
$stagePath = Join-Path $stageRoot $packageName
New-Item -ItemType Directory -Path $stagePath -Force | Out-Null

Copy-Item -LiteralPath (Join-Path $projectPath "target\release\minggui-desktop.exe") -Destination $stagePath
Copy-Item -LiteralPath (Join-Path $projectPath "docs\release\user-package-readme.md") -Destination (Join-Path $stagePath "README.md")

$zipPath = Join-Path $outputPath $zipFileName
if (Test-Path -LiteralPath $zipPath) {
    Remove-Item -LiteralPath $zipPath -Force
}
Compress-Archive -Path $stagePath -DestinationPath $zipPath

$hash = Get-FileHash -Algorithm SHA256 -LiteralPath $zipPath
$checksumPath = Join-Path $outputPath "SHA256SUMS.txt"
"$($hash.Hash.ToLowerInvariant())  $zipFileName" | Set-Content -LiteralPath $checksumPath -Encoding UTF8

Write-Host "Windows desktop package: $zipPath"
Write-Host "SHA256 checksum file: $checksumPath"
