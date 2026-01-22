use anyhow::{Context, Result};
use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use crate::config::Config;

pub struct HotkeyManager {
    _manager: GlobalHotKeyManager,
    show_hide_id: u32,
    toggle_ai_id: u32,
    last_pressed: std::sync::Mutex<Option<u32>>,
}

#[derive(Debug, Clone, Copy)]
pub enum HotKeyEvent {
    ShowHide,
    ToggleAI,
}

impl HotkeyManager {
    pub fn new(config: &Config) -> Result<Self> {
        let manager = GlobalHotKeyManager::new()
            .context("Failed to create hotkey manager")?;

        let show_hide_key: HotKey = config
            .hotkey_show
            .parse()
            .context("Failed to parse show/hide hotkey")?;
        manager
            .register(show_hide_key)
            .context("Failed to register show/hide hotkey")?;

        let toggle_ai_key: HotKey = config
            .hotkey_ai_toggle
            .parse()
            .context("Failed to parse AI toggle hotkey")?;
        manager
            .register(toggle_ai_key)
            .context("Failed to register AI toggle hotkey")?;

        Ok(Self {
            _manager: manager,
            show_hide_id: show_hide_key.id(),
            toggle_ai_id: toggle_ai_key.id(),
            last_pressed: std::sync::Mutex::new(None),
        })
    }

    pub fn try_recv(&self) -> Result<Option<HotKeyEvent>> {
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
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

            if event.id == self.toggle_ai_id {
                return Ok(Some(HotKeyEvent::ToggleAI));
            }
        }

        Ok(None)
    }
}

// Esc key handling is done in the UI layer since it's a window-level event
