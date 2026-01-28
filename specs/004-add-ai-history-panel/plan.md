# Implementation Plan: AI History Panel

**Branch**: `004-add-ai-history-panel` | **Date**: 2026-01-26 | **Spec**: `/workspaces/Stando/specs/004-add-ai-history-panel/spec.md`
**Input**: Feature specification from `/workspaces/Stando/specs/004-add-ai-history-panel/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

AI mode’s bottom pane gets reimagined as a read-only detail box on the left and a chronological history list on the right so the active entry is always visible with arrow key navigation and the empty/sync states described in `/workspaces/Stando/specs/004-add-ai-history-panel/spec.md`. Research will focus on how to reuse `src/ui.rs`’s current list layout, how `src/history.rs` exposes persisted entries, and how to keep selection/network updates responsive within 2 seconds as described in the success criteria.

## Technical Context

**Language/Version**: Rust 2021 (Cargo package `stando` 0.1.0)  
**Primary Dependencies**: `gtk4`/`libadwaita` for composing the UI, `glib`/`gio` for main-loop integration, `tokio` for background/history loading, `tracing`/`tracing-subscriber` for diagnostics, and `reqwest` for AI service calls (existing stack from prior AI features).  
**Storage**: N/A at the feature level; chat history is already persisted elsewhere and surfaced via `src/history.rs`/`src/ai.rs`.  
**Testing**: `cargo test` plus `cargo clippy` to keep lint and unit checks green, with any new tests validating selection behavior and layout helpers.  
**Target Platform**: Linux desktop, GTK4/libadwaita-based UI.  
**Project Type**: Single Rust desktop application (one GTK-based binary).  
**Performance Goals**: Maintain the success criteria: history list and detail view appear within 2 seconds of opening AI mode, arrow key selection updates detail text within 1 second, and new entries keep list/text synchronized without blocking the UI thread.  
**Constraints**: Must stay keyboard-first, avoid blocking the GTK main loop (use async updates when loading history), reuse the existing layout shell so window size/metrics remain unchanged, and handle empty-state messaging without requiring network calls outside explicit AI mode.  
**Scale/Scope**: This is a single AI mode UI enhancement affecting the in-window panel and hotkey behavior; no new services or CLI flags are introduced.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Keyboard-first UX: The new panel keeps AI mode controls fully keyboard-operated (arrow navigation, list selection) and the update logic will use asynchronous selection handling so the GTK UI thread stays responsive.  
- Local-first/privacy: This feature only re-uses existing local chat history data; no new network calls are introduced beyond the AI mode interactions already gated behind the configured API key.  
- Daemon reliability: No changes to daemon lifecycle, hotkeys remain single-instance, and the AI panel continues to render within the main window, so the current start/stop behavior is preserved.  
- Resource efficiency: The panel reuses the existing window layout and defers history loads to the background (the history data is already cached), keeping CPU/memory impact minimal.  
- Linux integration: UI work stays inside GTK4/libadwaita components (`ListBox`, `TextView`, etc.), so desktop integration and styling conventions remain intact.

## Project Structure

### Documentation (this feature)

```text
/workspaces/Stando/specs/004-add-ai-history-panel/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
/workspaces/Stando/src/
├── main.rs              # Entrypoint wiring GTK/app lifecycle and AI toggle
├── app.rs               # Application state machine (AI mode state, hotkeys)
├── window.rs            # Window management helpers (geometry, presentation)
├── ui.rs                # Search window UI widgets (search bar + results box)
├── history.rs           # Chat history model & selection helpers
├── search.rs            # File/app search logic
├── ai.rs                # AI service layer (OpenAI requests and history references)
├── config.rs            # Shortcut and config defaults
├── logging.rs           # Tracing helpers
└── assets/              # Bundled CSS/resources
/workspaces/Stando/tests/
├── unit/
└── integration/
```

**Structure Decision**: The feature builds on the existing single Cargo binary under `/workspaces/Stando/src/`, updating UI-specific modules (`src/ui.rs`, `src/history.rs`, `src/app.rs`) to replace the lower pane while keeping tests under `/workspaces/Stando/tests/` for any new assertions.
