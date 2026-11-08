# Craft Anything for Android (`craft-anything-andriod`)

This repository adds Android APK packaging and automated CI/CD builds for **Craft Anything** using Rust, `eframe` (egui), `android-activity` (`GameActivity`), and `cargo-apk`.

---

## 🚀 How to Get the APK

### Option 1: Automatic Download from GitHub Actions
Every push to `main` and every manual workflow trigger builds a fresh Android APK artifact.

1. Go to the **Actions** tab on GitHub: [GitHub Actions Workflows](https://github.com/tubers9999/craft-anything-andriod/actions)
2. Click on the latest **Build Android APK** workflow run.
3. Under the **Artifacts** section at the bottom, download **`craft-anything-andriod-apk`**.
4. Unzip and install `craft-anything-andriod.apk` on your Android device!

---

### Option 2: Using the CLI on Termux / Root

If you have `gh` (GitHub CLI) installed:

```bash
# 1. Trigger an APK build in GitHub Actions:
./trigger-build.sh
# or: gh workflow run android.yml

# 2. Monitor the build progress:
gh run list --workflow=android.yml
gh run watch

# 3. Download the built APK directly to your current directory:
./download-apk.sh
# or: gh run download --name craft-anything-andriod-apk --dir ./
```

---

### Option 3: Building Locally on Linux / Termux

To build the APK locally, you need:
- Android SDK & NDK (API level 26+ / NDK r26+)
- Rust with `aarch64-linux-android` target
- `cargo-apk`:
  ```bash
  cargo install cargo-apk
  ```

Build command:
```bash
./packaging/android/build.sh
# or directly:
cargo apk build --release -p craft-anything-android
```
The output APK will be placed in `target/release/apk/` or `dist/`.

---

## 🛠️ Architecture & Changes

- **Android App Crate:** Located in `apps/craft-anything-android/`.
  - Crate type: `cdylib` and `rlib`.
  - Lifecycle: `winit` 0.30 + `android-activity` with Jetpack `GameActivity`.
  - UI: Seamlessly connects to `CraftApp` from `crates/ui-egui`.
  - Storage: Configures `app.internal_data_path()` so databases and settings persist in Android app storage.
- **Platform Compatibility:**
  - `arboard` (desktop clipboard) and `rfd` (desktop native file picker) are cleanly gated with `#[cfg(not(target_os = "android"))]` so Android builds compile smoothly without missing platform symbols.
  - Safe fallbacks for report saving to app data directory.
- **Workflow:** `.github/workflows/android.yml` automates the build pipeline on GitHub runners with pre-installed Android NDK, caching, and artifact uploads.
