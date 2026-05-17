# Run one example per language to verify the build+run pipeline.
# Languages: C, Rust, Go, .NET, PowerShell, Shell, Python
#
# Usage: .\scripts\test-examples.ps1

$ErrorActionPreference = "Continue"

$RepoRoot = Split-Path -Parent $PSScriptRoot
$ExamplesDir = Join-Path $RepoRoot "examples"
$Failures = 0

function Run-Example {
    param([string]$Name)

    $dir = Join-Path $ExamplesDir $Name
    Write-Host "--- $Name ---"

    if (-not (Test-Path $dir)) {
        Write-Host "  SKIP: directory not found"
        return
    }

    Push-Location $dir

    # Build
    Write-Host -NoNewline "  build: "
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    just build 2>&1 | Out-Null
    $sw.Stop()
    if ($LASTEXITCODE -eq 0) {
        Write-Host "ok $($sw.ElapsedMilliseconds)ms"
    } else {
        Write-Host "FAILED (exit=$LASTEXITCODE, $($sw.ElapsedMilliseconds)ms)"
        $script:Failures++
        Pop-Location
        return
    }

    # Rootfs
    Write-Host -NoNewline "  rootfs: "
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    just rootfs 2>&1 | Out-Null
    $sw.Stop()
    if ($LASTEXITCODE -eq 0) {
        Write-Host "ok $($sw.ElapsedMilliseconds)ms"
    } else {
        Write-Host "FAILED (exit=$LASTEXITCODE, $($sw.ElapsedMilliseconds)ms)"
        $script:Failures++
        Pop-Location
        return
    }

    # Run
    Write-Host -NoNewline "  run: "
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    just run 2>&1 | Out-Null
    $sw.Stop()
    if ($LASTEXITCODE -eq 0) {
        Write-Host "ok $($sw.ElapsedMilliseconds)ms"
    } else {
        Write-Host "FAILED (exit=$LASTEXITCODE, $($sw.ElapsedMilliseconds)ms)"
        $script:Failures++
    }

    Pop-Location
}

Write-Host "=== test-examples - $(Get-Date) ==="
Write-Host ""

Run-Example "helloworld-c"
Run-Example "rust"
Run-Example "go"
Run-Example "dotnet"
Run-Example "powershell"
Run-Example "shell"
Run-Example "python"

Write-Host ""
if ($Failures -gt 0) {
    Write-Host "DONE with $Failures failure(s)"
    exit $Failures
} else {
    Write-Host "ALL PASSED"
    exit 0
}
