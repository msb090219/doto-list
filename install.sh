#!/bin/bash

# Doto Installation Script for macOS and Linux
# This script downloads and installs the latest release of Doto

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
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
LATEST_URL=$(curl -s https://api.github.com/repos/msb090219/doto-list/releases/latest |
              grep "browser_download_url" |
              grep "$OS-$ARCH" |
              cut -d '"' -f 4)

if [ -z "$LATEST_URL" ]; then
    echo ""
    echo -e "${RED}Installation failed: Unable to find releases for doto-list${NC}"
    echo ""
    echo -e "${YELLOW}This could mean:${NC}"
    echo "  1. No releases have been created yet (check GitHub releases page)"
    echo "  2. The repository name has changed"
    echo "  3. Network connectivity issues"
    echo ""
    echo -e "${YELLOW}Please check the repository and try again.${NC}"
    echo -e "${CYAN}Repository: https://github.com/msb090219/doto-list${NC}"
    exit 1
fi

# Create installation directory
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

echo "Downloading from: $LATEST_URL"
curl -L -o "$INSTALL_DIR/doto" "$LATEST_URL"

# Make executable
chmod +x "$INSTALL_DIR/doto"

echo -e "${GREEN}✓ Doto installed to: $INSTALL_DIR${NC}"

# Detect shell and config file
SHELL_CONFIG=""
if [ -n "$ZSH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.bashrc"
else
    SHELL_CONFIG="$HOME/.profile"
fi

# Check if already in PATH
if echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo -e "${GREEN}✓ doto is already in your PATH!${NC}"
else
    echo -e "${YELLOW}Adding doto to your PATH...${NC}"

    # Add to shell config
    echo "" >> "$SHELL_CONFIG"
    echo "# Doto todo app" >> "$SHELL_CONFIG"
    echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$SHELL_CONFIG"

    echo -e "${GREEN}✓ Added to PATH in $SHELL_CONFIG${NC}"
    echo ""
    echo -e "${YELLOW}Please run: source $SHELL_CONFIG${NC}"
    echo -e "${YELLOW}Or restart your terminal for PATH changes to take effect.${NC}"
fi

echo ""
echo -e "${GREEN}Installation complete!${NC}"
echo -e "${CYAN}Run 'doto' to start using your terminal todo app!${NC}"
echo ""
echo -e "${YELLOW}If doto doesn't work immediately, restart your terminal or run:${NC}"
echo -e "${CYAN}  source $SHELL_CONFIG${NC}"
