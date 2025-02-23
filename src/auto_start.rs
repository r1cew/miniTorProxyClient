use std::io;
use winreg::RegKey;
use winreg::enums::*;

/// Проверяет, включён ли автозапуск (то есть существует ли значение "TorClient" в реестре)
pub fn is_auto_start_enabled() -> io::Result<bool> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    match hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_READ) {
        Ok(run_key) => {
            match run_key.get_value::<String, _>("TorClient") {
                Ok(_) => Ok(true),
                Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

/// Добавляет в реестр запись для автозапуска приложения.
/// В качестве команды используется запуск через cmd.exe, чтобы задать рабочую директорию.
pub fn enable_auto_start(app_path: &str) -> io::Result<()> {
    let command = format!(r#""cmd.exe" /c start "" "{}""#, app_path);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (run_key, _) = hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run")?;
    run_key.set_value("TorClient", &command)?;
    Ok(())
}

/// Удаляет запись автозапуска из реестра.
pub fn disable_auto_start() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_WRITE)?;
    match run_key.delete_value("TorClient") {
        Ok(()) => Ok(()),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
    }
}

/// Переключает автозапуск: если включён – выключает, если выключен – включает.
/// Возвращает новое состояние (true – включён, false – выключен).
pub fn toggle_auto_start(app_path: &str) -> io::Result<bool> {
    if is_auto_start_enabled()? {
        disable_auto_start()?;
        Ok(false)
    } else {
        enable_auto_start(app_path)?;
        Ok(true)
    }
}
