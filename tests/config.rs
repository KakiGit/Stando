mod support;

use stando::config::{Config, FloatingPreference};
use std::fs;
use support::{unique_temp_dir, EnvGuard, ENV_LOCK};

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
