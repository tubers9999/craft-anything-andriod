//! System clipboard reading & writing helper with Android and Termux support.

use std::sync::Mutex;

static IN_MEMORY_CLIPBOARD: Mutex<Option<String>> = Mutex::new(None);
static ANDROID_ACTIVITY: Mutex<Option<usize>> = Mutex::new(None);

/// Set text in the internal application clipboard cache.
pub fn set_in_memory_clipboard(text: &str) {
    if let Ok(mut g) = IN_MEMORY_CLIPBOARD.lock() {
        *g = Some(text.to_string());
    }
}

/// Get text from the internal application clipboard cache.
pub fn get_in_memory_clipboard() -> Option<String> {
    IN_MEMORY_CLIPBOARD.lock().ok().and_then(|g| g.clone()).filter(|s| !s.trim().is_empty())
}

/// Store the pointer to the Android NativeActivity jobject for direct JNI clipboard access.
pub fn set_android_activity(ptr: usize) {
    if let Ok(mut g) = ANDROID_ACTIVITY.lock() {
        *g = Some(ptr);
    }
}

/// Get the pointer to the Android NativeActivity jobject if available.
pub fn get_android_activity() -> Option<usize> {
    ANDROID_ACTIVITY.lock().ok().and_then(|g| *g)
}

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
use arboard::Clipboard;

/// Get plain text from the system clipboard.
pub fn get_text() -> Option<String> {
    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
    {
        if let Ok(mut cb) = Clipboard::new() {
            if let Ok(t) = cb.get_text() {
                if !t.is_empty() {
                    set_in_memory_clipboard(&t);
                    return Some(t);
                }
            }
        }
    }

    #[cfg(target_os = "android")]
    {
        if let Ok(Some(text)) = std::panic::catch_unwind(|| get_android_clipboard()) {
            if !text.trim().is_empty() {
                set_in_memory_clipboard(&text);
                return Some(text);
            }
        }
        if let Ok(Some(text)) = std::panic::catch_unwind(|| get_termux_clipboard()) {
            if !text.trim().is_empty() {
                set_in_memory_clipboard(&text);
                return Some(text);
            }
        }
    }

    get_in_memory_clipboard()
}

