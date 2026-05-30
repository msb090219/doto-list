# Doto Installation Script for Windows
# This script downloads and installs the latest release of Doto

$ErrorActionPreference = "Stop"

Write-Host "Installing Doto for Windows..." -ForegroundColor Green

# Detect architecture
$arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
Write-Host "Detected architecture: $arch" -ForegroundColor Cyan

# Get latest release info
Write-Host "Checking for latest release..." -ForegroundColor Yellow
$latestUrl = "https://api.github.com/repos/msb090219/doto-list/releases/latest"

try {
    $releaseInfo = Invoke-RestMethod -Uri $latestUrl
    Write-Host "Found release: $($releaseInfo.name)" -ForegroundColor Green
}
catch {
    Write-Host ""
    Write-Host "Installation failed: Unable to find releases for doto-list." -ForegroundColor Red
    Write-Host ""
    Write-Host "This could mean:" -ForegroundColor Yellow
    Write-Host "  1. No releases have been created yet (check GitHub releases page)"
    Write-Host "  2. The repository name has changed"
    Write-Host "  3. Network connectivity issues"
    Write-Host ""
    Write-Host "Please check the repository and try again." -ForegroundColor Yellow
    Write-Host "Repository: https://github.com/msb090219/doto-list" -ForegroundColor Cyan
    exit 1
}

$asset = $releaseInfo.assets | Where-Object { $_.name -like "*windows-$arch*" } | Select-Object -First 1

if (-not $asset) {
    Write-Host "Installation failed: No binary found for windows-$arch" -ForegroundColor Red
    Write-Host "Available binaries:" -ForegroundColor Yellow
    foreach ($a in $releaseInfo.assets) {
        Write-Host "  - $($a.name)" -ForegroundColor Cyan
    }
    exit 1
}

$downloadUrl = $asset.browser_download_url
$installDir = "$env:USERPROFILE\.local\bin"

Write-Host "Downloading from: $downloadUrl"

# Create installation directory
New-Item -ItemType Directory -Force -Path $installDir | Out-Null

# Download binary
Invoke-RestMethod -Uri $downloadUrl -OutFile "$installDir\doto.exe"

Write-Host "Doto installed to: $installDir" -ForegroundColor Green

# Check if already in PATH
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$installDir*") {
    Write-Host ""
    Write-Host "Adding doto to your PATH..." -ForegroundColor Yellow

    # Add to user PATH
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$installDir", "User")

    Write-Host "PATH updated successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "IMPORTANT: Please restart your terminal for PATH changes to take effect." -ForegroundColor Yellow
} else {
    Write-Host "doto is already in your PATH!" -ForegroundColor Green
}

Write-Host ""
Write-Host "Installation complete!" -ForegroundColor Green
Write-Host "Run 'doto' to start using your terminal todo app!" -ForegroundColor Cyan
Write-Host ""
Write-Host "If doto doesn't work immediately, restart your terminal." -ForegroundColor Yellow
