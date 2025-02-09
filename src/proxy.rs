use winreg::enums::*;
use winreg::RegKey;
use std::io;


pub fn enable_proxy() -> io::Result<()> {
    let proxy_server = "127.0.0.1:8118"; // Tor proxy port

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"Software\Microsoft\Windows\CurrentVersion\Internet Settings";
    let settings = hkcu.open_subkey_with_flags(path, KEY_WRITE)?;

    settings.set_value("ProxyEnable", &1u32)?;
    settings.set_value("ProxyServer", &proxy_server)?;
    Ok(())
}

pub fn disable_proxy() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"Software\Microsoft\Windows\CurrentVersion\Internet Settings";
    let settings = hkcu.open_subkey_with_flags(path, KEY_WRITE)?;

    settings.set_value("ProxyEnable", &0u32)?;
    settings.delete_value("ProxyServer")?;
    Ok(())
}