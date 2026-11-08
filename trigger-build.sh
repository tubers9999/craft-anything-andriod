#!/usr/bin/env bash
# Triggers the Android APK build workflow on GitHub Actions
set -euo pipefail

echo "==> Triggering Android APK build workflow on GitHub..."
gh workflow run android.yml

echo "==> Workflow triggered successfully!"
echo "Check progress with:"
echo "  gh run list --workflow=android.yml"
echo "  gh run watch"
