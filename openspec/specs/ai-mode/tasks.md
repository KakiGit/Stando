## 1. Implementation
* **1.1** Add a `--ai-mode`/`-a` flag to the CLI parser (`src/main.rs`).
* **1.2** Seed the `ai_mode` state before callbacks are registered; update button indicator accordingly.
* **1.3** Persist chat entries with an optional `chat_hash`. Use `Uuid::new_v4()` for new sessions.
* **1.4** Group chat entries by `chat_hash` when loading history; entries without a hash start a new session.
* **1.5** Ensure the history panel displays a list of chat sessions and a content pane that shows all rounds in chronological order.
* **1.6** Update README with the new AI mode flag and chat history usage.

## 2. Validation
* **2.1** `cargo fmt` passes.
* **2.2** `cargo build` produces a binary.
* **2.3** `cargo test` (if applicable) passes.
* **2.4** Verify that launching with `--ai-mode` shows the AI indicator and that new chat sessions are created with proper hash IDs.
