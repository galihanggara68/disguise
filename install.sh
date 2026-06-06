#!/bin/bash
set -e

# Disguise Installation Script
# This script detects the OS and architecture, downloads the latest release of disguise-rs,
# extracts it, and installs it to a bin directory.

REPO="galihanggara68/disguise"
BINARY_NAME="disguise"
RELEASE_NAME="disguise-rs"

# Check if already installed before doing anything
if command -v disguise >/dev/null 2>&1; then
    ALREADY_INSTALLED=true
else
    ALREADY_INSTALLED=false
fi

# Spinner function for visual feedback
spinner() {
    local pid=$!
    local delay=0.1
    local spinstr='|/-\'
    while [ "$(ps a | awk '{print $1}' | grep $pid)" ]; do
        local temp=${spinstr#?}
        printf " [%c]  " "$spinstr"
        local spinstr=$temp${spinstr%"$temp"}
        sleep $delay
        printf "\b\b\b\b\b\b"
    done
    printf "    \b\b\b\b\b"
}

echo "--- Disguise Installation ---"

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

printf "Detecting environment: %s (%s)..." "${OS_NAME}" "${ARCH}"
(sleep 0.5) & spinner
echo " Done."

echo "Downloading latest release..."
if ! curl -L --progress-bar "$URL" -o "$DOWNLOAD_PATH"; then
    echo "Error: Failed to download from $URL"
    exit 1
fi

printf "Extracting archive..."
cd "$TEMP_DIR" > /dev/null
(
    if [ "$EXT" = "tar.gz" ]; then
        tar -xzf "$DOWNLOAD_PATH"
    elif [ "$EXT" = "zip" ]; then
        if command -v unzip >/dev/null 2>&1; then
            unzip -q "$DOWNLOAD_PATH"
        else
            powershell.exe -Command "Expand-Archive -Path '${DOWNLOAD_PATH}' -DestinationPath '${TEMP_DIR}'" > /dev/null 2>&1
        fi
    fi
) & spinner
echo " Done."

# Determine binary filename (Windows has .exe)
SRC_BIN="${RELEASE_NAME}"
[ "$OS" = "windows" ] && SRC_BIN="${RELEASE_NAME}.exe"

if [ ! -f "$SRC_BIN" ]; then
    SRC_BIN=$(find . -name "${RELEASE_NAME}*" -type f | head -n 1)
fi

if [ -z "$SRC_BIN" ]; then
    echo "Error: Could not find binary in extracted archive"
    exit 1
fi

chmod +x "$SRC_BIN"

printf "Installing $BINARY_NAME..."
(
    if [ "$OS" = "windows" ]; then
        INSTALL_DIR="/usr/local/bin"
        [ ! -d "$INSTALL_DIR" ] && INSTALL_DIR="$HOME/bin"
        mkdir -p "$INSTALL_DIR"
        FINAL_BIN="$INSTALL_DIR/${BINARY_NAME}.exe"
        cp "$SRC_BIN" "$FINAL_BIN"
    else
        INSTALL_DIR="/usr/local/bin"
        FINAL_BIN="$INSTALL_DIR/$BINARY_NAME"
        if [ -w "$INSTALL_DIR" ]; then
            mv "$SRC_BIN" "$FINAL_BIN"
        else
            sudo mv "$SRC_BIN" "$FINAL_BIN"
        fi
    fi
) & spinner
echo " Done."

# Cleanup
rm -rf "$TEMP_DIR"

# Only ask for shell setup if it was not already installed
if [ "$ALREADY_INSTALLED" = false ]; then
    echo ""
    read -p "Do you want to set up autocomplete and 'dx' alias? (y/N): " setup_shell
    if [[ "$setup_shell" =~ ^[Yy]$ ]]; then
        echo "Select your shell:"
        echo "1) bash"
        echo "2) zsh"
        echo "3) fish"
        read -p "Enter choice [1-3]: " shell_choice

        case $shell_choice in
            1) SELECTED_SHELL="bash"; PROFILE="$HOME/.bashrc";;
            2) SELECTED_SHELL="zsh"; PROFILE="$HOME/.zshrc";;
            3) SELECTED_SHELL="fish"; PROFILE="$HOME/.config/fish/config.fish";;
            *) echo "Invalid choice, skipping shell setup."; SELECTED_SHELL="";;
        esac

        if [ -n "$SELECTED_SHELL" ]; then
            printf "Downloading complete_alias..."
            (curl -sSL https://raw.githubusercontent.com/cykerway/complete-alias/refs/heads/master/complete_alias -o "$HOME/.complete_alias") & spinner
            echo " Done."

            case $SELECTED_SHELL in
                bash)
                    cat <<EOF >> "$PROFILE"

# Disguise Setup
alias dx='disguise run'
source "\$HOME/.complete_alias"
complete -F _complete_alias dx
source <(disguise completions bash)
EOF
                    ;;
                zsh)
                    cat <<EOF >> "$PROFILE"

# Disguise Setup
alias dx='disguise run'
autoload -U +X bashcompinit && bashcompinit
source "\$HOME/.complete_alias"
complete -F _complete_alias dx
source <(disguise completions zsh)
EOF
                    ;;
                fish)
                    mkdir -p "$(dirname "$PROFILE")"
                    cat <<EOF >> "$PROFILE"

# Disguise Setup
alias dx='disguise run'
disguise completions fish | source
EOF
                    ;;
            esac
            echo "Shell profile $PROFILE updated. Please restart your shell or source it."
        fi
    fi
fi

# Add the 'update' script to Disguise (always re-register to ensure it's up to date)
if command -v disguise >/dev/null 2>&1; then
    printf "Registering 'update' script in Disguise..."
    (disguise add -n update -c '
    printf "Checking for updates..."
    (latest=$(curl -s https://api.github.com/repos/galihanggara68/disguise/releases/latest | grep tag_name | cut -d "\"" -f 4)
    echo ""
    echo "Latest version found: $latest") &
    pid=$!
    spinstr="|/-\"
    while [ "$(ps a | awk "{print \$1}" | grep $pid)" ]; do
        temp=${spinstr#?}
        printf " [%c]  " "$spinstr"
        spinstr=$temp${spinstr%"$temp"}
        sleep 0.1
        printf "\b\b\b\b\b\b"
    done
    printf "    \b\b\b\b\b"
    echo " Done."
    curl -sSL https://raw.githubusercontent.com/galihanggara68/disguise/main/install.sh | bash
    ' -d "Update Disguise to the latest version" --tags "system,update" --force || true) & spinner
    echo " Done."
fi

echo ""
echo "Done! Run '$BINARY_NAME --help' to verify installation."
