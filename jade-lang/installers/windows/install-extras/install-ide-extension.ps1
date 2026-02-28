# Install Jade VS Code extension into user's VS Code and Cursor extensions folders.
# Called by the Jade Windows installer when "Install Jade extension for VS Code and Cursor" is selected.
param([Parameter(Mandatory=$true)][string]$JadeAppPath)

$ErrorActionPreference = "Stop"
$extName = "jade-language-0.2.0"
$src = Join-Path $JadeAppPath "ide\vscode"
if (-not (Test-Path $src)) { exit 0 }

$destVSCode = Join-Path $env:USERPROFILE ".vscode\extensions\$extName"
$destCursor = Join-Path $env:USERPROFILE ".cursor\extensions\$extName"

foreach ($dest in @($destVSCode, $destCursor)) {
    $parent = Split-Path -Parent $dest
    New-Item -ItemType Directory -Force -Path $parent | Out-Null
    New-Item -ItemType Directory -Force -Path $dest | Out-Null
    Copy-Item -Path (Join-Path $src "*") -Destination $dest -Recurse -Force
}
