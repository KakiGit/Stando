# Quickstart — AI Mode Chat Grouping

1. **Build the app**
   - Run `cargo fmt && cargo clippy && cargo test` from the repository root to ensure tooling and tests pass.
   - Create a local build with `cargo run --release` (or `cargo run` for debug) if you want UI timing insights.

2. **Populate history**
   - Launch Stando with AI mode enabled and drive several conversations in succession so each chat generates multiple rounds.  
   - Alternatively, pre-fill `~/.stando/chat-history.json` with synthetic `ChatHistoryRecord`s that include differing `chat_hash` values (or omit/`null` to exercise the “Recent Chat” fallback).

3. **Verify history panel behavior**
   - Open the history panel and confirm the list shows one row per chat hash, including metadata (hash, last speaker, timestamp, round count).  
   - Select each chat entry: the detail pane must render every round in earliest-to-latest order, separated by role/timestamp labels.  
   - While keeping a chat selected, trigger new rounds (or artificial sync) and ensure the list refreshes without losing the selection (the updated chat should stay focused).  
   - Confirm empty and single-chat states show the placeholder messaging from `DEFAULT_EMPTY_MESSAGE`.

4. **Regression checklist**
   - History selections are still navigable using arrow keys, and focus stays within the grouped list.  
   - No individual conversation round appears as a standalone row anymore.  
   - The “Recent Chat” fallback displays rounds lacking hashes so that no data is orphaned.
