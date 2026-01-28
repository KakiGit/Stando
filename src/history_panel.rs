use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::ops::RangeInclusive;
use uuid::Uuid;

pub const DEFAULT_EMPTY_MESSAGE: &str =
    "No history yet. Ask a question to populate the chat history panel.";

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

/// A single entry that is shown inside the AI history panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatHistoryEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub role: ChatRole,
    pub summary: String,
    pub content: String,
    pub source: Option<String>,
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
}

impl From<ChatHistoryEntry> for ChatHistoryRecord {
    fn from(entry: ChatHistoryEntry) -> Self {
        Self {
            id: entry.id,
            timestamp: entry.timestamp,
            role: entry.role,
            summary: entry.summary,
            content: entry.content,
            source: entry.source,
        }
    }
}

impl From<ChatHistoryRecord> for ChatHistoryEntry {
    fn from(record: ChatHistoryRecord) -> Self {
        Self {
            id: record.id,
            timestamp: record.timestamp,
            role: record.role,
            summary: record.summary,
            content: record.content,
            source: record.source,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HistoryPanelState {
    entries: Vec<ChatHistoryEntry>,
    selected_entry_id: Option<Uuid>,
    selected_visible_range: RangeInclusive<usize>,
    empty_state_message: String,
}

impl Default for HistoryPanelState {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            selected_entry_id: None,
            selected_visible_range: 0..=0,
            empty_state_message: DEFAULT_EMPTY_MESSAGE.to_string(),
        }
    }
}

impl HistoryPanelState {
    pub fn new_with_message(message: impl Into<String>) -> Self {
        Self {
            entries: Vec::new(),
            selected_entry_id: None,
            selected_visible_range: 0..=0,
            empty_state_message: message.into(),
        }
    }

    pub fn with_entries(entries: Vec<ChatHistoryEntry>) -> Self {
        let mut state = Self::default();
        state.set_entries(entries);
        state
    }

    pub fn from_records(records: Vec<ChatHistoryRecord>) -> Self {
        let entries = records
            .into_iter()
            .map(ChatHistoryEntry::from)
            .collect::<Vec<_>>();
        Self::with_entries(entries)
    }

    pub fn set_entries(&mut self, entries: Vec<ChatHistoryEntry>) {
        self.entries = entries;
        if let Some(last_id) = self.entries.last().map(|entry| entry.id) {
            self.selected_entry_id = Some(last_id);
            self.update_visible_range(self.entries.len().saturating_sub(1));
        } else {
            self.selected_entry_id = None;
            self.selected_visible_range = 0..=0;
        }
        tracing::info!("HistoryPanelState loaded {} entries", self.entries.len());
    }

    pub fn entries(&self) -> &[ChatHistoryEntry] {
        &self.entries
    }

    pub fn selected_entry(&self) -> Option<&ChatHistoryEntry> {
        let id = self.selected_entry_id?;
        self.entries.iter().find(|entry| entry.id == id)
    }

    pub fn selected_index(&self) -> Option<usize> {
        let id = self.selected_entry_id?;
        self.entries.iter().position(|entry| entry.id == id)
    }

    pub fn selected_entry_id(&self) -> Option<Uuid> {
        self.selected_entry_id
    }

    pub fn selected_visible_range(&self) -> RangeInclusive<usize> {
        self.selected_visible_range.clone()
    }

    pub fn empty_state_message(&self) -> &str {
        &self.empty_state_message
    }

    pub fn set_empty_state_message(&mut self, message: impl Into<String>) {
        self.empty_state_message = message.into();
    }

    pub fn append_entry(&mut self, entry: ChatHistoryEntry) {
        let entry_id = entry.id;
        self.entries.push(entry);
        self.selected_entry_id = Some(entry_id);
        let last_index = self.entries.len().saturating_sub(1);
        self.update_visible_range(last_index);
        tracing::debug!(entry_id = %entry_id, "Appended chat history entry");
    }

    pub fn move_selection_up(&mut self) {
        if let Some(current) = self.selected_index() {
            let previous = current.saturating_sub(1);
            if previous < self.entries.len() {
                if let Some(entry) = self.entries.get(previous) {
                    let entry_id = entry.id;
                    self.selected_entry_id = Some(entry_id);
                    self.update_visible_range(previous);
                    tracing::debug!(selected = %entry_id, "History selection moved up");
                }
            }
        }
    }

    pub fn move_selection_down(&mut self) {
        if let Some(current) = self.selected_index() {
            let next = min(current + 1, self.entries.len().saturating_sub(1));
            if next < self.entries.len() {
                if let Some(entry) = self.entries.get(next) {
                    let entry_id = entry.id;
                    self.selected_entry_id = Some(entry_id);
                    self.update_visible_range(next);
                    tracing::debug!(selected = %entry_id, "History selection moved down");
                }
            }
        } else if !self.entries.is_empty() {
            let last_index = self.entries.len().saturating_sub(1);
            let entry_id = self.entries[last_index].id;
            self.selected_entry_id = Some(entry_id);
            self.update_visible_range(last_index);
        }
    }

    pub fn select_entry(&mut self, entry_id: Option<Uuid>) {
        if let Some(id) = entry_id {
            if self.entries.iter().any(|entry| entry.id == id) {
                self.selected_entry_id = Some(id);
                if let Some(index) = self.entries.iter().position(|entry| entry.id == id) {
                    self.update_visible_range(index);
                }
                tracing::debug!(selected = %id, "Selected chat history entry");
                return;
            }
        }

        self.selected_entry_id = None;
        self.selected_visible_range = 0..=0;
    }

    pub fn ensure_latest_selected(&mut self) {
        if !self.entries.is_empty() && self.selected_entry_id.is_none() {
            let index = self.entries.len() - 1;
            self.selected_entry_id = Some(self.entries[index].id);
            self.update_visible_range(index);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn update_visible_range(&mut self, index: usize) {
        let start = index.saturating_sub(4);
        self.selected_visible_range = start..=index;
    }
}
