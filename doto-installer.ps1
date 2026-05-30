# Doto Installation Script for Windows
# This script downloads and installs the latest release of Doto

$ErrorActionPreference = "Stop"

Write-Host "Installing Doto for Windows..." -ForegroundColor Green

# Detect architecture
$arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }

# Get latest release info
$latestUrl = "https://api.github.com/repos/msb090219/doto/releases/latest"
$releaseInfo = Invoke-RestMethod -Uri $latestUrl
$asset = $releaseInfo.assets | Where-Object { $_.name -like "*windows-$arch*" } | Select-Object -First 1

if (-not $asset) {
    Write-Host "Could not find release for windows-$arch" -ForegroundColor Red
    exit 1
}

$downloadUrl = $asset.browser_download_url
$installDir = "$env:USERPROFILE\.local\bin"

Write-Host "Downloading from: $downloadUrl"

# Create installation directory
New-Item -ItemType Directory -Force -Path $installDir | Out-Null

# Download binary
Invoke-RestMethod -Uri $downloadUrl -OutFile "$installDir\doto.exe"

Write-Host "✓ Doto installed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "Add to PATH (if not already added):" -ForegroundColor Yellow
Write-Host "  $installDir"
Write-Host ""
Write-Host "Then run 'doto' to start using your terminal todo app!" -ForegroundColor Green
