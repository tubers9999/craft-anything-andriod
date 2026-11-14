## 📱 Craft Anything Android APK (v0.4.10)

Official automated Android release build by **RedGlitchX**.

### 🚀 What's New in v0.4.10:
- **Startup Crash Fix ("Craft Anything keeps stopping" / Runtime Abort)**:
  - Eliminated redundant `ndk_context::initialize_android_context` call in `android_main` that conflicted with `android-activity`'s internal initialization, which was triggering an assertion panic on launch.
  - Added a global panic handler to route any unhandled errors directly to Android's system logcat (`log::error!`).
  - Wrapped all clipboard JNI interactions in `catch_unwind` safeguards to prevent background crashes.
- **Package Validation & Installation**:
  - Permanent `release.keystore` with v2 and v3 signatures.
  - `extractNativeLibs="true"` for full Android 8.0 through Android 15 compatibility.
  - Multi-density launcher icons (`mdpi`, `hdpi`, `xhdpi`, `xxhdpi`, `xxxhdpi`).
  - Soft keyboard `adjustResize` support.
- **Native System Clipboard & Cloud Builds**:
  - Instant copy/paste from external apps directly into prompts and settings.
  - One-click GitHub private build repository creation.

> [!IMPORTANT]
> **INSTALLATION NOTE:**
> If you have a previous version installed on your device (v0.4.9, v0.4.8, or earlier), **please uninstall the old app first** before installing this APK! Android blocks package updates when the package structure, native libraries, or signing certificate change (`INSTALL_FAILED_UPDATE_INCOMPATIBLE`).

- **Author & Copyright:** RedGlitchX ([@tubers9999](https://github.com/tubers9999))
- **License:** Dual-licensed MIT / Apache 2.0
- **Target Architecture:** `aarch64-linux-android` (ARM64)
- **Min Android SDK:** Android 8.0+ (API 26)
- **Signatures:** v1 (JAR) + v2 (APK Signature v2) + v3 (APK Signature v3)
- **Alignment:** 4-byte page aligned (zipalign)

### ⬇️ Download APK
- **[craft-anything-andriod.apk](https://github.com/tubers9999/craft-anything-andriod/releases/download/v0.4.10-android/craft-anything-andriod.apk)**
