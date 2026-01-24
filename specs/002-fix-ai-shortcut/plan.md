# Implementation Plan: Window-Local AI Mode Shortcut

**Branch**: `002-fix-ai-shortcut` | **Date**: 2026-01-24 | **Spec**: /workspaces/Stando/specs/002-fix-ai-shortcut/spec.md  
**Input**: Feature specification from `/specs/002-fix-ai-shortcut/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Make the AI mode shortcut window-local so it only triggers in the focused Stando window, using GTK/GLib shortcut infrastructure and existing mode-switch behavior.

## Technical Context

**Language/Version**: Rust 2021  
**Primary Dependencies**: gtk4, libadwaita, glib, gio  
**Storage**: N/A  
**Testing**: cargo test; manual shortcut verification per spec  
**Target Platform**: Linux desktop (GTK4/libadwaita)  
**Project Type**: single project  
**Performance Goals**: Shortcut action is perceived instantaneous (<100 ms)  
**Constraints**: Window-local shortcut only; no global registration; must not insert characters into inputs  
**Scale/Scope**: Single app with multiple windows; per-window AI mode state

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Keyboard-first UX: Shortcut remains keyboard-accessible for focused window; no new blocking work on UI thread.
- Local-first/privacy: No changes to AI enablement; no new network calls; key handling stays local.
- Daemon reliability: Global hotkey behavior unchanged; window-local shortcut scoped to focused window.
- Resource efficiency: No polling; shortcut handling is event-driven.
- Linux integration: Uses GTK4/libadwaita shortcut facilities and window-local actions.

**Post-Design Re-check**: Pass; design keeps GTK actions and window-local shortcut controllers with no new background work.

## Project Structure

### Documentation (this feature)

```text
specs/002-fix-ai-shortcut/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
└── tasks.md
```

### Source Code (repository root)

```text
src/
tests/
```

**Structure Decision**: Single Rust application using the existing `src/` and `tests/` layout.

## Complexity Tracking

No constitution violations requiring justification.
