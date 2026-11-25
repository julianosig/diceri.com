#!/usr/bin/env bash
# Installs the exact clang that LiveKit's prebuilt libwebrtc was built with into
# .toolchain/clang (gitignored). The system clang is too old for its libc++.
# Usage: scripts/install-webrtc-clang.sh [revision]
set -euo pipefail

# Keep in sync with CR_CLANG_REVISION in the libwebrtc artifact's webrtc.ninja
# (target/*/build/scratch/*/out/livekit_webrtc/**/webrtc.ninja).
readonly REVISION="${1:-llvmorg-23-init-10931-g20b6ec66-11}"
readonly URL="https://commondatastorage.googleapis.com/chromium-browser-clang/Linux_x64/clang-${REVISION}.tar.xz"

cd "$(git rev-parse --show-toplevel)"
readonly DEST=".toolchain/clang"

if [[ -f "$DEST/REVISION" && "$(cat "$DEST/REVISION")" == "$REVISION" ]]; then
	echo "clang $REVISION already installed in $DEST"
	exit 0
fi

rm -rf "$DEST"
mkdir -p "$DEST"
echo "downloading clang $REVISION…"
curl -fsSL "$URL" | tar -xJ -C "$DEST"
echo "$REVISION" >"$DEST/REVISION"
"$DEST/bin/clang++" --version | head -1
