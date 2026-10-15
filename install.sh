#!/usr/bin/env bash
set -euo pipefail

REPO="dogagezer/tusk"
BIN="tusk"
ASSET=""

OS="$(uname -s)"
ARCH="$(uname -m)"
VERSION="${VERSION:-latest}"

case "$OS" in
  Linux)
    case "$ARCH" in
      x86_64) ASSET="tusk-linux-x86_64" ;;
      aarch64|arm64) ASSET="tusk-linux-aarch64" ;;
      *) echo "Unsupported Linux architecture: $ARCH" >&2; exit 1 ;;
    esac
    VERSION="${VERSION:-latest}"
    if [ "$VERSION" = "latest" ]; then
      DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${ASSET}"
    else
      DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ASSET}"
    fi
    ;;
  Darwin)
    # A universal macOS binary is committed in the repository, so installation
    # works even before GitHub release assets are published.
    DOWNLOAD_URL="https://raw.githubusercontent.com/${REPO}/main/releases/tusk-macos-universal"
    ;;
  *)
    echo "Unsupported OS: $OS" >&2
    exit 1
    ;;
esac

INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
if [ ! -w "$INSTALL_DIR" ]; then
  INSTALL_DIR="$HOME/.local/bin"
  mkdir -p "$INSTALL_DIR"
fi

echo "Downloading ${ASSET} ${VERSION}..."
TMP_FILE="$(mktemp)"
curl -fsSL "$DOWNLOAD_URL" -o "$TMP_FILE"
chmod +x "$TMP_FILE"
mv "$TMP_FILE" "${INSTALL_DIR}/${BIN}"

echo "Installed ${BIN} to ${INSTALL_DIR}/${BIN}"
echo "Make sure ${INSTALL_DIR} is on your PATH."