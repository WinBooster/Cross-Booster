use std::collections::HashMap;

#[cfg(target_os = "windows")]
use crate::utils::{run_cmd_hidden, run_cmd_hidden_with_output};

#[cfg(target_os = "windows")]
pub fn get_current_power_scheme_guid() -> Result<String, Box<dyn std::error::Error>> {
    let output = run_cmd_hidden_with_output("powercfg /getactivescheme")?;
    // Parse the GUID from output like: "Power Scheme GUID: 381b4222-f694-41f0-9685-ff5bb260df2e (Balanced)"
    // or in Russian: "GUID схемы питания: 381b4222-f694-41f0-9685-ff5bb260df2e (Сбалансированная)"
    for line in output.lines() {
        if line.contains("GUID") {
            if let Some(colon_pos) = line.find(':') {
                let guid_part = &line[colon_pos + 1..].trim_start();
                if let Some(space_pos) = guid_part.find(' ') {
                    let guid = guid_part[..space_pos].to_string();
                    if guid.len() == 36 {
                        // GUID length
                        return Ok(guid);
                    }
                }
            }
        }
    }
    Err("Failed to parse power scheme GUID".into())
}

#[cfg(target_os = "windows")]
pub fn check_if_scheme_exists(guid: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let output = run_cmd_hidden_with_output("powercfg /list")?;
    Ok(output.contains(guid))
}

#[cfg(target_os = "windows")]
pub fn create_scheme_if_not_exists(guid: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !check_if_scheme_exists(guid)? {
        // Duplicate the balanced scheme to create high performance
        run_cmd_hidden(&format!(
            "powercfg /duplicatescheme 381b4222-f694-41f0-9685-ff5bb260df2e {}",
            guid
        ))?;
    }
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn get_power_schemes() -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let output = run_cmd_hidden_with_output("powercfg /list")?;
    let mut schemes = Vec::new();
    for line in output.lines() {
        if line.contains("Power Scheme GUID:") || line.contains("GUID схемы питания:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let guid = parts[3];
                let mut name = parts[4..].join(" ");
                // Remove the active indicator *
                if name.ends_with(" *") {
                    name = name.trim_end_matches(" *").to_string();
                }
                if guid.len() == 36 {
                    schemes.push((guid.to_string(), name));
                }
            }
        }
    }
    Ok(schemes)
}

#[cfg(target_os = "windows")]
pub fn delete_scheme(guid: &str) -> Result<(), Box<dyn std::error::Error>> {
    run_cmd_hidden(&format!("powercfg /delete {}", guid))
}

#[cfg(target_os = "windows")]
pub fn cleanup_duplicate_schemes() -> Result<(), Box<dyn std::error::Error>> {
    let schemes = get_power_schemes()?;
    let mut seen = HashMap::new();
    for (guid, name) in schemes {
        if seen.contains_key(&name) {
            // Keep the default ones, delete others
            if guid != "381b4222-f694-41f0-9685-ff5bb260df2e"
                && guid != "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c"
                && guid != "a1841308-3541-4fab-bc81-f71556f20b4a"
            {
                delete_scheme(&guid)?;
            }
        } else {
            seen.insert(name, guid);
        }
    }
    Ok(())
}
