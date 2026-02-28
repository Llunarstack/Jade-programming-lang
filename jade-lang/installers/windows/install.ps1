# Jade Windows install script
# Run from repo root or jade-lang: .\installers\windows\install.ps1
# Optionally: .\installers\windows\install.ps1 -BinaryPath "C:\path\to\jade.exe"

param(
    [string]$BinaryPath = "",
    [string]$InstallDir = "$env:LOCALAPPDATA\Programs\Jade",
    [switch]$NoAssociation,
    [switch]$UserPath
)

$ErrorActionPreference = "Stop"

# Resolve binary
$repoRoot = $PSScriptRoot
while (-not (Test-Path (Join-Path $repoRoot "Cargo.toml"))) {
    $repoRoot = Split-Path $repoRoot -Parent
    if (-not $repoRoot) {
        $repoRoot = Split-Path $PSScriptRoot -Parent
        $repoRoot = Join-Path $repoRoot "jade-lang"
        break
    }
}
$defaultBinary = Join-Path $repoRoot "target\release\jade.exe"
if (-not $BinaryPath) { $BinaryPath = $defaultBinary }

if (-not (Test-Path $BinaryPath)) {
    Write-Host "Jade binary not found at: $BinaryPath"
    Write-Host "Build first: cd jade-lang; cargo build --release"
    exit 1
}

if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}
Copy-Item -Path $BinaryPath -Destination (Join-Path $InstallDir "jade.exe") -Force
$jadeExe = Join-Path $InstallDir "jade.exe"
# Copy Jade icon so Explorer can show it for .jdl files (jade.exe has no embedded icon)
$iconSrc = Join-Path $PSScriptRoot "icon\jade.ico"
if (Test-Path $iconSrc) {
    Copy-Item -Path $iconSrc -Destination (Join-Path $InstallDir "jade.ico") -Force
}
Write-Host "Installed: $jadeExe"

# Add to PATH (user or machine)
$pathVar = if ($UserPath) { [Environment]::GetEnvironmentVariable("Path", "User") } else { [Environment]::GetEnvironmentVariable("Path", "Machine") }
if ($pathVar -notlike "*$InstallDir*") {
    if ($UserPath) {
        [Environment]::SetEnvironmentVariable("Path", "$pathVar;$InstallDir", "User")
    } else {
        # Requires elevation for Machine
        $pathVar = [Environment]::GetEnvironmentVariable("Path", "Machine")
        [Environment]::SetEnvironmentVariable("Path", "$pathVar;$InstallDir", "Machine")
    }
    Write-Host "Added to PATH: $InstallDir"
} else {
    Write-Host "Already in PATH: $InstallDir"
}

# File association: .jdl -> run with jade (uses Registry:: so it works without HKCR: drive)
if (-not $NoAssociation) {
    try {
        $hkcr = "Registry::HKEY_CLASSES_ROOT"
        New-Item -Path "$hkcr\.jdl" -Force | Out-Null
        Set-ItemProperty -Path "$hkcr\.jdl" -Name "(default)" -Value "Jade.SourceFile" -Force
        New-Item -Path "$hkcr\Jade.SourceFile" -Force | Out-Null
        Set-ItemProperty -Path "$hkcr\Jade.SourceFile" -Name "(default)" -Value "Jade source file" -Force
        New-Item -Path "$hkcr\Jade.SourceFile\DefaultIcon" -Force | Out-Null
        $iconPath = Join-Path $InstallDir "jade.ico"
        if (Test-Path $iconPath) {
            Set-ItemProperty -Path "$hkcr\Jade.SourceFile\DefaultIcon" -Name "(default)" -Value $iconPath -Force
        } else {
            Set-ItemProperty -Path "$hkcr\Jade.SourceFile\DefaultIcon" -Name "(default)" -Value "$jadeExe,0" -Force
        }
        New-Item -Path "$hkcr\Jade.SourceFile\shell\open\command" -Force | Out-Null
        # Use cmd /k so the console stays open after running (so you can see output when double-clicking)
        $openCmd = "cmd /k `"`"$jadeExe`" `"%1`"`""
Set-ItemProperty -Path "$hkcr\Jade.SourceFile\shell\open\command" -Name "(default)" -Value $openCmd -Force
        Write-Host "Associated .jdl files with Jade (double-click to run)"
    } catch {
        Write-Warning "Could not set file association (run as Administrator?): $_"
    }
}

Write-Host ""
Write-Host "Jade is ready. Restart the terminal and run: jade --help"
