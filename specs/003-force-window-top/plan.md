# Implementation Plan: Force Window Top

**Branch**: `003-force-window-top` | **Date**: 2026-01-25 | **Spec**: specs/003-force-window-top/spec.md
**Input**: Feature specification from `/specs/003-force-window-top/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Keep Stando’s main window visible above every other client—including tiled applications in i3, sway, and hyprland—by adding a persistent floating toggle that reasserts top stacking across workspace switches, focus shifts, and multi-monitor moves while gracefully handling window manager refusals.

## Technical Context

**Language/Version**: Rust 2021  
**Primary Dependencies**: GTK4/libadwaita, glib/gio, tracing, tracing-subscriber, global-hotkey, tokio (existing stack)  
**Storage**: Local configuration (e.g., `~/.config/stando/config.toml`) for remembering floating preferences  
**Testing**: `cargo test`, `cargo clippy`  
**Target Platform**: Linux desktop distributions, especially tiling window managers like i3, sway, and hyprland  
**Project Type**: Single desktop GTK4 application  
**Performance Goals**: Ensure floating-state updates complete within a single UI frame (<16ms) so the window never visibly blinks behind other clients when focus or workspace changes occur  
**Constraints**: Must respect compositor/WM restrictions on stacking overrides (especially on Wayland) and provide user-visible fallbacks when the override is rejected  
**Scale/Scope**: One persistent Stando window that can live across multiple monitors/workspaces for a single user session

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Keyboard-first UX: core flows are keyboard-accessible; async work keeps UI responsive.  
- Local-first/privacy: no network calls outside explicit AI mode; key handling defined.  
- Daemon reliability: single-instance, hotkey behavior, clean start/stop addressed.  
- Resource efficiency: idle behavior, indexing bounds, and perf budget described.  
- Linux integration: GTK4/libadwaita usage and desktop artifacts covered.

## Project Structure

### Documentation (this feature)

```text
specs/003-force-window-top/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── main.rs
├── app.rs
├── ui.rs
├── ai.rs
├── search.rs
├── config.rs
├── history.rs
├── logging.rs
└── window.rs (new module to own stacking control and floating-state helpers)
```

**Structure Decision**: Keep the existing single Rust GTK project rooted at `src/`, adding any new floating-window coordination logic to a dedicated `window.rs` module so the rest of the application stays focused on search and UI state.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
|  |  |  |
