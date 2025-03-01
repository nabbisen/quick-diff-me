#[cfg(target_os = "windows")]
const PRIMARY_FONT: &str = "BIZ UDGothic";

#[cfg(target_os = "macos")]
const PRIMARY_FONT: &str = "Hiragino Sans Mono";

#[cfg(target_os = "linux")]
const PRIMARY_FONT: &str = "monospace";

/// get app font on non-linux
#[cfg(not(target_os = "linux"))]
pub fn app_font<'a>() -> &'a str {
    PRIMARY_FONT
}

/// get app font on linux
#[cfg(target_os = "linux")]
pub fn app_font<'a>() -> &'a str {
    linux_font()
}

/// get primary font on linux
#[cfg(target_os = "linux")]
fn linux_font() -> &'static str {
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

    PRIMARY_FONT
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
