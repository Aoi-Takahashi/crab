#!/usr/bin/env bash
# install.sh - Crab Credential Manager installer

set -euo pipefail

# Configuration
REPO="Aoi-Takahashi/crab"
BINARY_NAME="crab"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

# Detect platform
detect_platform() {
    local os
    local arch
    
    os=$(uname -s | tr '[:upper:]' '[:lower:]')
    arch=$(uname -m)
    
    case "$os" in
        linux*)
            os="linux"
            ;;
        darwin*)
            os="macos"
            ;;
        mingw* | msys* | cygwin*)
            os="windows"
            ;;
        *)
            error "Unsupported OS: $os"
            exit 1
            ;;
    esac
    
    case "$arch" in
        x86_64 | amd64)
            arch="x86_64"
            ;;
        aarch64 | arm64)
            arch="aarch64"
            ;;
        *)
            error "Unsupported architecture: $arch"
            exit 1
            ;;
    esac
    
    echo "${os}-${arch}"
}

# Get latest release version
get_latest_version() {
    local api_url="https://api.github.com/repos/${REPO}/releases/latest"
    
    if command -v curl >/dev/null 2>&1; then
        curl -s "$api_url" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "$api_url" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
    else
        error "Neither curl nor wget is available"
        exit 1
    fi
}

# Download and install binary
install_binary() {
    local platform="$1"
    local version="$2"
    local download_url
    local tmp_dir
    local archive_name
    local binary_path
    
    case "$platform" in
        *windows*)
            archive_name="${BINARY_NAME}-${platform}.zip"
            binary_path="${BINARY_NAME}.exe"
            ;;
        *)
            archive_name="${BINARY_NAME}-${platform}.tar.gz"
            binary_path="${BINARY_NAME}"
            ;;
    esac
    
    download_url="https://github.com/${REPO}/releases/download/${version}/${archive_name}"
    tmp_dir=$(mktemp -d)
    
    info "Downloading ${archive_name}..."
    
    if command -v curl >/dev/null 2>&1; then
        curl -L -o "${tmp_dir}/${archive_name}" "$download_url"
    elif command -v wget >/dev/null 2>&1; then
        wget -O "${tmp_dir}/${archive_name}" "$download_url"
    else
        error "Neither curl nor wget is available"
        exit 1
    fi
    
    info "Extracting archive..."
    cd "$tmp_dir"
    
    case "$archive_name" in
        *.zip)
            if command -v unzip >/dev/null 2>&1; then
                unzip -q "$archive_name"
            else
                error "unzip is not available"
                exit 1
            fi
            ;;
        *.tar.gz)
            tar -xzf "$archive_name"
            ;;
    esac
    
    # Create install directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"
    
    # Install binary
    info "Installing to ${INSTALL_DIR}/${BINARY_NAME}..."
    mv "$binary_path" "${INSTALL_DIR}/${BINARY_NAME}"
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    
    # Cleanup
    cd -
    rm -rf "$tmp_dir"
    
    success "Installation completed!"
}

# Check if install directory is in PATH
check_path() {
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        warn "⚠️  ${INSTALL_DIR} is not in your PATH"
        echo ""
        echo "To use crab from anywhere, add this to your shell profile:"
        echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
        echo ""
        echo "Shell profiles:"
        echo "  • Bash: ~/.bashrc or ~/.bash_profile"
        echo "  • Zsh: ~/.zshrc"
        echo "  • Fish: ~/.config/fish/config.fish"
        echo ""
    fi
}

# Main installation function
main() {
    echo "🦀 Crab Credential Manager Installer"
    echo ""
    
    # Detect platform
    local platform
    platform=$(detect_platform)
    info "Detected platform: $platform"
    
    # Get latest version
    local version
    version=$(get_latest_version)
    info "Latest version: $version"
    
    # Check if already installed
    if command -v "$BINARY_NAME" >/dev/null 2>&1; then
        local current_version
        current_version=$("$BINARY_NAME" --version 2>/dev/null | head -n1 | awk '{print $NF}' || echo "unknown")
        warn "crab is already installed (version: $current_version)"
        
        read -p "Do you want to continue? [y/N]: " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            info "Installation cancelled"
            exit 0
        fi
    fi
    
    # Install binary
    install_binary "$platform" "$version"
    
    # Check PATH
    check_path
    
    echo ""
    success "🎉 Crab has been installed successfully!"
    echo ""
    echo "Try running: $BINARY_NAME --help"
    echo ""
    echo "To get started:"
    echo "  $BINARY_NAME add --service github --account yourusername"
    echo ""
}

# Handle command line arguments
case "${1:-}" in
    --help|-h)
        echo "Crab Credential Manager Installer"
        echo ""
        echo "Usage: $0 [OPTIONS]"
        echo ""
        echo "Options:"
        echo "  --help, -h    Show this help message"
        echo ""
        echo "Environment variables:"
        echo "  INSTALL_DIR   Installation directory (default: ~/.local/bin)"
        echo ""
        exit 0
        ;;
    *)
        main "$@"
        ;;
esac