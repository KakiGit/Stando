use crate::logging;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::sync::Mutex;
use uuid::Uuid;

pub const RECENT_CHAT_ID: &str = "recent-chat";

/// Indicates which participant authored the entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatRole {
    User,
    Assistant,
}

impl ChatRole {
    pub fn label(&self) -> &'static str {
        match self {
            ChatRole::User => "You",
            ChatRole::Assistant => "AI",
        }
    }
}

/// A serialization-friendly record stored on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatHistoryRecord {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub role: ChatRole,
    pub summary: String,
    pub content: String,
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_hash: Option<String>,
}

impl ChatHistoryRecord {
    pub fn chat_id(&self) -> String {
        normalize_chat_hash(self.chat_hash.as_deref())
    }
}

#[derive(Debug, Clone)]
pub struct ConversationRound {
    pub round_id: Uuid,
    pub role: ChatRole,
    pub timestamp: DateTime<Utc>,
    pub content: String,
    pub source: Option<String>,
}

impl From<&ChatHistoryRecord> for ConversationRound {
    fn from(record: &ChatHistoryRecord) -> Self {
        Self {
            round_id: record.id,
            role: record.role,
            timestamp: record.timestamp,
            content: record.content.clone(),
            source: record.source.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChatGroupMetadata {
    pub last_speaker: ChatRole,
    pub last_timestamp: DateTime<Utc>,
    pub summary: String,
}

#[derive(Debug, Clone)]
pub struct ChatGroup {
    pub id: String,
    pub metadata: ChatGroupMetadata,
    pub rounds: Vec<ConversationRound>,
}

#[derive(Debug, Clone)]
pub struct ChatSummary {
    pub chat_hash: String,
    pub display_label: String,
    pub last_actor: ChatRole,
    pub last_timestamp: DateTime<Utc>,
    pub round_count: usize,
}

pub fn normalize_chat_hash(hash: Option<&str>) -> String {
    let normalized = hash
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    normalized.unwrap_or_else(|| RECENT_CHAT_ID.to_string())
}

pub fn group_chat_history(records: Vec<ChatHistoryRecord>) -> Vec<ChatGroup> {
    let mut groups: HashMap<String, ChatGroup> = HashMap::new();

    for record in records {
        let chat_id = record.chat_id();
        let summary = record.summary.clone();
        let round = ConversationRound::from(&record);
        let entry = groups.entry(chat_id.clone()).or_insert_with(|| ChatGroup {
            id: chat_id.clone(),
            metadata: ChatGroupMetadata {
                last_speaker: record.role,
                last_timestamp: record.timestamp,
                summary: summary.clone(),
            },
            rounds: Vec::new(),
        });

        entry.rounds.push(round);
        if record.timestamp >= entry.metadata.last_timestamp {
            entry.metadata.last_timestamp = record.timestamp;
            entry.metadata.last_speaker = record.role;
            entry.metadata.summary = summary;
        }
    }

    for group in groups.values_mut() {
        group.rounds.sort_by_key(|round| round.timestamp);
    }

    let mut grouped = groups.into_values().collect::<Vec<_>>();
    grouped.sort_by(|left, right| right.metadata.last_timestamp.cmp(&left.metadata.last_timestamp));
    grouped
}

pub fn grouped_history_with_selection(
    records: Vec<ChatHistoryRecord>,
) -> (Vec<ChatGroup>, Option<String>) {
    let grouped = group_chat_history(records);
    let selected_chat_id = grouped.first().map(|chat| chat.id.clone());
    tracing::debug!(
        chat_count = grouped.len(),
        selected_chat_id = ?selected_chat_id,
        "Grouped chat history"
    );
    (grouped, selected_chat_id)
}

pub fn chat_timeline_by_hash(
    chats: &[ChatGroup],
    chat_id: &str,
) -> Option<Vec<ConversationRound>> {
    chats
        .iter()
        .find(|chat| chat.id == chat_id)
        .map(|chat| {
            let mut rounds = chat.rounds.clone();
            rounds.sort_by_key(|round| round.timestamp);
            rounds
        })
}

pub struct UsageHistory {
    path: PathBuf,
    counts: Mutex<HashMap<String, u64>>,
    chat_history_path: PathBuf,
    chat_entries: Mutex<Vec<ChatHistoryRecord>>,
}

impl UsageHistory {
    pub fn load() -> Self {
        let _log_guard = logging::function_guard("UsageHistory::load");
        let dir = Self::history_dir();
        let path = dir.join("search-history.json");
        let counts = Self::load_counts(&path);
        let chat_history_path = dir.join("chat-history.json");
        let chat_entries = Self::load_chat_entries(&chat_history_path);
        Self {
            path,
            counts: Mutex::new(counts),
            chat_history_path,
            chat_entries: Mutex::new(chat_entries),
        }
    }

    pub async fn snapshot(&self) -> HashMap<String, u64> {
        let guard = self.counts.lock().await;
        guard.clone()
    }

    pub async fn record_launch(&self, id: String) {
        let mut guard = self.counts.lock().await;
        let counter = guard.entry(id.clone()).or_insert(0);
        *counter = counter.saturating_add(1);
        let snapshot = guard.clone();
        drop(guard);
        if let Err(err) = self.persist(&snapshot) {
            tracing::warn!(error = ?err, "Failed to persist search history");
        }
    }

    pub async fn chat_entries(&self) -> Vec<ChatHistoryRecord> {
        let guard = self.chat_entries.lock().await;
        guard.clone()
    }

    pub async fn record_chat_entry(&self, entry: ChatHistoryRecord) {
        let mut guard = self.chat_entries.lock().await;
        guard.push(entry.clone());
        let snapshot = guard.clone();
        drop(guard);
        if let Err(err) = Self::persist_chat_entries(&self.chat_history_path, &snapshot) {
            tracing::warn!(error = ?err, "Failed to persist chat history entry");
        }
    }

    fn history_dir() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
        let dir = Path::new(&home).join(".stando");
        if let Err(err) = fs::create_dir_all(&dir) {
            tracing::warn!(error = ?err, "Failed to ensure ~/.stando exists");
        }
        dir
    }

    fn load_counts(path: &Path) -> HashMap<String, u64> {
        if !path.exists() {
            if let Err(err) = fs::write(path, "{}") {
                tracing::warn!(error = ?err, "Failed to create history file");
            }
            return HashMap::new();
        }

        match fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str::<HashMap<String, u64>>(&content) {
                Ok(map) => map,
                Err(err) => {
                    tracing::warn!(error = ?err, "Failed to parse search history file");
                    HashMap::new()
                }
            },
            Err(err) => {
                tracing::warn!(error = ?err, "Failed to read search history");
                HashMap::new()
            }
        }
    }

    fn load_chat_entries(path: &Path) -> Vec<ChatHistoryRecord> {
        if !path.exists() {
            if let Err(err) = fs::write(path, "[]") {
                tracing::warn!(
                    error = ?err,
                    "Failed to create chat history file; proceeding with empty state"
                );
            }
            return Vec::new();
        }

        match fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str::<Vec<ChatHistoryRecord>>(&content) {
                Ok(entries) => entries,
                Err(err) => {
                    tracing::warn!(error = ?err, "Failed to parse chat history file");
                    Vec::new()
                }
            },
            Err(err) => {
                tracing::warn!(error = ?err, "Failed to read chat history");
                Vec::new()
            }
        }
    }

    fn persist(&self, counts: &HashMap<String, u64>) -> Result<()> {
        let _log_guard = logging::function_guard("UsageHistory::persist");
        let tmp_path = self.path.with_extension("json.tmp");
        let content =
            serde_json::to_string_pretty(counts).context("Failed to serialize search history")?;
        fs::write(&tmp_path, content).context("Failed to write temporary history file")?;
        fs::rename(tmp_path, &self.path).context("Failed to rename history file")?;
        Ok(())
    }

    fn persist_chat_entries(path: &Path, entries: &[ChatHistoryRecord]) -> Result<()> {
        let _log_guard = logging::function_guard("UsageHistory::persist_chat_entries");
        let tmp_path = path.with_extension("json.tmp");
        let content =
            serde_json::to_string_pretty(entries).context("Failed to serialize chat history")?;
        fs::write(&tmp_path, content).context("Failed to write temporary chat history file")?;
        fs::rename(tmp_path, path).context("Failed to rename chat history file")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

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
}
