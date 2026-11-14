## 📱 Craft Anything Android APK (v0.4.9)

Official automated Android release build by **RedGlitchX**.

### 🚀 What's New in v0.4.9:
- **Package Validation & Installation Fix ("Package appears to be invalid" / Parse Error)**:
  - Explicitly enabled `extractNativeLibs="true"` in both `AndroidManifest.xml` and Cargo Android package metadata. On Android 10+ (API 29-35), compressed native libraries with `extractNativeLibs="false"` cause installation rejection (`INSTALL_FAILED_INVALID_APK`).
  - Added multi-density launcher icons (`res/mipmap-mdpi`, `res/mipmap-hdpi`, `res/mipmap-xhdpi`, `res/mipmap-xxhdpi`, `res/mipmap-xxxhdpi`) so Android launcher parsers on all screen densities resolve icon assets cleanly.
  - Injected `windowSoftInputMode="adjustResize|stateHidden"` directly into the `cargo-apk` activity metadata to ensure the soft keyboard smoothly resizes views.
  - Signed with canonical permanent `release.keystore` supporting v1 (JAR), v2, and v3 signature schemes.
- **Native Android JNI Clipboard (Direct System Copy/Paste)**:
  - Direct system clipboard access via JNI (`ClipboardManager`), enabling instant paste into prompts, API keys, and endpoints.
- **Mobile Keyboard & Viewport Fix**:
  - Soft keyboard no longer clips or pushes edit controls out of view (`adjustResize`).
- **Dedicated Private Cloud Build Repo & Copyright**:
  - Create and manage private GitHub build repo with 1 tap.
  - Attributed and copyrighted by **RedGlitchX (@tubers9999)**.

> [!IMPORTANT]
> **INSTALLATION NOTE:**
> If you have a previous version installed on your device (v0.4.8, v0.4.7, or earlier), **please uninstall the old app first** before installing this APK! Android blocks package updates when the package structure, native libraries, or signing certificate change (`INSTALL_FAILED_UPDATE_INCOMPATIBLE`).

- **Author & Copyright:** RedGlitchX ([@tubers9999](https://github.com/tubers9999))
- **License:** Dual-licensed MIT / Apache 2.0
- **Target Architecture:** `aarch64-linux-android` (ARM64)
- **Min Android SDK:** Android 8.0+ (API 26)
- **Signatures:** v1 (JAR) + v2 (APK Signature v2) + v3 (APK Signature v3)
- **Alignment:** 4-byte page aligned (zipalign)

### ⬇️ Download APK
- **[craft-anything-andriod.apk](https://github.com/tubers9999/craft-anything-andriod/releases/download/v0.4.9-android/craft-anything-andriod.apk)**
