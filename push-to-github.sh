#!/usr/bin/env bash
# Push to GitHub repository tubers9999/craft-anything-andriod
set -euo pipefail

echo "==> Pushing to git@github.com:tubers9999/craft-anything-andriod.git..."
git push -u origin main

echo ""
echo "============================================================"
echo "  Successfully pushed to GitHub!"
echo "  View repository: https://github.com/tubers9999/craft-anything-andriod"
echo "  View APK build:  https://github.com/tubers9999/craft-anything-andriod/actions"
echo "============================================================"
