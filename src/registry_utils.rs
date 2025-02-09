use winreg::enums::*;
use winreg::RegKey;
use std::io;

pub fn is_proxy_enabled() -> io::Result<bool> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"Software\Microsoft\Windows\CurrentVersion\Internet Settings";
    let settings = hkcu.open_subkey(path)?;
    let enabled: u32 = settings.get_value("ProxyEnable").unwrap_or(0);
    Ok(enabled != 0)
}
