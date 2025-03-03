#[cfg(target_os = "windows")]
const APP_DEFAULT_FONT: &str = "Yu Gothic";

#[cfg(target_os = "macos")]
const APP_DEFAULT_FONT: &str = "Hiragino Sans";

#[cfg(target_os = "linux")]
const APP_DEFAULT_FONT: &str = "sans-self";

/// get app font on non-linux
#[cfg(not(target_os = "linux"))]
pub fn app_default_font<'a>() -> &'a str {
    APP_DEFAULT_FONT
}

/// get app font on linux
#[cfg(target_os = "linux")]
pub fn app_default_font<'a>() -> &'a str {
    linux_app_default_font()
}

/// get primary font on linux
#[cfg(target_os = "linux")]
fn linux_app_default_font() -> &'static str {
    let fonts = [
        "Noto Sans CJK JP",
        "Droid Sans",
        "IPAGothic",
        "TakaoPGothic",
        "VL Gothic",
        "DejaVu Sans",
    ];

    for font in fonts {
        if is_installed_font(font) {
            return font;
        }
    }

    APP_DEFAULT_FONT
}

#[cfg(target_os = "windows")]
const DIFF_FONT: &str = "BIZ UDGothic";

#[cfg(target_os = "macos")]
const DIFF_FONT: &str = "Hiragino Sans Mono";

#[cfg(target_os = "linux")]
const DIFF_FONT: &str = "monospace";

/// get app font on non-linux
#[cfg(not(target_os = "linux"))]
pub fn diff_font<'a>() -> &'a str {
    DIFF_FONT
}

/// get app font on linux
#[cfg(target_os = "linux")]
pub fn diff_font<'a>() -> &'a str {
    linux_diff_font()
}

/// get primary font on linux
#[cfg(target_os = "linux")]
fn linux_diff_font() -> &'static str {
    let fonts = [
        "Noto Sans Mono CJK JP",
        "Droid Sans Mono",
        "Source Code Pro",
        "Hack",
        "Ubuntu Mono",
    ];

    for font in fonts {
        if is_installed_font(font) {
            return font;
        }
    }

    DIFF_FONT
}

/// check if the font is installed in os
#[cfg(target_os = "linux")]
fn is_installed_font(font_name: &str) -> bool {
    use std::process::Command;

    if let Ok(output) = Command::new("fc-list").arg(":family").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.contains(font_name)
    } else {
        false
    }
}
