use chrono::{DateTime, TimeZone, Utc};
use stando::history::{
    ChatGroup, ChatGroupMetadata, ChatHistoryRecord, ChatRole, ConversationRound,
};
use stando::history_panel::{HistoryPanelState, NEW_CHAT_DETAIL_MESSAGE};
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

fn round(id: u128, role: ChatRole, timestamp: DateTime<Utc>, content: &str) -> ConversationRound {
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
    let mut state = HistoryPanelState::with_chats(vec![chat_group("chat-new", timestamp)], None);
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
    let mut state = HistoryPanelState::with_chats(vec![chat_group("chat-move", timestamp)], None);
    assert!(!state.is_new_chat_selected());
    state.move_selection_down();
    assert!(state.is_new_chat_selected());
    state.move_selection_up();
    assert_eq!(state.selected_chat_id(), Some("chat-move"));
    assert!(!state.is_new_chat_selected());
}
