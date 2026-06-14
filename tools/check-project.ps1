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

Push-Location $projectPath
try {
    cargo fmt --check
    cargo test

    Push-Location (Join-Path $projectPath "frontend")
    try {
        npm.cmd run check
    } finally {
        Pop-Location
    }

    powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $projectPath "tools\check-governance-scaffold.ps1") -ProjectRoot $projectPath
    powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $projectPath "tools\check-product-surface.ps1") -ProjectRoot $projectPath
    powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $projectPath "tools\check-release-candidate.ps1") -ProjectRoot $projectPath
    powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $projectPath "tools\check-astronomy-preflight.ps1") -ProjectRoot $projectPath
} finally {
    Pop-Location
}
