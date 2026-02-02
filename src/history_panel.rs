use crate::history::{
    chat_timeline_by_hash, grouped_history_with_selection, normalize_chat_hash, ChatGroup,
    ChatGroupMetadata, ChatHistoryRecord, ChatSummary, ConversationRound, RECENT_CHAT_ID,
};
use chrono::Local;
use std::ops::RangeInclusive;
use crate::history::GLOBAL_HISTORY;
use glib;

pub const DEFAULT_EMPTY_MESSAGE: &str =
    "No chats yet. Each entry groups a full chat and shows every round when selected.";
pub const NEW_CHAT_DETAIL_MESSAGE: &str =
    "Send a prompt while this entry is selected to start a fresh AI conversation.";

#[derive(Debug, Clone)]
pub struct HistoryPanelState {
    chats: Vec<ChatGroup>,
    selected_chat_id: Option<String>,
    selected_visible_range: RangeInclusive<usize>,
    empty_state_message: String,
    selected_new_chat: bool,
}

impl Default for HistoryPanelState {
    fn default() -> Self {
        Self {
            chats: Vec::new(),
            selected_chat_id: None,
            selected_visible_range: 0..=0,
            empty_state_message: DEFAULT_EMPTY_MESSAGE.to_string(),
            selected_new_chat: false,
        }
    }
}

impl HistoryPanelState {
    pub fn new_with_message(message: impl Into<String>) -> Self {
        Self {
            chats: Vec::new(),
            selected_chat_id: None,
            selected_visible_range: 0..=0,
            empty_state_message: message.into(),
            selected_new_chat: false,
        }
    }

    pub fn with_chats(chats: Vec<ChatGroup>, selected_chat_id: Option<String>) -> Self {
        let mut state = Self::default();
        state.set_chats(chats, selected_chat_id);
        state
    }

    pub fn from_records(records: Vec<ChatHistoryRecord>) -> Self {
        let (chats, selected_chat_id) = grouped_history_with_selection(records);
        Self::with_chats(chats, selected_chat_id)
    }

    pub fn update_from_records(&mut self, records: Vec<ChatHistoryRecord>) {
        let (chats, default_selected) = grouped_history_with_selection(records);
        let selected_chat_id = self
            .selected_chat_id
            .clone()
            .and_then(|id| chats.iter().any(|chat| chat.id == id).then_some(id))
            .or(default_selected);
        self.set_chats(chats, selected_chat_id);
    }

    pub fn set_chats(&mut self, chats: Vec<ChatGroup>, selected_chat_id: Option<String>) {
        self.chats = chats;
        self.selected_new_chat = false;
        let resolved = selected_chat_id.and_then(|id| {
            if self.chats.iter().any(|chat| chat.id == id) {
                Some(id)
            } else {
                None
            }
        });
        self.selected_chat_id = resolved.or_else(|| self.chats.first().map(|chat| chat.id.clone()));
        if let Some(index) = self.selected_index() {
            self.update_visible_range(index);
        } else {
            self.selected_visible_range = 0..=0;
        }
        tracing::info!("HistoryPanelState loaded {} chats", self.chats.len());
    }

    pub fn chats(&self) -> &[ChatGroup] {
        &self.chats
    }

    pub fn chat_summaries(&self) -> Vec<ChatSummary> {
        self.chats
            .iter()
            .map(|chat| {
                let last_timestamp = chat.metadata.last_timestamp;
                let local_time = last_timestamp.with_timezone(&Local);
                let display_label = format!(
                    "{} · {}",
                    chat.metadata.last_speaker.label(),
                    local_time.format("%Y-%m-%d %H:%M:%S")
                );
                // Compute a short preview of the first conversation round
                let first_round_summary = if let Some(first) = chat.rounds.first() {
                    // Replicate the summarization logic used by App::summarize_content
                    let trimmed = first.content.trim();
                    if trimmed.is_empty() {
                        "[no content]".to_string()
                    } else {
                        let first_line = trimmed.lines().next().unwrap_or("").trim();
                        let mut preview = first_line.chars().take(80).collect::<String>();
                        if preview.is_empty() {
                            "[no preview]".to_string()
                        } else {
                            preview
                        }
                    }
                } else {
                    "[empty]".to_string()
                };
                ChatSummary {
                    chat_hash: chat.id.clone(),
                    display_label,
                    last_actor: chat.metadata.last_speaker,
                    last_timestamp,
                    round_count: chat.rounds.len(),
                    first_round_summary,
                }
            })
            .collect()
    }

