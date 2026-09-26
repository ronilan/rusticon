#
# install.ps1 — globally install this project's terminal binary from its latest
# GitHub release. Designed to be run standalone — download and run; no git
# clone or checkout required. Installs the binary into a subfolder of
# C:\Program Files and adds that folder to the system PATH.
#
# The repository and its release assets must be publicly accessible.
#
# Usage:
#   powershell -ExecutionPolicy Bypass -File install.ps1
#   powershell -ExecutionPolicy Bypass -File install.ps1 -Repo owner/repo
#
# Self-elevates (UAC prompt) when not run as administrator.

param(
    # Optional owner/repo override (e.g. for testing a fork).
    [string]$Repo
)

$ErrorActionPreference = "Stop"

# --- The only project-specific setting ---------------------------------------
# The GitHub repository (owner/name) whose releases carry the
# `<binary>-terminal-windows.zip` asset built by
# .github/workflows/downloadable_binaries.yml.
$DefaultRepo = "ronilan/rusticon"
# ------------------------------------------------------------------------------

$repoOwnerAndName = if ($Repo) { $Repo } else { $DefaultRepo }
$folderName = ($repoOwnerAndName -split '/')[-1]

# --- 1. Self-elevate if not running as administrator -------------------------
# (Required to write to C:\Program Files and edit the system PATH.)
$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
           ).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "Requesting administrator rights..."
    if ($PSCommandPath) {
        # Run from a file on disk: relaunch it elevated.
        $relaunchArgs = if ($Repo) { "-ExecutionPolicy Bypass -File `"$PSCommandPath`" -Repo $Repo" }
                        else       { "-ExecutionPolicy Bypass -File `"$PSCommandPath`"" }
        Start-Process powershell $relaunchArgs -Verb RunAs
        exit
    }
    # Piped straight into iex (no file on disk): re-download to temp and
    # relaunch that copy elevated.
    $tmpScript = Join-Path ([System.IO.Path]::GetTempPath()) "install.ps1"
    Invoke-WebRequest -Uri "https://raw.githubusercontent.com/$repoOwnerAndName/main/install.ps1" -OutFile $tmpScript
    $relaunchArgs = if ($Repo) { "-ExecutionPolicy Bypass -File `"$tmpScript`" -Repo $Repo" }
                    else       { "-ExecutionPolicy Bypass -File `"$tmpScript`"" }
    Start-Process powershell $relaunchArgs -Verb RunAs
    exit
}

$Dest = Join-Path $env:ProgramFiles $folderName

# --- 2. Resolve the asset's download URL from the latest release -------------
Write-Host "Looking up latest release for ${repoOwnerAndName}..."
$release = $null
try {
    $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$repoOwnerAndName/releases?per_page=1" |
        Select-Object -First 1
} catch { $release = $null }
$assetObj = $release.assets | Where-Object { $_.name -match '-terminal-windows\.zip$' } | Select-Object -First 1
if (-not $assetObj) {
    Write-Error "No *-terminal-windows.zip asset found in the latest release. Verify that $repoOwnerAndName and its release assets are publicly accessible."
}

# The asset is "<binary>-terminal-windows.zip" containing the bare exe (the
# CI zip step stores the filename without extension), so re-add the extension.
$binName = $assetObj.name -replace '-terminal-windows\.zip$', ''
$exeName = "$binName.exe"
$url = $assetObj.browser_download_url

# --- 3. Download and extract into a temp dir ---------------------------------
$tmp = Join-Path ([System.IO.Path]::GetTempPath()) ("install_" + [Guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Path $tmp | Out-Null
try {
    Invoke-WebRequest -Uri $url -OutFile (Join-Path $tmp $assetObj.name)
    Expand-Archive -Path (Join-Path $tmp $assetObj.name) -DestinationPath $tmp -Force

    # --- 4. Install into the destination folder ------------------------------
    New-Item -ItemType Directory -Path $Dest -Force | Out-Null
    $exe = Join-Path $Dest $exeName
    Copy-Item (Join-Path $tmp $exeName) $exe -Force
    Unblock-File $exe

    # --- 5. Append to the system PATH (only if not already there) ------------
    $machinePath = [Environment]::GetEnvironmentVariable("Path", "Machine")
    if (($machinePath -split ';') -notcontains $Dest) {
        [Environment]::SetEnvironmentVariable("Path", "$machinePath;$Dest", "Machine")
        Write-Host "Added $Dest to the system PATH."
    }

    Write-Host "Installed: $exe"
    Write-Host "Verify it works: open a NEW terminal anywhere and run '$binName'"
}
finally {
    Remove-Item $tmp -Recurse -Force -ErrorAction SilentlyContinue
}
