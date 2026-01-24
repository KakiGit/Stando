use crate::logging;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use xdg::BaseDirectories;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub openai_api_key: Option<String>,
    pub search_paths: Vec<String>,
    pub max_results: usize,
    pub hotkey_show: String,
    /// Window-local shortcut for toggling AI mode.
    pub hotkey_ai_toggle: String,
}

impl Default for Config {
    fn default() -> Self {
        let _log_guard = logging::function_guard("Config::default");
        Self {
            openai_api_key: None,
            search_paths: vec![std::env::var("HOME").unwrap_or_else(|_| "/home".to_string())],
            max_results: 20,
            hotkey_show: "Super+Space".to_string(),
            hotkey_ai_toggle: "<Control>i".to_string(),
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
}
