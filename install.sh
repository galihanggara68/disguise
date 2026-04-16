#!/bin/bash
set -e

# Disguise Installation Script
# This script detects the OS and architecture, downloads the latest release of disguise-rs,
# and installs it to /usr/local/bin.

REPO="galihanggara68/disguise"
BINARY_NAME="disguise"
RELEASE_BINARY="disguise-rs"

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
  linux)
    TARGET="x86_64-unknown-linux-gnu"
    ;;
  darwin)
    if [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
      TARGET="aarch64-apple-darwin"
    else
      TARGET="x86_64-apple-darwin"
    fi
    ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

URL="https://github.com/$REPO/releases/latest/download/${RELEASE_BINARY}-${TARGET}"

echo "Detecting OS: $OS ($ARCH)"
echo "Targeting: $TARGET"
echo "Downloading latest release from $URL..."

if ! curl -L "$URL" -o "$BINARY_NAME"; then
    echo "Error: Failed to download binary from $URL"
    exit 1
fi

chmod +x "$BINARY_NAME"

echo "Installing $BINARY_NAME to /usr/local/bin (requires sudo)..."
if sudo mv "$BINARY_NAME" /usr/local/bin/; then
    echo "Successfully installed $BINARY_NAME to /usr/local/bin/$BINARY_NAME"
    echo "Run 'disguise --help' to get started!"
else
    echo "Error: Failed to move $BINARY_NAME to /usr/local/bin/"
    exit 1
fi
