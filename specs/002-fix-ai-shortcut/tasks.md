---

description: "Task list for window-local AI mode shortcut"
---

# Tasks: Window-Local AI Mode Shortcut

**Input**: Design documents from `/specs/002-fix-ai-shortcut/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/, quickstart.md

**Tests**: No automated tests requested; rely on manual verification in `/workspaces/Stando/specs/002-fix-ai-shortcut/quickstart.md`.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 Review existing AI shortcut wiring in `src/hotkeys.rs` and `src/app.rs` to identify global toggle removal points

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T002 Refactor AI mode toggle logic into a reusable helper in `src/app.rs` (used by button and window shortcut)

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Switch AI mode in focused window (Priority: P1) 🎯 MVP

**Goal**: Window-focused shortcut switches AI mode using the existing in-app behavior

**Independent Test**: Focus the Stando window, press the shortcut, and confirm the mode changes as it does via the UI button without inserting text

### Implementation for User Story 1

- [ ] T003 [US1] Add a window-scoped `gio::SimpleAction` (e.g., `win.toggle-ai-mode`) in `src/app.rs` that calls the shared toggle helper
- [ ] T004 [US1] Register a `gtk4::ShortcutController` on the window in `src/app.rs` using the configured AI shortcut and map it to `win.toggle-ai-mode`
- [ ] T005 [US1] Update the AI button tooltip to reflect the configured shortcut in `src/ui.rs`

**Checkpoint**: User Story 1 is fully functional and testable independently

---

## Phase 4: User Story 2 - Shortcut does not act globally (Priority: P2)

**Goal**: The AI mode shortcut no longer triggers when the app is unfocused

**Independent Test**: Switch focus to another app and press the shortcut; Stando does not change mode or steal focus

### Implementation for User Story 2

- [ ] T006 [US2] Remove global AI toggle hotkey registration and event mapping from `src/hotkeys.rs`
- [ ] T007 [US2] Remove `HotKeyEvent::ToggleAI` handling from `src/app.rs` hotkey polling loop
- [ ] T008 [US2] Keep show/hide hotkey behavior intact and update any related documentation strings in `src/config.rs`

**Checkpoint**: User Stories 1 and 2 both work independently

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T009 [P] Update user-facing documentation in `README.md` to clarify the AI shortcut is window-local and uses the configured key
- [ ] T010 Run the manual validation in `/workspaces/Stando/specs/002-fix-ai-shortcut/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: Depend on Foundational phase completion
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - no dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - removes global behavior and validates focus scoping

### Within Each User Story

- Shared toggle helper before window shortcuts
- Window shortcut before removing global toggle to keep functional coverage
- Story complete before moving to polish

---

## Parallel Example: User Story 1

No safe parallel tasks within this story; handle sequentially to keep shortcut wiring consistent.

---

## Parallel Example: User Story 2

No safe parallel tasks within this story; global hotkey removal touches shared types and should be sequential.

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Run manual verification in `/workspaces/Stando/specs/002-fix-ai-shortcut/quickstart.md`

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Validate manually
3. Add User Story 2 → Validate manually
4. Finish Polish tasks
