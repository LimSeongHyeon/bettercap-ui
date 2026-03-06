#!/usr/bin/env bash
set -euo pipefail

REPO="LimSeongHyeon/bettercap-ui"
APP_NAME="bettercap-ui"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info()  { echo -e "${GREEN}[+]${NC} $1"; }
warn()  { echo -e "${YELLOW}[!]${NC} $1"; }
error() { echo -e "${RED}[-]${NC} $1"; exit 1; }

# --- Checks ---
[[ "$EUID" -eq 0 ]] || error "Run as root: curl -sSL ... | sudo bash"
command -v dpkg >/dev/null || error "dpkg not found. This installer is for Debian/Kali/Ubuntu only."

ARCH=$(dpkg --print-architecture)
[[ "$ARCH" == "amd64" || "$ARCH" == "arm64" ]] || error "Unsupported architecture: $ARCH"

# --- Detect latest release ---
info "Fetching latest release..."
if command -v curl >/dev/null; then
    LATEST=$(curl -sSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | head -1 | cut -d'"' -f4)
elif command -v wget >/dev/null; then
    LATEST=$(wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | head -1 | cut -d'"' -f4)
else
    error "curl or wget required"
fi

[[ -n "$LATEST" ]] || error "No release found. Check https://github.com/${REPO}/releases"
info "Latest version: ${LATEST}"

# --- Download .deb ---
VERSION="${LATEST#v}"
DEB_NAME="${APP_NAME}_${VERSION}_${ARCH}.deb"
DEB_URL="https://github.com/${REPO}/releases/download/${LATEST}/${DEB_NAME}"

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

info "Downloading ${DEB_NAME}..."
if command -v curl >/dev/null; then
    curl -sSL -o "${TMPDIR}/${DEB_NAME}" "$DEB_URL" || error "Download failed. Check if release exists: ${DEB_URL}"
else
    wget -q -O "${TMPDIR}/${DEB_NAME}" "$DEB_URL" || error "Download failed. Check if release exists: ${DEB_URL}"
fi

# --- Install ---
info "Installing dependencies..."
apt-get update -qq
apt-get install -y -qq -f 2>/dev/null

info "Installing ${APP_NAME}..."
dpkg -i "${TMPDIR}/${DEB_NAME}" 2>/dev/null || apt-get install -y -qq -f

info "Done! Run with: ${APP_NAME}"
