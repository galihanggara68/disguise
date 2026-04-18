#!/bin/bash
set -e

# Disguise Installation Script
# This script detects the OS and architecture, downloads the latest release of disguise-rs,
# extracts it, and installs it to a bin directory.

REPO="galihanggara68/disguise"
BINARY_NAME="disguise"
RELEASE_NAME="disguise-rs"

OS_NAME="$(uname -s)"
ARCH="$(uname -m)"

case "${OS_NAME}" in
    Linux*)     OS='linux';;
    Darwin*)    OS='darwin';;
    MSYS*|MINGW*|CYGWIN*) OS='windows';;
    *)          OS='unknown';;
esac

if [ "$OS" = "unknown" ]; then
    echo "Unsupported OS: ${OS_NAME}"
    exit 1
fi

case "${ARCH}" in
    x86_64|amd64) ARCH_TARGET="x86_64";;
    arm64|aarch64) ARCH_TARGET="aarch64";;
    *)             ARCH_TARGET="x86_64";; # Default to x64
esac

# Determine target triple and extension
if [ "$OS" = "linux" ]; then
    TARGET="x86_64-unknown-linux-gnu"
    EXT="tar.gz"
elif [ "$OS" = "darwin" ]; then
    if [ "$ARCH_TARGET" = "aarch64" ]; then
        TARGET="aarch64-apple-darwin"
    else
        TARGET="x86_64-apple-darwin"
    fi
    EXT="tar.gz"
elif [ "$OS" = "windows" ]; then
    TARGET="x86_64-pc-windows-msvc"
    EXT="zip"
fi

URL="https://github.com/$REPO/releases/latest/download/${RELEASE_NAME}-${TARGET}.${EXT}"
TEMP_DIR=$(mktemp -d)
DOWNLOAD_PATH="${TEMP_DIR}/${RELEASE_NAME}.${EXT}"

echo "Detecting OS: ${OS_NAME} (${ARCH})"
echo "Targeting: $TARGET"
echo "Downloading latest release from $URL..."

if ! curl -L "$URL" -o "$DOWNLOAD_PATH"; then
    echo "Error: Failed to download from $URL"
    exit 1
fi

echo "Extracting archive..."
cd "$TEMP_DIR"
if [ "$EXT" = "tar.gz" ]; then
    tar -xzf "$DOWNLOAD_PATH"
elif [ "$EXT" = "zip" ]; then
    if command -v unzip >/dev/null 2>&1; then
        unzip "$DOWNLOAD_PATH"
    else
        # Fallback to PowerShell for Windows if unzip is missing
        powershell.exe -Command "Expand-Archive -Path '${DOWNLOAD_PATH}' -DestinationPath '${TEMP_DIR}'"
    fi
fi

# Determine binary filename (Windows has .exe)
SRC_BIN="${RELEASE_NAME}"
[ "$OS" = "windows" ] && SRC_BIN="${RELEASE_NAME}.exe"

if [ ! -f "$SRC_BIN" ]; then
    # Sometimes actions package them in a subfolder or with different name
    # Search for it
    SRC_BIN=$(find . -name "${RELEASE_NAME}*" -type f | head -n 1)
fi

if [ -z "$SRC_BIN" ]; then
    echo "Error: Could not find binary in extracted archive"
    exit 1
fi

chmod +x "$SRC_BIN"

# Installation destination
if [ "$OS" = "windows" ]; then
    # For Windows in Bash, install to current directory or a standard local path
    INSTALL_DIR="/usr/local/bin"
    [ ! -d "$INSTALL_DIR" ] && INSTALL_DIR="$HOME/bin"
    mkdir -p "$INSTALL_DIR"
    cp "$SRC_BIN" "$INSTALL_DIR/${BINARY_NAME}.exe"
    echo "Installed to $INSTALL_DIR/${BINARY_NAME}.exe"
else
    # Linux/macOS
    INSTALL_DIR="/usr/local/bin"
    if [ -w "$INSTALL_DIR" ]; then
        mv "$SRC_BIN" "$INSTALL_DIR/$BINARY_NAME"
    else
        echo "Requires sudo to install to $INSTALL_DIR"
        sudo mv "$SRC_BIN" "$INSTALL_DIR/$BINARY_NAME"
    fi
    echo "Successfully installed $BINARY_NAME to $INSTALL_DIR/$BINARY_NAME"
fi

# Cleanup
rm -rf "$TEMP_DIR"

echo "Done! Run '$BINARY_NAME --help' to verify installation."
