use crate::logging;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::sync::Mutex;

pub struct UsageHistory {
    path: PathBuf,
    counts: Mutex<HashMap<String, u64>>,
}

impl UsageHistory {
    pub fn load() -> Self {
        let _log_guard = logging::function_guard("UsageHistory::load");
        let dir = Self::history_dir();
        let path = dir.join("search-history.json");
        let counts = Self::load_counts(&path);
        Self {
            path,
            counts: Mutex::new(counts),
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

    fn persist(&self, counts: &HashMap<String, u64>) -> Result<()> {
        let _log_guard = logging::function_guard("UsageHistory::persist");
        let tmp_path = self.path.with_extension("json.tmp");
        let content =
            serde_json::to_string_pretty(counts).context("Failed to serialize search history")?;
        fs::write(&tmp_path, content).context("Failed to write temporary history file")?;
        fs::rename(tmp_path, &self.path).context("Failed to rename history file")?;
        Ok(())
    }
}
