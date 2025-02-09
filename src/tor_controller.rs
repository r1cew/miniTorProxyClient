use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use tao::event_loop::EventLoopProxy;
use crate::{AppState, SharedState, UserEvent};
use std::sync::{Arc, Mutex};
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn start_tor_monitor(state: Arc<Mutex<SharedState>>, event_proxy: EventLoopProxy<UserEvent>) {
    let mut child = Command::new(r"./tor.exe")
        .arg("-f")
        .arg(r"./torrc")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(CREATE_NO_WINDOW) // Скрываем окно консоли
        .spawn()
        .expect("Failed to start tor.exe");

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        if let Ok(line) = line {
            println!("{}", line);
            if line.contains("Bootstrapped 100%") {
                let mut state = state.lock().unwrap();
                state.current_state = AppState::TorRunning;
                let _ = event_proxy.send_event(UserEvent::StatusUpdate);
            }
        }
    }

    let mut state = state.lock().unwrap();
    state.current_state = AppState::TorStopped;
    let _ = event_proxy.send_event(UserEvent::StatusUpdate);
}
