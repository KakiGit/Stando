use crate::logging;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use xdg::BaseDirectories;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FloatingPreference {
    pub enabled: bool,
    pub preferred_workspace: Option<String>,
    pub preferred_display: Option<String>,
}

impl Default for FloatingPreference {
    fn default() -> Self {
        Self {
            enabled: false,
            preferred_workspace: None,
            preferred_display: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub openai_api_key: Option<String>,
    pub search_paths: Vec<String>,
    pub app_search_paths: Vec<String>,
    pub max_results: usize,
    pub hotkey_show: String,
    /// Window-local shortcut for toggling AI mode.
    pub hotkey_ai_toggle: String,
    pub floating_preference: FloatingPreference,
}

impl Default for Config {
    fn default() -> Self {
        let _log_guard = logging::function_guard("Config::default");
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
        Self {
            openai_api_key: None,
            search_paths: vec![home_dir.clone()],
            app_search_paths: vec![
                "/usr/share/applications".to_string(),
                "/usr/local/share/applications".to_string(),
                format!("{}/.local/share/applications", home_dir),
            ],
            max_results: 20,
            hotkey_show: "Super+Space".to_string(),
            hotkey_ai_toggle: "<Control>i".to_string(),
            floating_preference: FloatingPreference::default(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let log_guard = logging::function_guard("Config::load");
        let result = (|| {
            let xdg_dirs =
                BaseDirectories::with_prefix("stando").context("Failed to get XDG directories")?;

            let config_path = xdg_dirs
                .place_config_file("config.toml")
                .context("Failed to get config file path")?;

            if !config_path.exists() {
                let default_config = Config::default();
                default_config.save()?;
                return Ok(default_config);
            }

            let content = fs::read_to_string(&config_path).context("Failed to read config file")?;

            let config: Config = toml::from_str(&content).context("Failed to parse config file")?;

            Ok(config)
        })();
        if result.is_err() {
            log_guard.mark_error();
        }
        result
    }

    pub fn save(&self) -> Result<()> {
        let log_guard = logging::function_guard("Config::save");
        let result = (|| {
            let xdg_dirs =
                BaseDirectories::with_prefix("stando").context("Failed to get XDG directories")?;

            let config_path = xdg_dirs
                .place_config_file("config.toml")
                .context("Failed to get config file path")?;

            // Ensure config directory exists
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent).context("Failed to create config directory")?;
            }

            let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

            fs::write(&config_path, content).context("Failed to write config file")?;

            Ok(())
        })();
        if result.is_err() {
            log_guard.mark_error();
        }
        result
    }

    #[allow(dead_code)]
    pub fn config_path() -> Result<PathBuf> {
        let log_guard = logging::function_guard("Config::config_path");
        let result = (|| {
            let xdg_dirs =
                BaseDirectories::with_prefix("stando").context("Failed to get XDG directories")?;

            xdg_dirs
                .place_config_file("config.toml")
                .context("Failed to get config file path")
        })();
        if result.is_err() {
            log_guard.mark_error();
        }
        result
    }

    pub fn floating_preference(&self) -> &FloatingPreference {
        &self.floating_preference
    }

    pub fn set_floating_preference(&mut self, preference: FloatingPreference) -> Result<()> {
        let log_guard = logging::function_guard("Config::set_floating_preference");
        let result = (|| {
            self.floating_preference = preference;
            self.save()
        })();
        if result.is_err() {
            log_guard.mark_error();
        }
        result
    }
}
