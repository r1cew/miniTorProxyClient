#[cfg(target_os = "windows")]
use std::io;
#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;
use auto_launch::AutoLaunch;
use std::env;

/// Проверяет, включён ли системный прокси (только для Windows)
#[cfg(target_os = "windows")]
pub fn is_proxy_enabled() -> io::Result<bool> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"Software\Microsoft\Windows\CurrentVersion\Internet Settings";
    let settings = hkcu.open_subkey(path)?;
    let enabled: u32 = settings.get_value("ProxyEnable").unwrap_or(0);
    Ok(enabled != 0)
}

/// Инициализирует AutoLaunch
fn init_auto_launch() -> AutoLaunch {
    let exe_path = env::current_exe().expect("Не удалось получить путь к EXE");
    let exe_str = exe_path.to_str().unwrap();
    
    // Добавляем кавычки, если путь содержит пробелы
    let escaped_path = format!("\"{}\"", exe_str);


    AutoLaunch::new("TorProxyApp", &escaped_path, &["--hidden"])
}

/// Проверяет, включён ли автостарт
pub fn is_auto_start_enabled() -> Result<bool, Box<dyn std::error::Error>> {
    let auto = init_auto_launch();
    let status = auto.is_enabled()?;
    println!("Статус автозагрузки: {}", status); // Логирование
    Ok(status)
}

/// Включает автостарт
pub fn enable_auto_start() -> Result<(), Box<dyn std::error::Error>> {
    let auto = init_auto_launch();
    println!("Попытка включить автозагрузку...");
    auto.enable().map_err(|e| {
        eprintln!("Ошибка включения: {}", e);
        e.into()
    })
}

/// Отключает автостарт
pub fn disable_auto_start() -> Result<(), Box<dyn std::error::Error>> {
    let auto = init_auto_launch();
    println!("Попытка отключить автозагрузку...");
    auto.disable().map_err(|e| {
        eprintln!("Ошибка отключения: {}", e);
        e.into()
    })
}