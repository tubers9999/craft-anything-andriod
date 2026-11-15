## 📱 Craft Anything Android APK (v0.4.14)

Official automated Android release build by **RedGlitchX** ([@tubers9999](https://github.com/tubers9999)).

### 🚀 What's New in v0.4.14:
- **Background Execution with WakeLock ("Run when not in app")**:
  - Added `android.permission.WAKE_LOCK`, `android.permission.FOREGROUND_SERVICE`, and `REQUEST_IGNORE_BATTERY_OPTIMIZATIONS` in `AndroidManifest.xml` and `Cargo.toml`.
  - Implemented automated JNI `PARTIAL_WAKE_LOCK` acquisition on Android startup. The CPU remains awake and native Rust worker threads continue running builds, downloads, and AI tasks uninterrupted even when the user minimizes the app, switches to other apps, or turns off the screen.
- **Direct Media & Asset Extraction (No AI Image Generator Required)**:
  - Revised builder brief and system prompt instructions: builders are now explicitly instructed to extract, copy, or download original images, icons, sprites, audio, video, textures, and assets directly from the target website/game/app/evidence into `app/`.
  - Builders will no longer demand fal.ai API keys or external AI image generators, nor will they replace images with neutral placeholder gradients/boxes when real media files can be extracted or downloaded directly.
- **Fixed "Build Stopped After Two Builder Turns" Failure**:
  - Implemented smart automatic extraction of markdown code blocks into `app/` whenever `app/` has no files yet, ensuring foundational project scaffold files (`index.html`, `style.css`, `main.js`, `package.json`) exist immediately on disk even if a builder model outputted code directly into chat without invoking tool calls.
  - Fixed fenced tool calls execution in OpenAI/OpenRouter adapter: tool calls with trailing explanation text are no longer dropped or rejected with warning turns.
  - Updated builder instructions to mandate writing foundational project scaffold files in the very first turn.
- **Vertical Phone GUI Compatibility & Button Layout Overhaul**:
  - **Make screen**: On vertical portrait phone screens (< 450px), `Built by` and `Start` button are stacked vertically with `Start` taking full card width and 48px touch height, preventing button clipping and overflow.
  - **Settings screen**: Endpoint URL input and action buttons wrap with touch-friendly spacing and full-width mobile inputs.
  - **Chat Prompt Box**: Reserved space for Send and Pause buttons on narrow portrait screens by hiding redundant desktop shortcut hints.

> [!IMPORTANT]
> **INSTALLATION NOTE:**
> If you have a previous version installed on your device, **please uninstall the old app first** before installing this APK! Android blocks package updates when the package structure, native libraries, or signing certificate change (`INSTALL_FAILED_UPDATE_INCOMPATIBLE`).

- **Author & Copyright:** RedGlitchX ([@tubers9999](https://github.com/tubers9999))
- **License:** Dual-licensed MIT / Apache 2.0
- **Target Architecture:** `aarch64-linux-android` (ARM64)
- **Min Android SDK:** Android 8.0+ (API 26)
- **Signatures:** v1 (JAR) + v2 (APK Signature v2) + v3 (APK Signature v3)
- **Alignment:** 4-byte page aligned (zipalign)

### ⬇️ Download APK
- **[craft-anything-andriod.apk](https://github.com/tubers9999/craft-anything-andriod/releases/download/v0.4.14-android/craft-anything-andriod.apk)**
