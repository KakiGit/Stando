use crate::history_panel::ChatHistoryRecord;
use crate::logging;
use anyhow::{Context, Result};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::sync::Mutex;

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
