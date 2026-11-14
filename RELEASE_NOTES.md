## 📱 Craft Anything Android APK (v0.4.11)

Official automated Android release build by **RedGlitchX**.

### 🚀 What's New in v0.4.11:
- **Direct System Clipboard Paste & Settings Fixes**:
  - Direct JNI clipboard access using active `NativeActivity` context alongside `Application` fallback.
  - Multi-tier clipboard text extraction: directly reads `ClipData.Item.getText()` before falling back to `coerceToText(context)`, ensuring instant paste from external apps (Chrome, GitHub, Notes, Gboard).
  - Preserves Settings modal: tapping "Paste" on API key or GitHub fields directly fills the input without disrupting or replacing Settings with secondary input dialogs.
  - Non-disruptive feedback: gracefully toasts if the clipboard is empty while keeping the current configuration screen intact.
- **Mobile Keyboard & UI Stability**:
  - Prevents accidental modal dismissals when the Android soft keyboard appears or resizes the viewport.
  - Expanded scrollable area on mobile screens to ensure all inputs, buttons, and switches remain easily accessible during typing.
- **GitHub Cloud Builder Integration**:
  - One-tap setup for private GitHub build repositories with automated APK, EXE, and Web workflows under RedGlitchX copyright.
- **All Previous Fixes (v0.4.10 / v0.4.9)**:
  - Startup crash / abort fix with safe `ndk_context` initialization and global panic hooks.
  - Valid v2 & v3 APK signatures with aligned native libraries (`extractNativeLibs="true"`).

> [!IMPORTANT]
> **INSTALLATION NOTE:**
> If you have a previous version installed on your device (v0.4.10, v0.4.9, or earlier), **please uninstall the old app first** before installing this APK! Android blocks package updates when the package structure, native libraries, or signing certificate change (`INSTALL_FAILED_UPDATE_INCOMPATIBLE`).

- **Author & Copyright:** RedGlitchX ([@tubers9999](https://github.com/tubers9999))
- **License:** Dual-licensed MIT / Apache 2.0
- **Target Architecture:** `aarch64-linux-android` (ARM64)
- **Min Android SDK:** Android 8.0+ (API 26)
- **Signatures:** v1 (JAR) + v2 (APK Signature v2) + v3 (APK Signature v3)
- **Alignment:** 4-byte page aligned (zipalign)

### ⬇️ Download APK
- **[craft-anything-andriod.apk](https://github.com/tubers9999/craft-anything-andriod/releases/download/v0.4.11-android/craft-anything-andriod.apk)**
