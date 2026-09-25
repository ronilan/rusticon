#
# uninstall.ps1 — remove this project's terminal binary that install.ps1 put
# on the system. Designed to be run standalone — download and run.
#
# Usage:
#   powershell -ExecutionPolicy Bypass -File uninstall.ps1
#   powershell -ExecutionPolicy Bypass -File uninstall.ps1 -BinName <name>
#
# Self-elevates (UAC prompt) when not run as administrator.

param(
    # Optional binary name fallback, used when the release lookup fails.
    [string]$BinName
)

$ErrorActionPreference = "Stop"

# --- The only project-specific setting ---------------------------------------
# The GitHub repository (owner/name) whose releases carry the
# `<binary>-terminal-windows.zip` asset. Must match install.ps1.
$DefaultRepo = "ronilan/rusticon"
# ------------------------------------------------------------------------------

$folderName = ($DefaultRepo -split '/')[-1]
$Dest = Join-Path $env:ProgramFiles $folderName

# --- 1. Self-elevate if not running as administrator -------------------------
$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
           ).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "Requesting administrator rights..."
    if ($PSCommandPath) {
        $relaunchArgs = if ($BinName) { "-ExecutionPolicy Bypass -File `"$PSCommandPath`" -BinName $BinName" }
                        else          { "-ExecutionPolicy Bypass -File `"$PSCommandPath`"" }
        Start-Process powershell $relaunchArgs -Verb RunAs
        exit
    }
    # Piped straight into iex (no file on disk): re-download to temp and
    # relaunch that copy elevated.
    $tmpScript = Join-Path ([System.IO.Path]::GetTempPath()) "uninstall.ps1"
    Invoke-WebRequest -Uri "https://raw.githubusercontent.com/$DefaultRepo/main/uninstall.ps1" -OutFile $tmpScript
    $relaunchArgs = if ($BinName) { "-ExecutionPolicy Bypass -File `"$tmpScript`" -BinName $BinName" }
                    else          { "-ExecutionPolicy Bypass -File `"$tmpScript`"" }
    Start-Process powershell $relaunchArgs -Verb RunAs
    exit
}

# --- 2. Determine the binary name --------------------------------------------
# Preferred: derive it from the latest release assets (same convention as
# install.ps1: the asset is "<binary>-terminal-windows.zip").
# Fallback: pass the name explicitly via -BinName.
if ($BinName) {
    $binName = $BinName
} else {
    Write-Host "Looking up binary name from the latest release of ${DefaultRepo}..."
    $assetObj = $null
    try {
        $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$DefaultRepo/releases?per_page=1" |
            Select-Object -First 1
        $assetObj = $release.assets | Where-Object { $_.name -match '-terminal-windows\.zip$' } | Select-Object -First 1
    } catch { $assetObj = $null }
    if ((-not $assetObj) -and (Get-Command gh -ErrorAction SilentlyContinue)) {
        gh auth status *> $null
        if ($LASTEXITCODE -eq 0) {
            try {
                $release = (gh api "repos/$DefaultRepo/releases?per_page=1") | ConvertFrom-Json | Select-Object -First 1
                $assetObj = $release.assets | Where-Object { $_.name -match '-terminal-windows\.zip$' } | Select-Object -First 1
            } catch { $assetObj = $null }
        }
    }
    if (-not $assetObj) {
        Write-Error "Could not determine the binary name from the latest release. If the repository is not publicly accessible, install/authenticate the GitHub CLI (gh auth login), or pass the name explicitly: -BinName <name>"
    }
    $binName = $assetObj.name -replace '-terminal-windows\.zip$', ''
}
$exeName = "$binName.exe"

# --- 3. Remove the binary ------------------------------------------------------
$exe = Join-Path $Dest $exeName
if (Test-Path $exe) {
    Remove-Item $exe -Force
    Write-Host "Removed: $exe"
} else {
    Write-Host "Nothing to do: $exe does not exist."
}

# --- 4. Remove the install folder if it is now empty --------------------------
if ((Test-Path $Dest) -and -not (Get-ChildItem $Dest -ErrorAction SilentlyContinue)) {
    Remove-Item $Dest -Force
    Write-Host "Removed empty folder: $Dest"
}

# --- 5. Remove the folder from the system PATH (only if present) --------------
$machinePath = [Environment]::GetEnvironmentVariable("Path", "Machine")
if (($machinePath -split ';') -contains $Dest) {
    $newPath = (($machinePath -split ';') | Where-Object { $_ -ne $Dest }) -join ';'
    [Environment]::SetEnvironmentVariable("Path", $newPath, "Machine")
    Write-Host "Removed $Dest from the system PATH."
}
