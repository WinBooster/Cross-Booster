use winreg::RegKey;
use winreg::enums::*;

#[cfg(target_os = "windows")]
pub struct RegistryUtils;

#[cfg(target_os = "windows")]
impl RegistryUtils {
    pub fn read_value(path: &str, key: &str) -> Result<String, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = path.split('\\').collect();
        if parts.len() < 2 {
            return Err("Invalid path".into());
        }
        let root = match parts[0] {
            "HKCR" => HKEY_CLASSES_ROOT,
            "HKCU" => HKEY_CURRENT_USER,
            "HKLM" => HKEY_LOCAL_MACHINE,
            "HKU" => HKEY_USERS,
            "HKCC" => HKEY_CURRENT_CONFIG,
            _ => return Err("Unknown root".into()),
        };
        let subpath = parts[1..].join("\\");
        let hkcu = RegKey::predef(root);
        let subkey = hkcu.open_subkey(subpath)?;
        let value: String = subkey.get_value(key)?;
        Ok(value)
    }

    pub fn read_dword_value(path: &str, key: &str) -> Result<u32, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = path.split('\\').collect();
        if parts.len() < 2 {
            return Err("Invalid path".into());
        }
        let root = match parts[0] {
            "HKCR" => HKEY_CLASSES_ROOT,
            "HKCU" => HKEY_CURRENT_USER,
            "HKLM" => HKEY_LOCAL_MACHINE,
            "HKU" => HKEY_USERS,
            "HKCC" => HKEY_CURRENT_CONFIG,
            _ => return Err("Unknown root".into()),
        };
        let subpath = parts[1..].join("\\");
        let hkcu = RegKey::predef(root);
        let subkey = hkcu.open_subkey(subpath)?;
        let value: u32 = subkey.get_value(key)?;
        Ok(value)
    }

    pub fn write_value(
        path: &str,
        key: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let parts: Vec<&str> = path.split('\\').collect();
        if parts.len() < 2 {
            return Err("Invalid path".into());
        }
        let root = match parts[0] {
            "HKCR" => HKEY_CLASSES_ROOT,
            "HKCU" => HKEY_CURRENT_USER,
            "HKLM" => HKEY_LOCAL_MACHINE,
            "HKU" => HKEY_USERS,
            "HKCC" => HKEY_CURRENT_CONFIG,
            _ => return Err("Unknown root".into()),
        };
        let subpath = parts[1..].join("\\");
        let hkcu = RegKey::predef(root);
        let (subkey, _) = hkcu.create_subkey(subpath)?;
        subkey.set_value(key, &value)?;
        Ok(())
    }

    pub fn write_dword_value(
        path: &str,
        key: &str,
        value: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let parts: Vec<&str> = path.split('\\').collect();
        if parts.len() < 2 {
            return Err("Invalid path".into());
        }
        let root = match parts[0] {
            "HKCR" => HKEY_CLASSES_ROOT,
            "HKCU" => HKEY_CURRENT_USER,
            "HKLM" => HKEY_LOCAL_MACHINE,
            "HKU" => HKEY_USERS,
            "HKCC" => HKEY_CURRENT_CONFIG,
            _ => return Err("Unknown root".into()),
        };
        let subpath = parts[1..].join("\\");
        let hkcu = RegKey::predef(root);
        let (subkey, _) = hkcu.create_subkey(subpath)?;
        subkey.set_value(key, &value)?;
        Ok(())
    }

    pub fn get_performance_visual_fx_setting() -> Result<u32, Box<dyn std::error::Error>> {
        Self::read_dword_value("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VisualEffects", "VisualFXSetting")
    }

    pub fn set_performance_visual_fx_setting(value: u32) -> Result<(), Box<dyn std::error::Error>> {
        Self::write_dword_value("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VisualEffects", "VisualFXSetting", value)
    }
}
