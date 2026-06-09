[CmdletBinding()]
param(
    [string]$ProjectRoot,
    [int]$TimeoutSec = 10
)

$ErrorActionPreference = "Stop"

if ([string]::IsNullOrWhiteSpace($ProjectRoot)) {
    $ProjectRoot = Join-Path $PSScriptRoot ".."
}

$resolvedProject = Resolve-Path -LiteralPath $ProjectRoot
$projectPath = $resolvedProject.ProviderPath
$policyPath = Join-Path $projectPath "data/generated/astronomy/source-policy.json"

if (-not (Test-Path -LiteralPath $policyPath)) {
    throw "Missing astronomy source policy: data/generated/astronomy/source-policy.json"
}

$policy = Get-Content -LiteralPath $policyPath -Encoding UTF8 -Raw | ConvertFrom-Json

function Test-Url {
    param(
        [string]$Id,
        [string]$Url,
        [bool]$Required
    )

    try {
        $response = Invoke-WebRequest -Uri $Url -TimeoutSec $TimeoutSec -UseBasicParsing
        return [pscustomobject]@{
            id = $Id
            url = $Url
            required = $Required
            status = "ok"
            http_status = [int]$response.StatusCode
            bytes = $response.Content.Length
        }
    } catch {
        return [pscustomobject]@{
            id = $Id
            url = $Url
            required = $Required
            status = if ($Required) { "failed" } else { "warning" }
            error = $_.Exception.Message
        }
    }
}

$checks = @(
    (Test-Url -Id $policy.calendar_standard.id -Url $policy.calendar_standard.official_url -Required $false),
    (Test-Url -Id $policy.online_validation_source.id -Url $policy.online_validation_source.official_url -Required $true),
    (Test-Url -Id "$($policy.online_validation_source.id)-api-smoke" -Url "https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%2710%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27NO%27" -Required $true),
    (Test-Url -Id $policy.standards_routine_source.id -Url $policy.standards_routine_source.official_url -Required $true),
    (Test-Url -Id $policy.offline_reproducibility_source.id -Url $policy.offline_reproducibility_source.official_url -Required $true)
)

$failed = @($checks | Where-Object { $_.status -eq "failed" })
$warnings = @($checks | Where-Object { $_.status -eq "warning" })

$result = [pscustomobject]@{
    source_policy_id = $policy.source_policy_id
    checked_at_local = (Get-Date).ToString("s")
    timeout_seconds = $TimeoutSec
    checks = $checks
    summary = if ($failed.Count -gt 0) { "failed" } elseif ($warnings.Count -gt 0) { "warning" } else { "ok" }
}

$result | ConvertTo-Json -Depth 8

if ($failed.Count -gt 0) {
    exit 1
}

exit 0
