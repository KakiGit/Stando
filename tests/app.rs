use chrono::Utc;
use stando::app::App;
use stando::history::{ChatHistoryRecord, ChatRole};
use stando::history_panel::HistoryPanelState;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[test]
fn normalized_ai_query_trims_and_returns_text() {
    let normalized = App::normalized_ai_query("  ask the ai to fetch  ");
    assert_eq!(normalized.as_deref(), Some("ask the ai to fetch"));
}

#[test]
fn normalized_ai_query_ignores_empty_or_whitespace() {
    assert!(App::normalized_ai_query("").is_none());
    assert!(App::normalized_ai_query("    ").is_none());
    assert!(App::normalized_ai_query("\n\t").is_none());
}

#[tokio::test]
async fn selected_history_chat_id_generates_new_id_when_history_empty() {
    let state = Arc::new(RwLock::new(HistoryPanelState::default()));
    let chat_id = App::selected_history_chat_id(state.clone()).await;
    let chat_id = chat_id.expect("expected helper to create a chat id");
    assert!(!chat_id.is_empty());
}

#[tokio::test]
async fn selected_history_chat_id_returns_none_when_selection_missing() {
    let record = ChatHistoryRecord {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        role: ChatRole::User,
        summary: "initial-round".to_string(),
        content: "the content".to_string(),
        source: None,
        chat_hash: Some("existing-chat".to_string()),
    };
    let mut state = HistoryPanelState::from_records(vec![record]);
    state.select_chat(None);
    let guard = Arc::new(RwLock::new(state));
    assert!(App::selected_history_chat_id(guard).await.is_none());
}

#[tokio::test]
async fn selected_history_chat_id_returns_new_id_when_new_chat_selected() {
    let record = ChatHistoryRecord {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        role: ChatRole::User,
        summary: "existing".to_string(),
        content: "prompt".to_string(),
        source: None,
        chat_hash: Some("existing-chat".to_string()),
    };
    let mut state = HistoryPanelState::from_records(vec![record]);
    state.select_new_chat();
    let guard = Arc::new(RwLock::new(state));
    let chat_id = App::selected_history_chat_id(guard).await;
    let chat_id = chat_id.expect("expected new chat id");
    assert_ne!(chat_id, "existing-chat");
}
