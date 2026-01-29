# Implementation Plan: AI Conversation Append

**Branch**: `006-append-conversation` | **Date**: 2026-01-31 | **Spec**: specs/006-append-conversation/spec.md
**Input**: Feature specification from `/specs/006-append-conversation/spec.md`

## Summary
When a user is in AI mode, the new text must be appended to the currently selected chat so that the conversation order is preserved, empty/whitespace messages are ignored, and the UI reports errors when no chat is selected. The implementation reuses the existing chat-selection and history persistence helpers, performs the append on the message pipeline, and updates the UI immediately while persisting to the local JSON storage that backs `ChatHistoryRecord`.

## Technical Context
**Language/Version**: Rust 2021 edition (codebase currently builds with Rust 1.77+).
**Primary Dependencies**: `gtk4`, `libadwaita`, `gdk4`, `glib`, `gio`, `tracing`, `tracing-subscriber`, `tokio`, `reqwest`, `serde`, `serde_json`, `global-hotkey`.
**Storage**: Local JSON files under `~/.stando` (`chat-history.json`, `search-history.json`) managed via `UsageHistory` helpers and `HistoryPersistence` abstractions.
**Testing**: `cargo fmt`, `cargo clippy`, `cargo test` (unit and integration tests as they exist in the Rust crate).
**Target Platform**: Linux desktop (GTK4/libadwaita UI) with Arch Linux as the reference distribution.
**Project Type**: Single Rust desktop application (Cargo package `stando` 0.1.0) combining UI and AI features in one crate.
**Performance Goals**: Append operations should complete within 1 second, keep the UI thread responsive, and avoid blocking the daemon; storage I/O happens asynchronously.
**Constraints**: Keyboard-first UX, no new background polling, AI-side network calls must only run when AI mode is explicitly enabled with a configured API key, and existing resource/performance budgets (<100ms UI lock, bounded memory for history caching) must hold.
**Scale/Scope**: Focused on the single-user local experience; no multi-tenant requirements and no external API beyond the configured AI service.

## Constitution Check
- Keyboard-first UX: The append flow uses the same keyboard shortcuts that already focus the AI panel, and all validation/persistence work runs asynchronously so the UI thread remains responsive.
- Local-first & privacy: Message submission only happens locally; AI network calls stay disabled until the user enables AI mode with a stored API key, and appended history is written to local files under `~/.stando`.
- Daemon reliability: Changes stay within the UI messaging pipeline and do not touch the daemon/hotkey logic, so single-instance behavior and hotkey responsiveness remain intact.
- Resource efficiency: The feature reuses the existing history persistence stack, performs writes asynchronously, and does not introduce new polling loops, so idle CPU/memory budgets are unchanged.
- Linux integration: All changes occur in the Rust GTK4/libadwaita code (`src/`), so desktop integration and UI theming remain compatible.

## Project Structure
### Documentation (this feature)
```text
specs/006-append-conversation/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
└── tasks.md (future)
```
### Source Code (repository root)
```text
src/
├── main.rs
├── app.rs
├── ai.rs
├── history.rs
├── history_panel.rs
├── window.rs
├── ui.rs
├── config.rs
├── search.rs
└── logging.rs
assets/
├── style.css
specs/
├── 001-add-debug-logging/
├── 002-fix-ai-shortcut/
├── 003-force-window-top/
├── 004-add-ai-history-panel/
├── 005-fix-chat-history-panel/
└── 006-append-conversation/
```
**Structure Decision**: A single Cargo crate (`stando`) houses the GTK windowing, AI service integration, global hotkey handling, and history persistence, so all related logic is contained under `src/` with shared modules for transformers like `ai.rs` and `history.rs`.

## Complexity Tracking
No Constitution violations were introduced by this change, so no additional structural complexity is required.
