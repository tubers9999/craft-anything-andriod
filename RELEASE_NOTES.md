## 📱 Craft Anything Android APK (v0.4.13)

Official automated Android release build by **RedGlitchX**.

### 🚀 What's New in v0.4.13:
- **Direct System Clipboard Paste on Android**:
  - Registered Java Main UI Thread runner (`Looper.getMainLooper()`) via `AndroidApp::run_on_java_main_thread` for all Android clipboard reading & writing operations.
  - Calling `getPrimaryClip()` on the Java main UI thread guarantees active window focus and eliminates Android 10+ background thread security blocks.
  - Removed restrictive and unreliable `hasPrimaryClip()` gate that caused OEM clipboard queries to silently fail or return false.
  - Multi-tier extraction: directly coerces `ClipData.Item` via `coerceToText(Context)`, `getText()`, and URI conversion to plain text.
  - Tapping **"Paste"** anywhere (Endpoint, API Key, Chat Prompt, Server Address, GitHub Token) now **directly inserts the clipboard text instantly** with zero intermediate popups!
- **Fixed Soft Keyboard Glitches & Screen Distortion**:
  - Switched `windowSoftInputMode` from destructive `adjustResize` to stable `adjustPan|stateHidden` in `AndroidManifest.xml`.
  - Prevents native EGL/Vulkan surface destruction, viewport squishing, and key stroke drops when opening or dismissing the Android soft keyboard.
- **Enhanced Endpoint URL Editing & Auto-Save**:
  - Editable Endpoint URL field in backend cards now auto-saves on losing focus / Enter key.
  - Dedicated "Save" button is always responsive to persist immediate endpoint corrections.
- **Automatic Model Detection**:
  - Dedicated "Detect Models" button in "Add AI Server" queries `GET /v1/models` and populates the ComboBox dropdown with available models.

> [!IMPORTANT]
> **INSTALLATION NOTE:**
> If you have a previous version installed on your device (v0.4.12, v0.4.11, or earlier), **please uninstall the old app first** before installing this APK! Android blocks package updates when the package structure, native libraries, or signing certificate change (`INSTALL_FAILED_UPDATE_INCOMPATIBLE`).

- **Author & Copyright:** RedGlitchX ([@tubers9999](https://github.com/tubers9999))
- **License:** Dual-licensed MIT / Apache 2.0
- **Target Architecture:** `aarch64-linux-android` (ARM64)
- **Min Android SDK:** Android 8.0+ (API 26)
- **Signatures:** v1 (JAR) + v2 (APK Signature v2) + v3 (APK Signature v3)
- **Alignment:** 4-byte page aligned (zipalign)

### ⬇️ Download APK
- **[craft-anything-andriod.apk](https://github.com/tubers9999/craft-anything-andriod/releases/download/v0.4.13-android/craft-anything-andriod.apk)**
