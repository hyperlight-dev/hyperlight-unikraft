# Install hluk on Windows from a GitHub release, no Rust toolchain needed:
#
#   irm https://raw.githubusercontent.com/hyperlight-dev/hyperlight-unikraft/main/install.ps1 | iex
#
#   $env:HLUK_VERSION = 'v0.14.1'   a release instead of the latest
#   $env:HLUK_INSTALL_DIR = 'DIR'   where hluk.exe goes (default %LOCALAPPDATA%\hluk\bin)
#
# Linux and macOS use install.sh.  hluk runs guests on the Windows Hypervisor
# Platform, an optional Windows feature.
$ErrorActionPreference = 'Stop'
# GitHub needs TLS 1.2, which older Windows PowerShell 5.1 setups don't offer
# by default (no effect on PowerShell 7).
[Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12
$ProgressPreference = 'SilentlyContinue'  # Invoke-WebRequest is slow with it

$repo = 'hyperlight-dev/hyperlight-unikraft'
$version = $env:HLUK_VERSION
$dir = if ($env:HLUK_INSTALL_DIR) { $env:HLUK_INSTALL_DIR } else { Join-Path $env:LOCALAPPDATA 'hluk\bin' }

$arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
if ($arch -ne 'X64') {
    throw "no prebuilt hluk for Windows/${arch}: ``cargo install hyperlight-unikraft`` builds it from source"
}
$target = 'x86_64-pc-windows-msvc'

if (-not $version) {
    $version = (Invoke-RestMethod -UseBasicParsing "https://api.github.com/repos/$repo/releases/latest").tag_name
    if (-not $version) { throw "cannot find the latest release of $repo" }
}
if (-not $version.StartsWith('v')) { $version = "v$version" }

$asset = "hluk-$version-$target.zip"
$base = "https://github.com/$repo/releases/download/$version"
$tmp = Join-Path ([System.IO.Path]::GetTempPath()) ([System.IO.Path]::GetRandomFileName())
New-Item -ItemType Directory -Path $tmp | Out-Null
try {
    Write-Host "downloading $base/$asset"
    # -UseBasicParsing: Windows PowerShell 5.1 otherwise needs Internet
    # Explorer's engine to parse the response.  -DisableKeepAlive: 5.1 may
    # reuse a connection the server has closed and fail the next request.
    Invoke-WebRequest -UseBasicParsing -DisableKeepAlive "$base/$asset" -OutFile (Join-Path $tmp $asset)

    # Verify against the release's checksum file when it has one.  Only its
    # absence (404) skips the check; any other failure stops the install.
    $sums = $null
    try {
        $sums = (Invoke-WebRequest -UseBasicParsing -DisableKeepAlive "$base/SHA256SUMS").Content
    } catch {
        $resp = $_.Exception.Response
        if (-not $resp -or [int]$resp.StatusCode -ne 404) { throw }
    }
    if ($sums -is [byte[]]) { $sums = [System.Text.Encoding]::UTF8.GetString($sums) }
    $line = if ($sums) { $sums -split "`n" | Where-Object { $_ -match " $([regex]::Escape($asset))\s*$" } | Select-Object -First 1 }
    if ($sums -and -not $line) { throw "$asset is not in SHA256SUMS" }
    if ($line) {
        $want = ($line -split '\s+')[0]
        $have = (Get-FileHash (Join-Path $tmp $asset) -Algorithm SHA256).Hash
        if ($want -ne $have.ToLower()) { throw "checksum mismatch for $asset" }
    }

    Expand-Archive (Join-Path $tmp $asset) -DestinationPath $tmp -Force
    New-Item -ItemType Directory -Path $dir -Force | Out-Null
    Copy-Item (Join-Path $tmp 'hluk.exe') (Join-Path $dir 'hluk.exe') -Force
} finally {
    Remove-Item $tmp -Recurse -Force -ErrorAction SilentlyContinue
}

$hluk = Join-Path $dir 'hluk.exe'
Write-Host "installed $(& $hluk --version) to $hluk"

# Put the directory on the user's PATH, for this session and later ones.  The
# registry value is read and written as is, so entries like %USERPROFILE%\bin
# stay unexpanded (SetEnvironmentVariable would expand them for good).
$key = [Microsoft.Win32.Registry]::CurrentUser.OpenSubKey('Environment', $true)
$userPath = [string]$key.GetValue('Path', '', [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames)
if (-not (($userPath -split ';') -contains $dir)) {
    $key.SetValue('Path', "$userPath;$dir".TrimStart(';'), [Microsoft.Win32.RegistryValueKind]::ExpandString)
    # Setting (and clearing) a variable through .NET tells running programs,
    # Explorer included, to reload the environment.
    [Environment]::SetEnvironmentVariable('HLUK_INSTALL_REFRESH', '1', 'User')
    [Environment]::SetEnvironmentVariable('HLUK_INSTALL_REFRESH', $null, 'User')
    Write-Host "added $dir to your PATH (open a new terminal to pick it up)"
}
$key.Close()
if (-not (($env:Path -split ';') -contains $dir)) { $env:Path += ";$dir" }

Write-Host "note: hluk needs the Windows Hypervisor Platform feature; if it is off, enable it (as administrator):"
Write-Host "  Enable-WindowsOptionalFeature -Online -FeatureName HypervisorPlatform"
Write-Host "next: hluk init"
