//! Craft Anything Android entry point.
//! Copyright (c) 2026 RedGlitchX. All rights reserved.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

#[cfg(target_os = "android")]
use craft_anything_ai::AiHub;
#[cfg(target_os = "android")]
use craft_anything_engine::{Engine, Session, Settings};
#[cfg(target_os = "android")]
use craft_anything_ui_egui::CraftApp;
#[cfg(target_os = "android")]
use std::sync::Arc;

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

/// Android Activity entry point invoked by android-activity / NativeActivity.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("CraftAnything"),
    );

    // Initialize ndk_context so JNI helpers (native Android clipboard, system services) can access the JavaVM and Activity.
    unsafe {
        ndk_context::initialize_android_context(app.vm_as_ptr().cast(), app.activity_as_ptr().cast());
    }

    log::info!("Craft Anything starting on Android (v{})", env!("CARGO_PKG_VERSION"));

    // Set internal app directory as working directory and HOME so settings and databases persist properly
    if let Some(path) = app.internal_data_path() {
        let _ = std::fs::create_dir_all(&path);
        let _ = std::env::set_current_dir(&path);
        // SAFETY: Only set HOME at startup before background threads spawn.
        unsafe {
            std::env::set_var("HOME", &path);
            std::env::set_var("CRAFT_ANYTHING_CONFIG_DIR", &path);
        }
        log::info!("Configured app data directory: {}", path.display());
    }

    // Enhance PATH with Termux and PRoot Ubuntu locations so coding agents (claude, codex, gemini) and tools are detected
    let termux_bins = [
        "/data/data/com.termux/files/usr/bin",
        "/data/data/com.termux/files/usr/bin/applets",
        "/data/data/com.termux/files/home/.local/bin",
        "/data/data/com.termux/files/home/.cargo/bin",
        "/data/data/com.termux/files/home/.npm-global/bin",
        "/data/data/com.termux/files/usr/var/lib/proot-distro/installed-rootfs/ubuntu/usr/bin",
        "/data/data/com.termux/files/usr/var/lib/proot-distro/installed-rootfs/ubuntu/bin",
        "/data/data/com.termux/files/usr/var/lib/proot-distro/installed-rootfs/ubuntu/usr/local/bin",
        "/data/data/com.termux/files/usr/var/lib/proot-distro/installed-rootfs/ubuntu/root/.local/bin",
        "/data/data/com.termux/files/usr/var/lib/proot-distro/installed-rootfs/ubuntu/root/.cargo/bin",
        "/system/bin",
        "/system/xbin",
    ];
    let cur_path = std::env::var("PATH").unwrap_or_default();
    let augmented_path = format!("{}:{}", termux_bins.join(":"), cur_path);
    unsafe {
        std::env::set_var("PATH", &augmented_path);
    }

    // Set fallback CLAUDE_CONFIG_DIR if not already set, checking PRoot and Termux home
    if std::env::var_os("CLAUDE_CONFIG_DIR").is_none() {
        let proot_claude = std::path::Path::new("/data/data/com.termux/files/usr/var/lib/proot-distro/installed-rootfs/ubuntu/root/.claude");
        let termux_claude = std::path::Path::new("/data/data/com.termux/files/home/.claude");
        if proot_claude.exists() {
            unsafe { std::env::set_var("CLAUDE_CONFIG_DIR", proot_claude); }
            log::info!("Detected Claude Code config in PRoot Ubuntu: {}", proot_claude.display());
        } else if termux_claude.exists() {
            unsafe { std::env::set_var("CLAUDE_CONFIG_DIR", termux_claude); }
            log::info!("Detected Claude Code config in Termux: {}", termux_claude.display());
        }
    }

    let settings = Settings::load();
    let mut session = Session::new(settings);
    let plugins = craft_anything_engine::plugins::install(&mut session);
    let mut layer_tools = craft_anything_ai::tools();
    layer_tools.extend(craft_anything_setup::tools::specs());
    let engine = Engine::shared_with(session, layer_tools, plugins);
    let hub = Arc::new(AiHub::new(engine.clone()));
    log::info!("Craft Anything engine & AI hub initialized for Android");

    let viewport = eframe::egui::ViewportBuilder::default()
        .with_title("Craft Anything")
        .with_app_id("ai.storyteller.craftanything");

    let options = eframe::NativeOptions {
        viewport,
        android_app: Some(app),
        ..Default::default()
    };

    let result = eframe::run_native(
        "Craft Anything",
        options,
        Box::new(move |cc| {
            log::info!("Craft Anything surface active");
            CraftApp::setup_context(&cc.egui_ctx);
            let mut craft_app = CraftApp::new(engine, hub);
            craft_app.load_prefs(cc.storage);
            Ok(Box::new(craft_app))
        }),
    );

    if let Err(e) = result {
        log::error!("Craft Anything stopped with error: {e}");
    } else {
        log::info!("Craft Anything closed gracefully");
    }
}

/// Fallback / placeholder for non-Android targets (e.g. host cargo check/clippy).
#[cfg(not(target_os = "android"))]
pub fn main() {
    println!("craft-anything-android is targeted for Android (cdylib/android_main).");
}
