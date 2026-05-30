# Doto Installation Script for Windows
$ErrorActionPreference = "Stop"

Write-Host "Installing Doto for Windows..." -ForegroundColor Green

# Detect architecture
$arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
Write-Host "Detected architecture: $arch" -ForegroundColor Cyan

# Get latest release info
Write-Host "Checking for latest release..." -ForegroundColor Yellow
$latestUrl = "https://api.github.com/repos/msb090219/doto/releases/latest"

try {
    $releaseInfo = Invoke-RestMethod -Uri $latestUrl
    Write-Host "Found release: $($releaseInfo.name)" -ForegroundColor Green
}
catch {
    Write-Host "No releases found yet. This is expected for testing." -ForegroundColor Yellow
    Write-Host "The installer will work once you create a GitHub release." -ForegroundColor Yellow
    exit 0
}

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

Write-Host "Doto installed successfully to: $installDir" -ForegroundColor Green
Write-Host "Add to PATH: $installDir" -ForegroundColor Yellow
Write-Host "Then run: doto" -ForegroundColor Green
