# Quickstart

1. Run `cargo build` and `cargo run` from the repo root to launch Stando on Linux with GTK4.
2. Switch into AI mode by activating the AI input panel (same keyboard shortcuts as before, e.g., open the AI pane and focus the text box).
3. Select an existing chat from the sidebar so that a chat ID is chosen (per FR-002).
4. Type a non-whitespace message and hit Enter. Verify that the new message appears immediately at the bottom of the selected chat (SC-001/SC-002) and persists across restarts by checking `~/.stando/chat-history.json` after exit.
5. Try sending whitespace-only text – the UI should ignore it (FR-006).
6. Leave AI mode enabled but deselect all chats and attempt to send input. Confirm the UI shows a "No chat selected" error and that no message is appended anywhere (FR-005).
7. Start with a clean profile or remove `~/.stando/chat-history.json` so the history is empty, enable AI mode, and send a normal message. Confirm the UI does not show an error, a new chat appears with the message, and the entry persists across restarts (FR-005).
