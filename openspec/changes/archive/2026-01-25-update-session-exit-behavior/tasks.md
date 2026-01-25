## 1. Implementation
- [x] 1.1 Remove the `TrayIcon`/daemon plumbing from `App`, eliminating the tray menu and any background-running helpers so the binary only runs while the window is visible.
- [x] 1.2 Make sure the app shows the search window as part of startup and that no additional actions are required before the UI is usable.
- [x] 1.3 Update the Escape-key handler to hide the window and then quit the GTK application so the process terminates instead of remaining in the tray.
- [x] 1.4 Close the application once a selected search result is opened so the process does not stay resident afterward.

## 2. Validation
- [x] 2.1 Run `cargo fmt`, `cargo build`, `cargo test`, and `cargo clippy` to ensure formatting and correctness.
- [x] 2.2 Manual verification: run `cargo run`, confirm the search window opens immediately, and check that pressing Esc closes the app without leaving a background process or tray icon (manual GUI run pending in this environment).
