#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use image::ImageReader;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tray_icon::{
    menu::{Menu, MenuItem, MenuEvent},
    Icon, TrayIconBuilder,
};

mod proxy;
mod tor_controller;
mod registry_utils; // модуль для работы с реестром

// Импорт для работы с буфером обмена и файловой системой
use arboard::Clipboard;
use std::fs;
use std::process::Command;

#[derive(Debug, Clone)]
enum UserEvent {
    Menu(MenuEvent),
    StatusUpdate,
}

#[derive(Clone, Copy, PartialEq)]
enum AppState {
    TorStopped,
    TorRunning,
    TorWithProxy,
}

struct SharedState {
    current_state: AppState,
    proxy_enabled: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Загружаем иконки
    let icons = load_icons()?;
    
    // Проверяем, включён ли системный прокси
    let proxy_enabled = registry_utils::is_proxy_enabled().unwrap_or(false);

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let event_proxy = event_loop.create_proxy();

    let state = Arc::new(Mutex::new(SharedState {
        current_state: if proxy_enabled { AppState::TorWithProxy } else { AppState::TorStopped },
        proxy_enabled,
    }));

    // Создаем пункты меню
    let menu = Menu::new();
    // Пункт для переключения прокси (ид="1001")
    let proxy_item = MenuItem::new(
        if proxy_enabled { "Очистить системный прокси" } else { "Использовать как системный прокси" },
        true,
        None,
    );
    // Пункт для обновления мостов (ид="1002")
    let update_bridges_item = MenuItem::new("Обновить мосты из буфера обмена", true, None);
    // Пункт для выхода (ид="1003")
    let exit_item = MenuItem::new("Выход", true, None);
    
    // Добавляем пункты в меню
    menu.append(&proxy_item)?;
    menu.append(&update_bridges_item)?;
    menu.append(&exit_item)?;

    // Устанавливаем начальную иконку в зависимости от состояния
    let initial_icon_idx = match (if proxy_enabled { AppState::TorWithProxy } else { AppState::TorStopped }, proxy_enabled) {
        (AppState::TorStopped, _) => 0,
        (AppState::TorRunning, false) => 1,
        (AppState::TorRunning, true) | (AppState::TorWithProxy, true) => 2,
        _ => 0,
    };
    
    let tray_icon = TrayIconBuilder::new()
        .with_icon(icons[initial_icon_idx].clone())
        .with_menu(Box::new(menu))
        .build()
        .unwrap();

    // Спавним поток для мониторинга Tor в цикле.
    {
        let state_clone = state.clone();
        let event_proxy_clone = event_proxy.clone();
        std::thread::spawn(move || {
            loop {
                // Функция блокирующая; если Tor завершается – перезапускаем её спустя 5 секунд.
                tor_controller::start_tor_monitor(state_clone.clone(), event_proxy_clone.clone());
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        });
    }

    // Обработчик событий меню: отправляем событие в главный цикл
    {
        let event_proxy_for_menu = event_proxy.clone();
        MenuEvent::set_event_handler(Some(move |event| {
            let _ = event_proxy_for_menu.send_event(UserEvent::Menu(event));
        }));
    }

    // Основной цикл событий
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            tao::event::Event::UserEvent(user_event) => match user_event {
                UserEvent::Menu(menu_event) => {
                    let mut state = state.lock().unwrap();
                    match menu_event.id.0.as_str() {
                        // "1001" - переключение состояния прокси
                        "1001" => {
                            state.proxy_enabled = !state.proxy_enabled;
                            if state.proxy_enabled {
                                proxy::enable_proxy().unwrap();
                                proxy_item.set_text("Очистить системный прокси\n");
                            } else {
                                proxy::disable_proxy().unwrap();
                                proxy_item.set_text("Использовать как системный прокси\n");
                            }
                            let _ = event_proxy.send_event(UserEvent::StatusUpdate);
                        },
                        // "1002" - обновление мостов
                        "1002" => {
                            match Clipboard::new() {
                                Ok(mut clipboard) => {
                                    match clipboard.get_text() {
                                        Ok(bridges_text) => {
                                            // Формируем содержимое torrc с базовыми настройками и мостами из буфера обмена.
                                            // Каждая строка с мостом будет начинаться с "Bridge ".
                                            let torrc_content = format!(
r#"SocksPort 127.0.0.1:9050
HTTPTunnelPort 127.0.0.1:8118
UseBridges 1
ClientTransportPlugin obfs4 exec obfs4proxy
Bridge {}"#,
                                                bridges_text.replace("\n", "\nBridge ").trim()
                                            );
                                            
                                            // Записываем содержимое в файл torrc
                                            if let Err(e) = fs::write("torrc", torrc_content) {
                                                eprintln!("Ошибка записи torrc: {}", e);
                                            } else {
                                                println!("torrc обновлён. Новые мосты будут применены после перезапуска Tor.");
                                                // Перезапускаем Tor, завершая текущий процесс.
                                                if let Err(e) = Command::new("taskkill")
                                                    .args(&["/F", "/IM", "tor.exe"])
                                                    .output()
                                                {
                                                    eprintln!("Ошибка завершения tor.exe: {}", e);
                                                }
                                            }
                                        },
                                        Err(e) => eprintln!("Ошибка получения текста из буфера обмена: {}", e),
                                    }
                                },
                                Err(e) => eprintln!("Ошибка доступа к буферу обмена: {}", e),
                            }
                        },
                        // "1003" - выход (при выходе, если прокси включён – отключаем его)
                        "1003" => {
                            if let Err(e) = Command::new("taskkill")
                                .args(&["/F", "/IM", "tor.exe"])
                                .output()
                            {
                                eprintln!("Ошибка завершения tor.exe: {}", e);
                            }
                            if state.proxy_enabled {
                                state.proxy_enabled = false;
                                proxy::disable_proxy().unwrap();
                            }
                            std::process::exit(0)
                        },
                        other => {
                            println!("Неизвестный идентификатор меню: {}", other);
                        }
                    }
                },
                UserEvent::StatusUpdate => {
                    let state = state.lock().unwrap();
                    let icon_idx = match (state.current_state, state.proxy_enabled) {
                        (AppState::TorStopped, _) => 0,
                        (AppState::TorRunning, false) => 1,
                        (AppState::TorRunning, true) | (AppState::TorWithProxy, true) => 2,
                        _ => 0,
                    };
                    let _ = tray_icon.set_icon(Some(icons[icon_idx].clone()));
                },
            },
            _ => {}
        }
    })
}

fn load_icons() -> Result<Vec<Icon>, Box<dyn std::error::Error>> {
    let paths = [
        "resources/off.png",
        "resources/on.png",
        "resources/on_system.png",
    ];

    paths
        .iter()
        .map(|path| {
            let img = ImageReader::open(Path::new(path))
                .map_err(|e| format!("Failed to open {}: {}", path, e))?
                .decode()
                .map_err(|e| format!("Failed to decode {}: {}", path, e))?
                .into_rgba8();
            Icon::from_rgba(img.to_vec(), img.width(), img.height())
                .map_err(|e| format!("Failed to create icon from {}: {}", path, e).into())
        })
        .collect()
}
