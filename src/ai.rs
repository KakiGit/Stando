use crate::history::UsageHistory;
use crate::history::ChatHistoryRecord;
use crate::history_panel::HistoryPanelState;
use crate::logging;
use crate::search::{SearchEngine, SearchResult};
use anyhow::{Context, Result};
use glib::MainContext;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    #[allow(dead_code)]
    delta: Option<Delta>,
    message: Option<Message>,
}

#[derive(Debug, Deserialize)]
struct Delta {
    #[allow(dead_code)]
    content: Option<String>,
}

pub struct AIService {
    client: Client,
    api_key: String,
    search_engine: Arc<SearchEngine>,
}

impl AIService {
    pub fn new(api_key: String, search_engine: Arc<SearchEngine>) -> Self {
        let _log_guard = logging::function_guard("AIService::new");
        Self {
            client: Client::new(),
            api_key,
            search_engine,
        }
    }

    pub fn process_query_blocking(&self, query: &str) -> Result<String> {
        let log_guard = logging::function_guard("AIService::process_query_blocking");
        let result = (|| {
            // Parse @ references (async search engine) on this background thread
            let context = futures::executor::block_on(self.parse_references(query))?;

            // Build messages
            let mut messages = vec![Message {
                role: "system".to_string(),
                content: "You are a helpful assistant integrated into a search application. Provide concise, helpful responses.".to_string(),
            }];

            if !context.is_empty() {
                let context_str = context
                    .iter()
                    .map(|(name, content)| format!("File: {}\n{}", name, content))
                    .collect::<Vec<_>>()
                    .join("\n\n");

                messages.push(Message {
                    role: "user".to_string(),
                    content: format!("Context from referenced files:\n\n{}", context_str),
                });
            }

            messages.push(Message {
                role: "user".to_string(),
                content: query.to_string(),
            });

            // Make API request (non-streaming for simplicity)
            let request = ChatRequest {
                model: "gpt-3.5-turbo".to_string(),
                messages,
                stream: false,
            };

            let response = self
                .client
                .post("https://api.openai.com/v1/chat/completions")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&request)
                .send()
                .context("Failed to send request to OpenAI API")?;

            if !response.status().is_success() {
                let error_text = response.text().unwrap_or_default();
                anyhow::bail!("OpenAI API error: {}", error_text);
            }

            let chat_response: ChatResponse =
                response.json().context("Failed to parse response")?;

            // Extract response text
            let mut content = String::new();
            for choice in chat_response.choices {
                if let Some(message) = choice.message {
                    content.push_str(&message.content);
                }
            }

            Ok(content)
        })();
        if result.is_err() {
            log_guard.mark_error();
        }
        result
    }

    async fn parse_references(&self, query: &str) -> Result<Vec<(String, String)>> {
        let log_guard = logging::function_guard("AIService::parse_references");
        let result = async {
            let mut context = Vec::new();

            // Find all @ references
            let words: Vec<&str> = query.split_whitespace().collect();
            for word in words {
                if word.starts_with('@') {
                    let reference = word.strip_prefix('@').unwrap();

                    if let Some(result) = self.search_engine.find_by_reference(reference).await {
                        match result {
                            SearchResult::File { path, .. } => {
                                if let Ok(content) = std::fs::read_to_string(&path) {
                                    context.push((path.display().to_string(), content));
                                }
                            }
                            SearchResult::Application { name, .. } => {
                                // For applications, just include the name in context
                                context.push((
                                    format!("Application: {}", name),
                                    format!("Application reference: {}", name),
                                ));
                            }
                            SearchResult::Text { .. } => {
                                // Text results (AI responses) are not used as references
                            }
                        }
                    }
                }
            }

            Ok(context)
        }
        .await;
        if result.is_err() {
            log_guard.mark_error();
        }
        result
    }

    pub fn notify_history_state(
        history_state: Arc<RwLock<HistoryPanelState>>,
        usage_history: Arc<UsageHistory>,
        record: ChatHistoryRecord,
    ) {
        MainContext::default().spawn_local(async move {
            let mut guard = history_state.write().await;
            guard.append_record(record.clone());
            drop(guard);
            usage_history.record_chat_entry(record).await;
        });
    }
}
