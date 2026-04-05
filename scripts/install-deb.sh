#!/usr/bin/env bash
# Install Blink from a .deb URL (e.g. a GitHub Release asset).
#
# Examples:
#   curl -fsSL https://raw.githubusercontent.com/OWNER/blink/main/scripts/install-deb.sh | bash -s -- 'https://github.com/OWNER/blink/releases/download/v0.1.0/blink_0.1.0_amd64.deb'
#
# Or download first, then:
#   ./scripts/install-deb.sh ./blink_0.1.0_amd64.deb
#
set -euo pipefail

if [[ "${1:-}" == "" ]]; then
  echo "Usage: $0 <path-or-https-url-to.deb>" >&2
  exit 1
fi

SRC="$1"
TMP_DEB="$(mktemp /tmp/blink-install-XXXXXX.deb)"
cleanup() { rm -f "$TMP_DEB"; }
trap cleanup EXIT

if [[ "$SRC" == http://* || "$SRC" == https://* ]]; then
  echo "Downloading: $SRC"
  curl -fL --retry 3 --retry-delay 2 -o "$TMP_DEB" "$SRC"
else
  cp "$SRC" "$TMP_DEB"
fi

echo "Installing package (pulls in WebKitGTK, GTK, gio, gtk-launch if needed)…"
if command -v sudo >/dev/null 2>&1; then
  sudo apt-get install -y "$TMP_DEB"
else
  apt-get install -y "$TMP_DEB"
fi

echo "Done. Run \"blink\" from the app menu or terminal using ctrl + space."
