---
description: "Task list template for feature implementation"
---

# Tasks: Force Window Top

**Input**: Design documents from `/specs/003-force-window-top/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/floating-mode.yaml, quickstart.md

**Tests**: None of the user stories request automated coverage; the quickstart steps document the independent validation criteria for each story.

**Organization**: Tasks are grouped by user story so that each priority increment can be implemented and tested independently while respecting shared infrastructure.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the scaffolding for the floating-window feature—new module surfaces, config hooks, and module wiring—so later phases can focus on concrete logic.

- [x] T001 [P] Create `src/window.rs` with the `FloatingWindowController` and state placeholder definitions so that floating-window logic lives in a dedicated module.
- [x] T002 Update `src/main.rs` to `mod window;`, instantiate the controller after building the GTK application, and pass the `gtk::Application` handle for future stacking control.
- [x] T003 [P] Declare `FloatingPreference` fields in `src/config.rs` (`enabled`, `preferred_workspace`, `preferred_display`) so the configuration layer exposes the persisted preference shape defined in the data model.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Implement the shared persistence, control, and contract plumbing that every story depends on before specific behaviors can be delivered.

- [x] T004 [P] Implement `FloatingPreference` persistence in `src/config.rs`, wiring the config loader and saver to read/write `enabled`, optional workspace/display metadata, and expose setters that the controller will call when the toggle changes.
- [ ] T005 [P] In `src/window.rs`, fully flesh out `StandoWindowState` (`is_floating`, `workspace_id`, `display_id`, `last_override_success`), add throttle-aware `reassert_floating` helpers, and expose methods that apply GTK stacking hints according to the plan’s constraints.
- [ ] T006 Add GTK/GLib signal hooks in `src/app.rs` to notify `FloatingWindowController` about workspace switches, focus changes, and monitor drags so the controller can respond immediately when these events occur.
- [ ] T007 Implement the floating-control contract in `src/window/floating_control.rs`: expose `/window/floating` GET/POST handlers per `contracts/floating-mode.yaml`, map GET responses to `StandoWindowState`, allow POST to toggle floating preference, and return the `fallbackReason`/`effectiveState` payload the contract requires.

---

## Phase 3: User Story 1 - Always-visible monitor (Priority: P1) 🎯 MVP

**Goal**: Keep Stando’s main window persistently on top of tiled clients inside i3/sway/hyprland so it never becomes hidden when other windows open or focus shifts.

**Independent Test**: Enable floating mode, open/focus multiple tiled windows, switch workspaces, and confirm the Stando window remains above every other client without manual repositioning.

### Tests for User Story 1
_No automated tests requested; rely on the quickstart validation steps inside `specs/003-force-window-top/quickstart.md`._

### Implementation for User Story 1

- [ ] T008 [US1] Add the floating-mode indicator/toggle widget to `src/ui.rs`, ensuring the icon reflects the state, exposes the control to keyboard navigation, and emits events when the user tries to change floating mode.
- [ ] T009 [US1] Wire the new indicator in `src/app.rs` so that clicks or hotkey activations invoke `FloatingWindowController::set_floating(true/false)` and log attempts for later troubleshooting.
- [ ] T010 [US1] Enhance `src/window.rs` to reassert that the window stays on top whenever `workspace_id` or focus changes are reported, ensuring repeated requests honor the 16ms frame budget described in the plan.

### Parallel Example: User Story 1

```bash
# Launch UI work while reporting logic evolves:
# - T008: Touches ui.rs to add the visible toggle.
# - T010: Touches window.rs to solidify reassertion helpers.
```

---

## Phase 4: User Story 2 - Multi-display focus (Priority: P2)

**Goal**: Track whichever monitor currently hosts Stando and keep it floating on that display even while the user drags other windows or shifts focus elsewhere.

**Independent Test**: Toggle floating mode on each connected monitor, move the window across displays, and verify it stays topmost on the currently occupied screen regardless of other focus shifts.

### Tests for User Story 2
_No automated tests requested; reuse the workspace/display exercises from `quickstart.md`._

### Implementation for User Story 2

- [ ] T011 [US2] Extend `src/window.rs` so `FloatingWindowController` records `display_id` updates when the window moves, re-applies stacking hints on the new monitor, and surfaces the display metadata to the UI.
- [ ] T012 [P] [US2] Update `src/ui.rs` to show the name or index of the current display next to the floating indicator, using `FloatingWindowController` callbacks so users know which screen is pinned.
- [ ] T013 [US2] Hook `src/app.rs` into monitor/display change signals (e.g., `gdk::Display::connect_monitors-changed`) and tell the controller to keep the window on top of the target monitor whenever the display layout changes.

### Parallel Example: User Story 2

```bash
# Display tracking tasks that touch different files can run together:
# - T011 (window.rs updates) can occur while T012 (ui.rs message) executes, as long as T013 wiring reads from the same controller interface.
```

---

## Phase 5: User Story 3 - Graceful degradation (Priority: P3)

**Goal**: Detect when stacking overrides are rejected, inform the user, and keep the window functional by staying in regular stacking until the environment changes.

**Independent Test**: Enable floating mode in a workspace/compositor that rejects overrides and confirm the UI shows the explanation while the window remains reachable.

### Tests for User Story 3
_Rely on the manual fallback validation described in `quickstart.md` to confirm messaging and fallback behavior._

### Implementation for User Story 3

- [ ] T014 [US3] Add a fallback notification area in `src/ui.rs` (e.g., near the floating indicator) that surfaces `fallbackReason` text whenever the controller reports override failures.
- [ ] T015 [US3] In `src/window.rs`, record `last_override_success`, throttle retries if the WM repeatedly rejects stacking overrides, and populate the contract’s `fallbackReason`/`overrideState` fields so the control API can explain what happened.
- [ ] T016 [P] [US3] Extend `src/logging.rs` (or the central tracing setup) to record override success/failure events so QA can correlate user reports with controller behavior.

### Parallel Example: User Story 3

```bash
# Messaging and logging can be implemented in tandem:
# - T014 (ui feedback) uses the controller’s failure indicators being added in T015 while T016 records those events for diagnostics.
```

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Round out the feature with documentation, logging, and validation that span multiple stories.

- [ ] T017 [P] Update `specs/003-force-window-top/quickstart.md` so each user story has a clear independent verification step and notes when floating mode should persist or degrade.
- [ ] T018 [P] Add tracing/logging hooks in `src/logging.rs` to capture floating-mode requests, successes, and failure contexts referenced by both controller logic and UI messages.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Starts immediately and yields the necessary modules/config shapes.
- **Foundational (Phase 2)**: Depends on Phase 1; provides persistence, controller, and API plumbing that all stories require.
- **User Stories (Phase 3+)**: Begin once the foundation is ready; stories are independent and can proceed in priority order or in parallel.
- **Polish (Final Phase)**: Runs after at least one story is complete to document and log the new behavior.

### User Story Dependencies

- **User Story 1 (P1)**: Depends on Phase 2; no dependence on US2/US3.
- **User Story 2 (P2)**: Depends on Phase 2 and optionally reads data surfaces from US1; can be implemented concurrently.
- **User Story 3 (P3)**: Depends on Phase 2 and the controller hooks in US1/US2; does not block earlier stories.

### Within Each User Story

- Tests: Manual validation per quickstart steps.
- UI indicator before controller wiring ensures visibility.
- Controller reassertion helpers before display/fallback behaviors.

### Parallel Execution Notes

- `[P]` tasks can run together because they touch different files with no hidden dependencies.
- Foundation tasks `T004`/`T005` may be executed side-by-side while `T006` and `T007` wait for their outputs.
- Once the foundation is ready, US1/US2/US3 teams can work on their respective files simultaneously.

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Finish Phase 1 & 2 so the controller, persistence, and contract endpoints exist.
2. Implement User Story 1 tasks (T008–T010) and validate using the independent test.
3. Pause to demo or ship if the always-visible window meets the spec.

### Incremental Delivery

1. Deliver Phase 1 + 2 for baseline infrastructure.
2. Add US1 (keep window on top) → test → release.
3. Add US2 (multi-display awareness) → test → release.
4. Add US3 (graceful degradation) → test → release.
5. Polish with documentation/logging after all stories deliver value.

### Parallel Team Strategy

1. Team finishes Setup and Foundation together.
2. Once the controller exists:
   - Developer A: US1 (T008–T010)
   - Developer B: US2 (T011–T013)
   - Developer C: US3 (T014–T016)
3. Polish tasks (T017–T018) can run while stories are validating.

### Verification

1. Run manual quickstart scenarios after each story.
2. Ensure `floating_control` responses match the contract payloads.
3. Confirm logs capture override attempts and fallback reasons.
