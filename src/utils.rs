use encoding_rs::IBM866;
use std::collections::HashMap;
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
use windows_sys::Win32::Globalization::GetUserDefaultUILanguage;

#[cfg(target_os = "windows")]
pub fn run_cmd_hidden(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("cmd")
        .args(&["/C", command])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()?;
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn run_cmd_hidden_with_output(command: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("cmd")
        .args(&["/C", command])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()?;
    let stdout = IBM866.decode(&output.stdout).0.to_string();
    Ok(stdout)
}

pub fn get_icon() -> &'static [u8; 3216] {
    let bytes: &'static [u8; 3216] = include_bytes!("../assets/icon.png");
    bytes
}

pub fn get_version() -> &'static str {
    option_env!("APP_VERSION").unwrap_or("1.0.0")
}

#[cfg(target_os = "windows")]
pub fn get_system_language() -> &'static str {
    unsafe {
        let lang_id = GetUserDefaultUILanguage();
        match lang_id & 0x3FF { // Primary language ID
            0x09 => "en", // English
            0x19 => "ru", // Russian
            // Add more languages as needed
            _ => "en", // Default to English
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_system_language() -> &'static str {
    "en" // Default for non-Windows
}