    pub fn selected_chat(&self) -> Option<&ChatGroup> {
        let id = self.selected_chat_id.as_deref()?;
        self.chats.iter().find(|chat| chat.id == id)
    }

    pub fn selected_index(&self) -> Option<usize> {
        if self.selected_new_chat {
            return self.new_chat_row_index();
        }
        let id = self.selected_chat_id.as_deref()?;
        self.chats.iter().position(|chat| chat.id == id)
    }

    pub fn selected_chat_id(&self) -> Option<&str> {
        self.selected_chat_id.as_deref()
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

    pub fn row_count(&self) -> usize {
        let base = self.chats.len();
        if self.has_new_chat_row() {
            base + 1
        } else {
            base
        }
    }

    pub fn has_new_chat_row(&self) -> bool {
        !self.chats.is_empty()
    }

    pub fn new_chat_row_index(&self) -> Option<usize> {
        self.has_new_chat_row().then(|| self.chats.len())
    }

    pub fn is_new_chat_row_index(&self, index: usize) -> bool {
        self.new_chat_row_index() == Some(index)
    }

    pub fn is_new_chat_selected(&self) -> bool {
        self.selected_new_chat
    }

    pub fn select_new_chat(&mut self) {
        if self.new_chat_row_index().is_none() {
            return;
        }
        self.selected_chat_id = None;
        self.selected_new_chat = true;
        if let Some(index) = self.selected_index() {
            self.update_visible_range(index);
        }
        tracing::debug!("Selected new chat row");
    }

    pub fn append_record(&mut self, record: ChatHistoryRecord) {
        let chat_id = record.chat_id();
        let round = ConversationRound::from(&record);
        let mut appended = false;
        if let Some(chat) = self.chats.iter_mut().find(|chat| chat.id == chat_id) {
            chat.rounds.push(round.clone());
            chat.rounds.sort_by_key(|round| round.timestamp);
            if record.timestamp >= chat.metadata.last_timestamp {
                chat.metadata.last_timestamp = record.timestamp;
                chat.metadata.last_speaker = record.role;
                chat.metadata.summary = record.summary.clone();
            }
            appended = true;
        }

        if !appended {
            self.chats.push(ChatGroup {
                id: chat_id.clone(),
                metadata: ChatGroupMetadata {
                    last_speaker: record.role,
                    last_timestamp: record.timestamp,
                    summary: record.summary.clone(),
                },
                rounds: vec![round],
            });
        }

        self.chats.sort_by(|left, right| {
            right
                .metadata
                .last_timestamp
                .cmp(&left.metadata.last_timestamp)
        });

        if self.selected_chat_id.is_none() || self.selected_new_chat {
            self.selected_chat_id = Some(chat_id);
            self.selected_new_chat = false;
        }

        if let Some(index) = self.selected_index() {
            self.update_visible_range(index);
        }
        tracing::debug!(
            selected_chat_id = ?self.selected_chat_id,
            "Appended chat history round"
        );
    }

    pub fn move_selection_up(&mut self) {
        if let Some(current) = self.selected_index() {
            if current > 0 {
                self.select_row_index(current - 1);
            }
        }
    }

    pub fn move_selection_down(&mut self) {
        let row_count = self.row_count();
        if row_count == 0 {
            return;
        }

        if let Some(current) = self.selected_index() {
            if current + 1 < row_count {
                self.select_row_index(current + 1);
            }
        } else if !self.chats.is_empty() {
            self.select_row_index(self.chats.len().saturating_sub(1));
        }
    }

    /// Delete the currently selected chat from the history.
    /// If the selected chat is the last one, the previous chat becomes selected.
    /// If no chats remain, clears selection.
    pub fn delete_selected_chat(&mut self) {
        // Only proceed if there is a selected chat row.
        let index_opt = self.selected_index();
        if let Some(index) = index_opt {
            // Keep track of the chat id that is being deleted.
            let deleted_id = self.selected_chat_id.clone();
            // Remove the chat at the index.
            if index < self.chats.len() {
                self.chats.remove(index);
            }
            // After removal, adjust selection.
            if self.chats.is_empty() {
                self.selected_chat_id = None;
                self.selected_new_chat = false;
                self.selected_visible_range = 0..=0;
            } else {
                // If we removed the last item, select the new last; otherwise keep same index.
                let new_index = if index >= self.chats.len() {
                    self.chats.len() - 1
                } else {
                    index
                };
                self.select_row_index(new_index);
            }
            // Persist deletion to chat history JSON if a global history is set.
            if let Some(id) = deleted_id {
                if let Some(history) = GLOBAL_HISTORY.get().cloned() {
                    // Spawn async task to delete records.
                    glib::MainContext::default().spawn_local(async move {
                        // Fire and forget; ignore the result.
                        let _ = history
                            .delete_chat_records_by_hash(&id)
                            .await;
                    });
                }
            }
        }
    }

    pub fn select_chat(&mut self, chat_id: Option<String>) {
        if let Some(id) = chat_id {
            if self.chats.iter().any(|chat| chat.id == id) {
                self.selected_new_chat = false;
                self.selected_chat_id = Some(id.clone());
                if let Some(index) = self.selected_index() {
                    self.update_visible_range(index);
                }
                tracing::debug!(selected = %id, "Selected chat history entry");
                return;
            }
        }

        self.selected_chat_id = None;
        self.selected_new_chat = false;
        self.selected_visible_range = 0..=0;
    }

    pub fn select_chat_by_index(&mut self, index: usize) {
        self.select_row_index(index);
    }

    fn select_row_index(&mut self, index: usize) {
        if self.is_new_chat_row_index(index) {
            self.select_new_chat();
        } else if let Some(chat) = self.chats.get(index) {
            self.select_chat(Some(chat.id.clone()));
        }
    }

    pub fn chat_id_at_index(&self, index: usize) -> Option<String> {
        self.chats.get(index).map(|chat| chat.id.clone())
    }

    pub fn ensure_latest_selected(&mut self) {
        self.selected_new_chat = false;
        if !self.chats.is_empty() && self.selected_chat_id.is_none() {
            let index = 0;
            self.selected_chat_id = Some(self.chats[index].id.clone());
            self.update_visible_range(index);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.chats.is_empty()
    }

    pub fn selected_timeline_text(&self) -> String {
        if self.selected_new_chat {
            return NEW_CHAT_DETAIL_MESSAGE.to_string();
        }
        let chat_id = match self.selected_chat_id.as_deref() {
            Some(id) => id,
            None => return self.empty_state_message.clone(),
        };
        let rounds = match chat_timeline_by_hash(&self.chats, chat_id) {
            Some(rounds) if !rounds.is_empty() => rounds,
            _ => return self.empty_state_message.clone(),
        };
        format_timeline(&rounds)
    }

    pub fn recent_chat_id() -> String {
        RECENT_CHAT_ID.to_string()
    }

    pub fn normalize_selected_chat_id(&self) -> String {
        normalize_chat_hash(self.selected_chat_id.as_deref())
    }

    fn update_visible_range(&mut self, index: usize) {
        let start = index.saturating_sub(4);
        self.selected_visible_range = start..=index;
    }
}

fn format_timeline(rounds: &[ConversationRound]) -> String {
    let mut ordered = rounds.to_vec();
    ordered.sort_by_key(|round| round.timestamp);
    let mut output = String::new();
    for (idx, round) in ordered.iter().enumerate() {
        let local_time = round.timestamp.with_timezone(&Local);
        output.push_str(&format!(
            "{} • {}\n",
            local_time.format("%Y-%m-%d %H:%M:%S"),
            round.role.label()
        ));
        output.push_str(round.content.trim());
        if idx + 1 < ordered.len() {
            output.push_str("\n\n");
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::ChatRole;
    use chrono::{DateTime, TimeZone, Utc};
    use uuid::Uuid;

    fn chat_record(
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

    fn round(
        id: u128,
        role: ChatRole,
        timestamp: DateTime<Utc>,
        content: &str,
    ) -> ConversationRound {
        ConversationRound {
            round_id: Uuid::from_u128(id),
            role,
            timestamp,
            content: content.to_string(),
            source: None,
        }
    }

    fn chat_group(id: &str, timestamp: DateTime<Utc>) -> ChatGroup {
        ChatGroup {
            id: id.to_string(),
            metadata: ChatGroupMetadata {
                last_speaker: ChatRole::Assistant,
                last_timestamp: timestamp,
                summary: "summary".to_string(),
            },
            rounds: Vec::new(),
        }
    }

    #[test]
    fn timeline_orders_rounds_chronologically() {
        let chat = ChatGroup {
            id: "chat-a".to_string(),
            metadata: ChatGroupMetadata {
                last_speaker: ChatRole::Assistant,
                last_timestamp: Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap(),
                summary: "summary".to_string(),
            },
            rounds: vec![
                round(
                    1,
                    ChatRole::Assistant,
                    Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap(),
                    "second",
                ),
                round(
                    2,
                    ChatRole::User,
                    Utc.with_ymd_and_hms(2026, 1, 1, 11, 0, 0).unwrap(),
                    "first",
                ),
            ],
        };
        let state = HistoryPanelState::with_chats(vec![chat], Some("chat-a".to_string()));
        let timeline = state.selected_timeline_text();
        let first_pos = timeline.find("first").unwrap();
        let second_pos = timeline.find("second").unwrap();
        assert!(first_pos < second_pos);
        assert!(timeline.contains("You"));
        assert!(timeline.contains("AI"));
    }

    #[test]
    fn preserves_selection_on_refresh() {
        let records = vec![ChatHistoryRecord {
            id: Uuid::from_u128(1),
            timestamp: Utc.with_ymd_and_hms(2026, 1, 1, 10, 0, 0).unwrap(),
            role: ChatRole::User,
            summary: "alpha".to_string(),
            content: "alpha".to_string(),
            source: None,
            chat_hash: Some("alpha".to_string()),
        }];
        let mut state = HistoryPanelState::from_records(records);
        state.select_chat(Some("alpha".to_string()));

        let refreshed = vec![
            ChatHistoryRecord {
                id: Uuid::from_u128(1),
                timestamp: Utc.with_ymd_and_hms(2026, 1, 1, 10, 0, 0).unwrap(),
                role: ChatRole::User,
                summary: "alpha".to_string(),
                content: "alpha".to_string(),
                source: None,
                chat_hash: Some("alpha".to_string()),
            },
            ChatHistoryRecord {
                id: Uuid::from_u128(2),
                timestamp: Utc.with_ymd_and_hms(2026, 1, 1, 11, 0, 0).unwrap(),
                role: ChatRole::Assistant,
                summary: "beta".to_string(),
                content: "beta".to_string(),
                source: None,
                chat_hash: Some("beta".to_string()),
            },
        ];
        state.update_from_records(refreshed);
        assert_eq!(state.selected_chat_id(), Some("alpha"));
    }

    #[test]
    fn append_record_keeps_chronological_rounds() {
        let initial = chat_record(
            1,
            "chat-a",
            ChatRole::User,
            Utc.with_ymd_and_hms(2026, 1, 1, 10, 0, 0).unwrap(),
            "first",
        );
        let mut state = HistoryPanelState::from_records(vec![initial]);
        let appended = chat_record(
            2,
            "chat-a",
            ChatRole::Assistant,
            Utc.with_ymd_and_hms(2026, 1, 1, 11, 0, 0).unwrap(),
            "second",
        );
        state.append_record(appended);
        let chat = state.selected_chat().expect("expected chat selected");
        assert_eq!(chat.rounds.len(), 2);
        assert!(chat.rounds[0].timestamp <= chat.rounds[1].timestamp);
    }

    #[test]
    fn selecting_new_chat_row_shows_instruction_message() {
        let timestamp = Utc.with_ymd_and_hms(2026, 1, 1, 10, 0, 0).unwrap();
        let mut state =
            HistoryPanelState::with_chats(vec![chat_group("chat-new", timestamp)], None);
        state.select_new_chat();
        assert!(state.is_new_chat_selected());
        assert_eq!(state.selected_index(), state.new_chat_row_index());
        assert_eq!(
            state.selected_timeline_text(),
            NEW_CHAT_DETAIL_MESSAGE.to_string()
        );
    }

    #[test]
    fn navigation_moves_into_and_out_of_new_chat_row() {
        let timestamp = Utc.with_ymd_and_hms(2026, 1, 1, 10, 0, 0).unwrap();
        let mut state =
            HistoryPanelState::with_chats(vec![chat_group("chat-move", timestamp)], None);
        assert!(!state.is_new_chat_selected());
        state.move_selection_down();
        assert!(state.is_new_chat_selected());
        state.move_selection_up();
        assert_eq!(state.selected_chat_id(), Some("chat-move"));
        assert!(!state.is_new_chat_selected());
    }
}
