#!/usr/bin/env bash
# Packages Craft Anything as an Android APK using cargo-apk
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

echo "==> Building Craft Anything Android APK"
TARGET="${1:-aarch64-linux-android}"

cargo apk build --release -p craft-anything-android --target "$TARGET"
echo "==> APK generated successfully!"
find target -name "*.apk"
