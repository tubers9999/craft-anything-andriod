//! System clipboard reading & writing helper with Android and Termux support.

use std::sync::Mutex;

static IN_MEMORY_CLIPBOARD: Mutex<Option<String>> = Mutex::new(None);

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
        if let Some(text) = get_android_clipboard() {
            if !text.trim().is_empty() {
                set_in_memory_clipboard(&text);
                return Some(text);
            }
        }
        if let Some(text) = get_termux_clipboard() {
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
    let activity_ptr = ctx.context();
    if vm_ptr.is_null() || activity_ptr.is_null() {
        return None;
    }
    unsafe {
        let vm = jni::JavaVM::from_raw(vm_ptr as *mut jni::sys::JavaVM).ok()?;
        let mut env = vm.attach_current_thread().ok()?;

        if let Ok(looper_class) = env.find_class("android/os/Looper") {
            let my_looper = env.call_static_method(&looper_class, "myLooper", "()Landroid/os/Looper;", &[]);
            if let Ok(val) = my_looper {
                if val.l().map_or(true, |obj| obj.is_null()) {
                    let _ = env.call_static_method(&looper_class, "prepare", "()V", &[]);
                }
            }
        }
        if env.exception_check().unwrap_or(false) {
            let _ = env.exception_clear();
        }

        let service_name = env.new_string("clipboard").ok()?;
        let activity = jni::objects::JObject::from_raw(activity_ptr as jni::sys::jobject);
        let cm = env
            .call_method(
                &activity,
                "getSystemService",
                "(Ljava/lang/String;)Ljava/lang/Object;",
                &[jni::objects::JValue::Object(&service_name)],
            )
            .ok()?
            .l()
            .ok()?;

        if cm.is_null() {
            return None;
        }

        let has_clip = env
            .call_method(&cm, "hasPrimaryClip", "()Z", &[])
            .ok()?
            .z()
            .ok()?;

        if !has_clip {
            return None;
        }

        let clip = env
            .call_method(&cm, "getPrimaryClip", "()Landroid/content/ClipData;", &[])
            .ok()?
            .l()
            .ok()?;

        if clip.is_null() {
            return None;
        }

        let count = env
            .call_method(&clip, "getItemCount", "()I", &[])
            .ok()?
            .i()
            .ok()?;

        if count <= 0 {
            return None;
        }

        let item = env
            .call_method(
                &clip,
                "getItemAt",
                "(I)Landroid/content/ClipData$Item;",
                &[jni::objects::JValue::Int(0)],
            )
            .ok()?
            .l()
            .ok()?;

        if item.is_null() {
            return None;
        }

        let text_cs = env
            .call_method(
                &item,
                "coerceToText",
                "(Landroid/content/Context;)Ljava/lang/CharSequence;",
                &[jni::objects::JValue::Object(&activity)],
            )
            .ok()?
            .l()
            .ok()?;

        if text_cs.is_null() {
            return None;
        }

        let j_str = env
            .call_method(&text_cs, "toString", "()Ljava/lang/String;", &[])
            .ok()?
            .l()
            .ok()?;

        let rust_str: String = env.get_string(&jni::objects::JString::from(j_str)).ok()?.into();
        if !rust_str.is_empty() {
            Some(rust_str)
        } else {
            None
        }
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
    let activity_ptr = ctx.context();
    if vm_ptr.is_null() || activity_ptr.is_null() {
        return;
    }
    unsafe {
        let Ok(vm) = jni::JavaVM::from_raw(vm_ptr as *mut jni::sys::JavaVM) else { return };
        let Ok(mut env) = vm.attach_current_thread() else { return };
        if let Ok(looper_class) = env.find_class("android/os/Looper") {
            let my_looper = env.call_static_method(&looper_class, "myLooper", "()Landroid/os/Looper;", &[]);
            if let Ok(val) = my_looper {
                if val.l().map_or(true, |obj| obj.is_null()) {
                    let _ = env.call_static_method(&looper_class, "prepare", "()V", &[]);
                }
            }
        }
        if env.exception_check().unwrap_or(false) {
            let _ = env.exception_clear();
        }
        let Ok(service_name) = env.new_string("clipboard") else { return };
        let activity = jni::objects::JObject::from_raw(activity_ptr as jni::sys::jobject);
        let Ok(cm) = env
            .call_method(
                &activity,
                "getSystemService",
                "(Ljava/lang/String;)Ljava/lang/Object;",
                &[jni::objects::JValue::Object(&service_name)],
            )
            .and_then(|v| v.l())
        else {
            return;
        };

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

        let _ = env.call_method(
            &cm,
            "setPrimaryClip",
            "(Landroid/content/ClipData;)V",
            &[jni::objects::JValue::Object(&clip)],
        );
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
        set_android_clipboard(text);
        let _ = std::process::Command::new("/data/data/com.termux/files/usr/bin/termux-clipboard-set")
            .arg(text)
            .status();
    }
}
