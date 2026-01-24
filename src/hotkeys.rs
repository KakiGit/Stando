use crate::config::Config;
use crate::logging;
use anyhow::Result;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};

pub struct HotkeyManager {
    _manager: GlobalHotKeyManager,
    show_hide_id: u32,
    last_pressed: std::sync::Mutex<Option<u32>>,
}

#[derive(Debug, Clone, Copy)]
pub enum HotKeyEvent {
    ShowHide,
}

impl HotkeyManager {
    pub fn new(_config: &Config) -> Result<Self> {
        let log_guard = logging::function_guard("HotkeyManager::new");
        let result = {
            let manager = GlobalHotKeyManager::new().unwrap();

            let show_hide_key = HotKey::new(Some(Modifiers::SUPER), Code::KeyA);
            manager.register(show_hide_key).unwrap();

            Ok(Self {
                _manager: manager,
                show_hide_id: show_hide_key.id(),
                last_pressed: std::sync::Mutex::new(None),
            })
        };
        if result.is_err() {
            log_guard.mark_error();
        }
        result
    }

    pub fn try_recv(&self) -> Result<Option<HotKeyEvent>> {
        let result = (|| {
            if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
                let _log_guard = logging::function_guard("HotkeyManager::try_recv");
                let mut last_pressed = self
                    .last_pressed
                    .lock()
                    .expect("HotkeyManager last_pressed mutex poisoned");

                match event.state {
                    HotKeyState::Pressed => {
                        *last_pressed = Some(event.id);
                    }
                    HotKeyState::Released => {
                        if last_pressed.take() == Some(event.id) {
                            return Ok(None);
                        }
                    }
                }

                if event.id == self.show_hide_id {
                    return Ok(Some(HotKeyEvent::ShowHide));
                }
            }

            Ok(None)
        })();
        if result.is_err() {
            let log_guard = logging::function_guard("HotkeyManager::try_recv");
            log_guard.mark_error();
        }
        result
    }
}

// Esc key handling is done in the UI layer since it's a window-level event