/// Read clipboard text directly from Android system ClipboardManager via JNI.
#[cfg(target_os = "android")]
fn get_android_clipboard() -> Option<String> {
    let ctx = match std::panic::catch_unwind(|| ndk_context::android_context()) {
        Ok(c) => c,
        Err(_) => return None,
    };
    let vm_ptr = ctx.vm();
    if vm_ptr.is_null() {
        return None;
    }
    unsafe {
        let vm = jni::JavaVM::from_raw(vm_ptr as *mut jni::sys::JavaVM).ok()?;
        let mut env = match vm.get_env() {
            Ok(e) => e,
            Err(_) => vm.attach_current_thread_permanently().ok()?,
        };

        if env.exception_check().unwrap_or(false) {
            let _ = env.exception_clear();
        }

        let Ok(service_name) = env.new_string("clipboard") else { return None };

        let mut contexts = Vec::new();
        if let Some(act_ptr) = get_android_activity() {
            if act_ptr != 0 {
                contexts.push(jni::objects::JObject::from_raw(act_ptr as jni::sys::jobject));
            }
        }
        let app_ptr = ctx.context();
        if !app_ptr.is_null() {
            let raw = app_ptr as jni::sys::jobject;
            if !contexts.iter().any(|c| c.as_raw() == raw) {
                contexts.push(jni::objects::JObject::from_raw(raw));
            }
        }
        if let Ok(at_class) = env.find_class("android/app/ActivityThread") {
            if let Ok(app_val) = env.call_static_method(&at_class, "currentApplication", "()Landroid/app/Application;", &[]) {
                if let Ok(app_obj) = app_val.l() {
                    if !app_obj.is_null() && !contexts.iter().any(|c| c.as_raw() == app_obj.as_raw()) {
                        contexts.push(app_obj);
                    }
                }
            }
            if env.exception_check().unwrap_or(false) {
                let _ = env.exception_clear();
            }
        }

        for context_obj in contexts {
            let cm = env.call_method(
                &context_obj,
                "getSystemService",
                "(Ljava/lang/String;)Ljava/lang/Object;",
                &[jni::objects::JValue::Object(&service_name)],
            );
            if env.exception_check().unwrap_or(false) {
                let _ = env.exception_clear();
                continue;
            }
            let Ok(cm_val) = cm.and_then(|v| v.l()) else {
                continue;
            };
            if cm_val.is_null() {
                continue;
            }

            // Check if there is primary clip
            let has_clip = env.call_method(&cm_val, "hasPrimaryClip", "()Z", &[]);
            if env.exception_check().unwrap_or(false) {
                let _ = env.exception_clear();
            } else if let Ok(val) = has_clip.and_then(|v| v.z()) {
                if !val {
                    continue;
                }
            }

            // Attempt getPrimaryClip directly
            let clip = env.call_method(&cm_val, "getPrimaryClip", "()Landroid/content/ClipData;", &[]);
            if env.exception_check().unwrap_or(false) {
                let _ = env.exception_clear();
                continue;
            }
            let Ok(clip_val) = clip.and_then(|v| v.l()) else {
                continue;
            };
            if clip_val.is_null() {
                continue;
            }

            let count = env
                .call_method(&clip_val, "getItemCount", "()I", &[])
                .ok()
                .and_then(|v| v.i().ok())
                .unwrap_or(0);
            if env.exception_check().unwrap_or(false) {
                let _ = env.exception_clear();
            }
            if count <= 0 {
                continue;
            }

            let item = env.call_method(
                &clip_val,
                "getItemAt",
                "(I)Landroid/content/ClipData$Item;",
                &[jni::objects::JValue::Int(0)],
            );
            if env.exception_check().unwrap_or(false) {
                let _ = env.exception_clear();
                continue;
            }
            let Ok(item_val) = item.and_then(|v| v.l()) else {
                continue;
            };
            if item_val.is_null() {
                continue;
            }

            // 1. Try item.getText() first (no Context required)
            let mut text_cs_obj = None;
            let text_call = env.call_method(&item_val, "getText", "()Ljava/lang/CharSequence;", &[]);
            if env.exception_check().unwrap_or(false) {
                let _ = env.exception_clear();
            } else if let Ok(val) = text_call.and_then(|v| v.l()) {
                if !val.is_null() {
                    text_cs_obj = Some(val);
                }
            }

            // 2. Fall back to item.coerceToText(context)
            if text_cs_obj.is_none() {
                let coerce_call = env.call_method(
                    &item_val,
                    "coerceToText",
                    "(Landroid/content/Context;)Ljava/lang/CharSequence;",
                    &[jni::objects::JValue::Object(&context_obj)],
                );
                if env.exception_check().unwrap_or(false) {
                    let _ = env.exception_clear();
                } else if let Ok(val) = coerce_call.and_then(|v| v.l()) {
                    if !val.is_null() {
                        text_cs_obj = Some(val);
                    }
                }
            }

            if let Some(cs) = text_cs_obj {
                let j_str = env.call_method(&cs, "toString", "()Ljava/lang/String;", &[]);
                if env.exception_check().unwrap_or(false) {
                    let _ = env.exception_clear();
                    continue;
                }
                if let Ok(j_str_val) = j_str.and_then(|v| v.l()) {
                    if !j_str_val.is_null() {
                        if let Ok(rust_str) = env.get_string(&jni::objects::JString::from(j_str_val)) {
                            let s: String = rust_str.into();
                            if !s.is_empty() {
                                log::info!("Read {} chars from Android clipboard", s.len());
                                return Some(s);
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

/// Set clipboard text directly on Android system ClipboardManager via JNI.
#[cfg(target_os = "android")]
fn set_android_clipboard(text: &str) {
    let ctx = match std::panic::catch_unwind(|| ndk_context::android_context()) {
        Ok(c) => c,
        Err(_) => return,
    };
    let vm_ptr = ctx.vm();
    if vm_ptr.is_null() {
        return;
    }
    unsafe {
        let Ok(vm) = jni::JavaVM::from_raw(vm_ptr as *mut jni::sys::JavaVM) else { return };
        let mut env = match vm.get_env() {
            Ok(e) => e,
            Err(_) => match vm.attach_current_thread_permanently() {
                Ok(e) => e,
                Err(_) => return,
            },
        };

        if env.exception_check().unwrap_or(false) {
            let _ = env.exception_clear();
        }

        let Ok(service_name) = env.new_string("clipboard") else { return };

        let mut contexts = Vec::new();
        if let Some(act_ptr) = get_android_activity() {
            if act_ptr != 0 {
                contexts.push(jni::objects::JObject::from_raw(act_ptr as jni::sys::jobject));
            }
        }
        let app_ptr = ctx.context();
        if !app_ptr.is_null() {
            let raw = app_ptr as jni::sys::jobject;
            if !contexts.iter().any(|c| c.as_raw() == raw) {
                contexts.push(jni::objects::JObject::from_raw(raw));
            }
        }
        if let Ok(at_class) = env.find_class("android/app/ActivityThread") {
            if let Ok(app_val) = env.call_static_method(&at_class, "currentApplication", "()Landroid/app/Application;", &[]) {
                if let Ok(app_obj) = app_val.l() {
                    if !app_obj.is_null() && !contexts.iter().any(|c| c.as_raw() == app_obj.as_raw()) {
                        contexts.push(app_obj);
                    }
                }
            }
            if env.exception_check().unwrap_or(false) {
                let _ = env.exception_clear();
            }
        }

        let Ok(j_text) = env.new_string(text) else { return };
        let Ok(j_label) = env.new_string("CraftAnything") else { return };
        let Ok(clip_class) = env.find_class("android/content/ClipData") else { return };
        let Ok(clip) = env
            .call_static_method(
                &clip_class,
                "newPlainText",
                "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Landroid/content/ClipData;",
                &[jni::objects::JValue::Object(&j_label), jni::objects::JValue::Object(&j_text)],
            )
            .and_then(|v| v.l())
        else {
            return;
        };

        for ctx_obj in contexts {
            let cm = env
                .call_method(
                    &ctx_obj,
                    "getSystemService",
                    "(Ljava/lang/String;)Ljava/lang/Object;",
                    &[jni::objects::JValue::Object(&service_name)],
                )
                .and_then(|v| v.l());

            if env.exception_check().unwrap_or(false) {
                let _ = env.exception_clear();
                continue;
            }

            if let Ok(cm_obj) = cm {
                if !cm_obj.is_null() {
                    let _ = env.call_method(
                        &cm_obj,
                        "setPrimaryClip",
                        "(Landroid/content/ClipData;)V",
                        &[jni::objects::JValue::Object(&clip)],
                    );
                    if env.exception_check().unwrap_or(false) {
                        let _ = env.exception_clear();
                    } else {
                        log::info!("Set Android clipboard text ({} chars)", text.len());
                        break;
                    }
                }
            }
        }
    }
}

/// Read clipboard text via Termux API if present.
#[cfg(target_os = "android")]
fn get_termux_clipboard() -> Option<String> {
    let termux_paths = [
        "/data/data/com.termux/files/usr/bin/termux-clipboard-get",
        "termux-clipboard-get",
    ];
    for cmd in termux_paths {
        if let Ok(output) = std::process::Command::new(cmd).output() {
            if output.status.success() {
                let s = String::from_utf8_lossy(&output.stdout).to_string();
                if !s.trim().is_empty() {
                    return Some(s);
                }
            }
        }
    }
    None
}

/// Set plain text to the clipboard.
pub fn set_text(ctx: &egui::Context, text: &str) {
    ctx.copy_text(text.to_string());
    set_in_memory_clipboard(text);
    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
    {
        if let Ok(mut cb) = Clipboard::new() {
            let _ = cb.set_text(text.to_string());
        }
    }
    #[cfg(target_os = "android")]
    {
        let _ = std::panic::catch_unwind(|| set_android_clipboard(text));
        let _ = std::process::Command::new("/data/data/com.termux/files/usr/bin/termux-clipboard-set")
            .arg(text)
            .status();
    }
}
