#!/usr/bin/env bash
# Downloads the latest built Android APK from GitHub Actions Artifacts
set -euo pipefail

echo "==> Fetching latest APK artifact from GitHub Actions..."
mkdir -p ./apk-output
gh run download --name craft-anything-andriod-apk --dir ./apk-output || {
    echo "Note: If no artifact was downloaded, ensure the GitHub Actions workflow has run first:"
    echo "  gh workflow run android.yml"
    exit 1
}

echo "==> Download complete! APK is located at:"
ls -lh ./apk-output/*.apk
