use crate::logging;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use xdg::BaseDirectories;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FloatingPreference {
    pub enabled: bool,
    pub preferred_workspace: Option<String>,
    pub preferred_display: Option<String>,
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
    /// Base URL for the AI service. Defaults to OpenAI.
    pub ai_base_url: Option<String>,
    /// Configurable AI model name.
    pub ai_model: Option<String>,
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
            ai_base_url: None,
            ai_model: None,
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

    #[allow(dead_code)]
    pub fn floating_preference(&self) -> &FloatingPreference {
        &self.floating_preference
    }

    #[allow(dead_code)]
    pub fn set_floating_preference(&mut self, preference: FloatingPreference) -> Result<()> {
        let log_guard = logging::function_guard("Config::set_floating_preference");
        let result = {
            self.floating_preference = preference;
            self.save()
        };
        if result.is_err() {
            log_guard.mark_error();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::ENV_LOCK;
    use std::fs;
    use uuid::Uuid;

    struct EnvGuard {
        key: &'static str,
        previous: Option<String>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: String) -> Self {
            let previous = std::env::var(key).ok();
            std::env::set_var(key, value);
            Self { key, previous }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            if let Some(value) = self.previous.take() {
                std::env::set_var(self.key, value);
            } else {
                std::env::remove_var(self.key);
            }
        }
    }

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        dir.push(format!("stando-test-{prefix}-{}", Uuid::new_v4()));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    #[test]
    fn config_load_creates_default_in_xdg_config_home() {
        let _lock = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let root = unique_temp_dir("config-default");
        let home = root.join("home");
        let xdg = root.join("xdg");
        fs::create_dir_all(&home).expect("create home dir");
        fs::create_dir_all(&xdg).expect("create xdg dir");
        let _home_guard = EnvGuard::set("HOME", home.to_string_lossy().to_string());
        let _xdg_guard = EnvGuard::set("XDG_CONFIG_HOME", xdg.to_string_lossy().to_string());

        let config = Config::load().expect("load config");
        let config_path = Config::config_path().expect("config path");

        assert!(config_path.exists());
        assert!(config_path.ends_with("stando/config.toml"));
        assert_eq!(config.max_results, 20);
        assert_eq!(config.hotkey_ai_toggle, "<Control>i");
        assert_eq!(
            config.search_paths,
            vec![home.to_string_lossy().to_string()]
        );
        assert!(config
            .app_search_paths
            .iter()
            .any(|path| path.ends_with("/.local/share/applications")));
    }

    #[test]
    fn config_round_trip_persists_updates() {
        let _lock = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let root = unique_temp_dir("config-round-trip");
        let home = root.join("home");
        let xdg = root.join("xdg");
        fs::create_dir_all(&home).expect("create home dir");
        fs::create_dir_all(&xdg).expect("create xdg dir");
        let _home_guard = EnvGuard::set("HOME", home.to_string_lossy().to_string());
        let _xdg_guard = EnvGuard::set("XDG_CONFIG_HOME", xdg.to_string_lossy().to_string());

        let mut config = Config::load().expect("load config");
        config.max_results = 55;
        config.ai_base_url = Some("https://example.invalid".to_string());
        config.ai_model = Some("test-model".to_string());
        config.hotkey_ai_toggle = "<Alt>F12".to_string();
        config
            .set_floating_preference(FloatingPreference {
                enabled: true,
                preferred_workspace: Some("workspace-1".to_string()),
                preferred_display: Some("display-1".to_string()),
            })
            .expect("save floating preference");

        let reloaded = Config::load().expect("reload config");
        assert_eq!(reloaded.max_results, 55);
        assert_eq!(
            reloaded.ai_base_url.as_deref(),
            Some("https://example.invalid")
        );
        assert_eq!(reloaded.ai_model.as_deref(), Some("test-model"));
        assert_eq!(reloaded.hotkey_ai_toggle, "<Alt>F12");
        assert!(reloaded.floating_preference.enabled);
        assert_eq!(
            reloaded.floating_preference.preferred_workspace.as_deref(),
            Some("workspace-1")
        );
        assert_eq!(
            reloaded.floating_preference.preferred_display.as_deref(),
            Some("display-1")
        );
    }
}
