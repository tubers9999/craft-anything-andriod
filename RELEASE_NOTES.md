## 📱 Craft Anything Android APK (v0.4.12)

Official automated Android release build by **RedGlitchX**.

### 🚀 What's New in v0.4.12:
- **Automatic Model Detection for Endpoints & Servers**:
  - Added dedicated **"Detect Models"** button in the "Add an AI by its address" form.
  - Automatically queries `GET /v1/models` on the server endpoint (supporting OpenAI-compatible, Ollama, LM Studio, vLLM, LocalAI, DeepSeek, etc.) and lists all available models in an interactive dropdown selector.
  - Automatically selects the optimal chat model or first available model on detection, eliminating the need to type complex model identifiers on mobile keyboards.
  - Auto-fills the server name based on host/address if left blank.
  - Enhanced server validation to prevent strict chat-model name filtering from rejecting valid custom models.
- **Editable Endpoint URLs for Existing Backends**:
  - Added editable **"Endpoint URL"** field for every AI backend card in Settings ("Your brains").
  - Easily update or correct a backend's base URL / server address without deleting the backend and starting over.
  - Includes dedicated **"Paste endpoint"** and **"Save"** actions with live persistence and feedback.
  - Updating an endpoint immediately updates model catalog lookups when changing models.
- **Guaranteed Clipboard Paste on Android**:
  - Fixed Android JNI clipboard bridge by eliminating non-main-thread Looper issues and integrating multi-tier Context resolution (`NativeActivity`, `Application`, `ActivityThread.currentApplication`).
  - Added active `hasPrimaryClip()` inspection before extracting clip items.
  - Unblocked fallback paste input: tapping "Paste" anywhere in Settings or in the app will now **always** let you paste or type directly via an in-dialog input if the Android OS restricts background clipboard access.
  - Added **"From Clipboard"** action inside the paste dialog to easily capture copied text from Gboard or the system clipboard without losing Settings context.
- **All Previous Mobile Enhancements (v0.4.11 / v0.4.10)**:
  - Preserved Settings modal state during keyboard resize and touch events.
  - GitHub cloud builder integration for remote compilation of APKs, EXEs, and Web games.
  - Aligned native libraries (`extractNativeLibs="true"`) and v1 + v2 + v3 APK signing.

> [!IMPORTANT]
> **INSTALLATION NOTE:**
> If you have a previous version installed on your device (v0.4.11, v0.4.10, or earlier), **please uninstall the old app first** before installing this APK! Android blocks package updates when the package structure, native libraries, or signing certificate change (`INSTALL_FAILED_UPDATE_INCOMPATIBLE`).

- **Author & Copyright:** RedGlitchX ([@tubers9999](https://github.com/tubers9999))
- **License:** Dual-licensed MIT / Apache 2.0
- **Target Architecture:** `aarch64-linux-android` (ARM64)
- **Min Android SDK:** Android 8.0+ (API 26)
- **Signatures:** v1 (JAR) + v2 (APK Signature v2) + v3 (APK Signature v3)
- **Alignment:** 4-byte page aligned (zipalign)

### ⬇️ Download APK
- **[craft-anything-andriod.apk](https://github.com/tubers9999/craft-anything-andriod/releases/download/v0.4.12-android/craft-anything-andriod.apk)**
