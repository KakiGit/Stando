## 1. Implementation
- [x] 1.1 Update `SearchWindow::new` so the window computes the primary monitor bounds, sizes itself to 50% of that width and height, and centers itself before becoming visible while keeping an acceptable fallback when display metrics are unavailable.
- [x] 1.2 Capture the new expectation in the `session-lifecycle` spec delta so the requirement explicitly mandates the centered half-screen size at launch.
- [x] 1.3 Run `cargo fmt`, `cargo build`, `cargo clippy`, and `cargo test` to ensure the Rust codebase continues to build cleanly.
- [x] 1.4 Run `openspec validate center-window-half-screen --strict --no-interactive` to verify the added spec delta is well-formed and complete.
