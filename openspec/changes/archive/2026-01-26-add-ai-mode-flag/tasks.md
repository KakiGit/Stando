## 1. Implementation
- [x] 1.1 Add a `--ai-mode`/`-a` flag to the CLI argument parser (`src/main.rs`) and forward the boolean value through the startup sequence.
- [x] 1.2 Update `App` so the incoming flag seeds the `ai_mode` state before callbacks are registered, the button indicator updates, and `reset_content`/`show_window` observe the seeded mode.
- [x] 1.3 Document the new flag in `README.md` (feature list, usage instructions) so users know how to start directly in AI mode.

## 2. Validation
- [x] 2.1 Run `cargo fmt`, `cargo build`, `cargo test`, and `cargo clippy` to ensure formatting, build, and lint expectations are satisfied (warnings about unused helpers already exist in the project).
