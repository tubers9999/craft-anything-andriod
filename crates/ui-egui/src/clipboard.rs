//! System clipboard reading & writing helper with Android and Termux support.

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
use arboard::Clipboard;

/// Get plain text from the system clipboard.
pub fn get_text() -> Option<String> {
    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
    {
        if let Ok(mut cb) = Clipboard::new() {
            if let Ok(t) = cb.get_text() {
                if !t.is_empty() {
                    return Some(t);
                }
            }
        }
    }

    #[cfg(target_os = "android")]
    {
        if let Some(text) = get_termux_clipboard() {
            if !text.trim().is_empty() {
                return Some(text);
            }
        }
    }

    None
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
    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
    {
        if let Ok(mut cb) = Clipboard::new() {
            let _ = cb.set_text(text.to_string());
        }
    }
    #[cfg(target_os = "android")]
    {
        let _ = std::process::Command::new("/data/data/com.termux/files/usr/bin/termux-clipboard-set")
            .arg(text)
            .status();
    }
}
