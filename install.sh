#!/bin/bash

# Doto Installation Script for macOS and Linux
# This script downloads and installs the latest release of Doto

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)     OS="linux";;
    Darwin*)    OS="macos";;
    MINGW*|MSYS*|CYGWIN*)
                echo -e "${RED}Please use the PowerShell installer (doto-installer.ps1) on Windows${NC}"
                echo "Or download the binary directly from GitHub releases"
                exit 1;;
    *)          echo -e "${RED}Unsupported OS: $OS${NC}"; exit 1;;
esac

case "$ARCH" in
    x86_64|amd64)    ARCH="x86_64";;
    aarch64|arm64)    ARCH="aarch64";;
    armv7l)          ARCH="armv7";;
    *)               echo -e "${RED}Unsupported architecture: $ARCH${NC}"; exit 1;;
esac

echo -e "${GREEN}Installing Doto for $OS-$ARCH...${NC}"

# Get latest release info
LATEST_URL=$(curl -s https://api.github.com/repos/msb090219/doto/releases/latest |
              grep "browser_download_url" |
              grep "$OS-$ARCH" |
              cut -d '"' -f 4)

if [ -z "$LATEST_URL" ]; then
    echo -e "${RED}Could not find release for $OS-$ARCH${NC}"
    exit 1
fi

# Create installation directory
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

echo "Downloading from: $LATEST_URL"
curl -L -o "$INSTALL_DIR/doto" "$LATEST_URL"

# Make executable
chmod +x "$INSTALL_DIR/doto"

echo -e "${GREEN}✓ Doto installed successfully!${NC}"
echo ""
echo -e "${YELLOW}Add to PATH (if not already added):${NC}"
echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
echo ""
echo -e "${GREEN}Run 'doto' to start using your terminal todo app!${NC}"
