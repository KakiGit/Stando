mod support;

use chrono::{DateTime, TimeZone, Utc};
use stando::history::{group_chat_history, ChatHistoryRecord, ChatRole, UsageHistory};
use std::collections::HashMap;
use std::fs;
use support::{unique_temp_dir, EnvGuard, ENV_LOCK};
use uuid::Uuid;

fn record(
    id: u128,
    chat_hash: &str,
    role: ChatRole,
    timestamp: DateTime<Utc>,
    summary: &str,
) -> ChatHistoryRecord {
    ChatHistoryRecord {
        id: Uuid::from_u128(id),
        timestamp,
        role,
        summary: summary.to_string(),
        content: format!("content-{summary}"),
        source: None,
        chat_hash: Some(chat_hash.to_string()),
    }
}

#[test]
fn groups_chat_history_by_latest_timestamp_and_metadata() {
    let records = vec![
        record(
            1,
            "alpha",
            ChatRole::User,
            Utc.with_ymd_and_hms(2026, 1, 1, 9, 0, 0).unwrap(),
            "alpha-1",
        ),
        record(
            2,
            "beta",
            ChatRole::Assistant,
            Utc.with_ymd_and_hms(2026, 1, 1, 10, 0, 0).unwrap(),
            "beta-1",
        ),
        record(
            3,
            "alpha",
            ChatRole::Assistant,
            Utc.with_ymd_and_hms(2026, 1, 1, 11, 0, 0).unwrap(),
            "alpha-2",
        ),
    ];

    let grouped = group_chat_history(records);
    assert_eq!(grouped.len(), 2);
    assert_eq!(grouped[0].id, "alpha");
    assert_eq!(grouped[0].metadata.last_speaker, ChatRole::Assistant);
    assert_eq!(
        grouped[0].metadata.last_timestamp,
        Utc.with_ymd_and_hms(2026, 1, 1, 11, 0, 0).unwrap()
    );
    assert_eq!(grouped[0].metadata.summary, "alpha-2");
    assert_eq!(grouped[0].rounds.len(), 2);
    assert_eq!(grouped[1].id, "beta");
    assert_eq!(grouped[1].rounds.len(), 1);
}

#[test]
fn usage_history_creates_expected_storage_files() {
    let _lock = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let root = unique_temp_dir("history-files");
    let home = root.join("home");
    fs::create_dir_all(&home).expect("create home dir");
    let _home_guard = EnvGuard::set("HOME", home.to_string_lossy().to_string());

    let _history = UsageHistory::load();
    let history_dir = home.join(".stando");
    assert!(history_dir.join("search-history.json").exists());
    assert!(history_dir.join("chat-history.json").exists());
}

#[test]
fn usage_history_persists_launch_counts() {
    let _lock = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let root = unique_temp_dir("history-persist");
    let home = root.join("home");
    fs::create_dir_all(&home).expect("create home dir");
    let _home_guard = EnvGuard::set("HOME", home.to_string_lossy().to_string());

    let history = UsageHistory::load();
    let rt = tokio::runtime::Runtime::new().expect("runtime");
    rt.block_on(async {
        history.record_launch("app:calculator".to_string()).await;
        history.record_launch("app:calculator".to_string()).await;
    });

    let history_file = home.join(".stando").join("search-history.json");
    let content = fs::read_to_string(&history_file).expect("read history");
    let map: HashMap<String, u64> = serde_json::from_str(&content).expect("parse history");
    assert_eq!(map.get("app:calculator").copied(), Some(2));
}
