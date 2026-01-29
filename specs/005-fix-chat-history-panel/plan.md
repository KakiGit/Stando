# Implementation Plan: AI Mode Chat History Panel — Chat Grouping

**Branch**: `005-fix-chat-history-panel` | **Date**: 2026-01-29 | **Spec**: specs/005-fix-chat-history-panel/spec.md  
**Input**: Feature specification from `/workspaces/Stando/specs/005-fix-chat-history-panel/spec.md`

## Summary

Group every history row by its chat hash so the list no longer shows individual rounds, surface metadata (hash, last actor, timestamp) in each entry, render the entire chat timeline in chronological order when a chat is selected, and keep the selection stable while syncing/appending rounds—with a “Recent Chat” fallback capturing hash-less rounds so nothing is orphaned.

## Technical Context

The change touches the existing Rust 2021 desktop app (Cargo package `stando` 0.1.0) that already wires GTK4/libadwaita UIs, `tokio` async loading, and `tracing` diagnostics.

**Language/Version**: Rust 2021 (cargo stando 0.1.0, current toolchain matching the `edition = "2021"` crate)  
**Primary Dependencies**: `gtk4`/`libadwaita`/`gdk4`/`glib`/`gio` for the UI, `tokio` (full runtime) for background history loading, `tracing` + `tracing-subscriber`, `serde` + `serde_json`, `reqwest` (AI HTTP stack), `chrono`, `uuid`, and the existing history helpers in `UsageHistory`.  
**Storage**: Local JSON files under `~/.stando` (`chat-history.json` for the rounds, `search-history.json` for counters); history data is read into `ChatHistoryRecord`s and will now extend to `ChatGroup`s before reaching the UI.  
**Testing**: `cargo fmt`, `cargo clippy`, and `cargo test` remain the enforcement tools; add targeted unit tests for grouping logic in `src/history_panel.rs`/`src/history.rs` and, if feasible, integration smoke tests for the `HistoryPanelState`.  
**Target Platform**: Linux desktop (GTK4/libadwaita with GNOME-style chrome).  
**Project Type**: Single Rust desktop binary with UI modules under `src/` and CLI-like entrypoints in `src/main.rs`, `src/app.rs`, `src/ui.rs`, and `src/window.rs`.  
**Performance Goals**: Meet SC-002 by updating the detail pane within 1 second after selection, ensure grouping runs off the UI thread so p95 stays low, and keep the dev-defined 95% unique-chat display goal from SC-001.  
**Constraints**: No new network calls outside the existing AI mode; history refreshes must preserve selection and reuse the current JSON file(s) without blocking the GTK main loop.  
**Scale/Scope**: Local single-user history panel; expect a few hundred chat entries (each tokenized into tens of rounds) and keep in-memory grouping bound to those limits; the UI should not assume more than thousands of rounds.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Keyboard-first UX**: The `ListBox` rows remain focused at the chat level; `HistoryPanelState` now updates selection via chat hashes instead of round IDs, and grouping/timeline assembly happens in background tasks before the GTK thread applies the new model.  
- **Local-first/privacy**: Only the pre-existing `~/.stando/chat-history.json` file is read/written; no new remote endpoints are introduced for this feature.  
- **Daemon reliability**: `UsageHistory`’s mutex-protected cache keeps a single instance of the file handles, and selection-preserving syncs reuse the current hotkey/watchdog plumbing—no new daemon concerns are introduced.  
- **Resource efficiency**: Grouping logic reuses existing round streams and only adds in-memory aggregation per chat hash; the timeline strings are rendered via the same `TextView`, so we avoid building per-round GTK widgets.  
- **Linux integration**: GTK4/libadwaita list/timeline controls and the existing `com.stando.App.desktop` launcher remain unchanged; the plan only redistributes data feeding those widgets.

## Project Structure

### Documentation (this feature)

```text
specs/005-fix-chat-history-panel/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/
    └── history.openapi.yaml
```

### Source Code (repository root)

```text
src/
├── history.rs           # UsageHistory persistence (chat-history.json)
├── history_panel.rs     # HistoryPanelState, selection handling, UI bindings
├── ai.rs
├── app.rs
├── ui.rs
├── window.rs
└── logging.rs
```

**Structure Decision**: The feature lives entirely inside the single Rust crate (`src/` plus tests). History persistence changes are staged in `src/history.rs` while UI/selection logic stays in `src/history_panel.rs`; no new modules or binaries are required.

## Complexity Tracking

No Constitution violations introduced; no additional justification required.
