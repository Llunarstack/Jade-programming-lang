# Register .jdl file extension with the Jade icon (all .jdl files on this PC will show the logo)
# Run from anywhere. Uses icon from this script's folder: installers\windows\icon\jade.ico
# Optional: run as Administrator if you get "Access denied" on registry.

param(
    [string]$IconDir = "$env:LOCALAPPDATA\Programs\Jade"
)

$ErrorActionPreference = "Stop"

$scriptDir = $PSScriptRoot
$iconSrc = Join-Path $scriptDir "icon\jade.ico"

if (-not (Test-Path $iconSrc)) {
    Write-Error "Icon not found: $iconSrc. Run this from the repo or ensure jade-lang\installers\windows\icon\jade.ico exists."
    exit 1
}

if (-not (Test-Path $IconDir)) {
    New-Item -ItemType Directory -Path $IconDir -Force | Out-Null
}
$iconDest = Join-Path $IconDir "jade.ico"
Copy-Item -Path $iconSrc -Destination $iconDest -Force
Write-Host "Icon copied to: $iconDest"

try {
    $hkcr = "Registry::HKEY_CLASSES_ROOT"
    New-Item -Path "$hkcr\.jdl" -Force | Out-Null
    Set-ItemProperty -Path "$hkcr\.jdl" -Name "(default)" -Value "Jade.SourceFile" -Force
    New-Item -Path "$hkcr\Jade.SourceFile" -Force | Out-Null
    Set-ItemProperty -Path "$hkcr\Jade.SourceFile" -Name "(default)" -Value "Jade source file" -Force
    New-Item -Path "$hkcr\Jade.SourceFile\DefaultIcon" -Force | Out-Null
    Set-ItemProperty -Path "$hkcr\Jade.SourceFile\DefaultIcon" -Name "(default)" -Value $iconDest -Force
    Write-Host "Done. All .jdl files will use the Jade icon. You may need to restart Explorer or refresh (F5) the folder."
} catch {
    Write-Warning "Registry update failed (run as Administrator?): $_"
    exit 1
}
