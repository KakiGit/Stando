---
description: "Task list template for feature implementation"
---

# Tasks: AI History Panel

**Input**: Design documents from `/workspaces/Stando/specs/004-add-ai-history-panel/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare the shared state surface and module layout that every story will consume.

- [X] T001 Create `src/history_panel.rs` module with `ChatHistoryEntry` and `HistoryPanelState` struct skeletons (fields: id, timestamp, role, summary, content, entries, selected_entry_id, selected_visible_range, empty_state_message) so the panel state has a dedicated home.
- [X] T002 Initialize an `Arc<RwLock<HistoryPanelState>>` in `src/app.rs` alongside the existing AI mode state to share history entries and selection between services and the UI.
- [X] T003 Add `mod history_panel;` (and required `use` statements) to `src/main.rs` and pass the shared `HistoryPanelState` into `App::new`, ensuring the new module is reachable from the root application.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Implement the data and async plumbing that keeps the list/detail widgets in sync before any story-specific UI work.

- [X] T004 Populate `HistoryPanelState.entries` by converting persisted records from `src/history.rs::UsageHistory` into `ChatHistoryEntry`s ordered chronologically, including timestamps/role labels for the list.
- [X] T005 Implement helper methods in `src/history_panel.rs` that append new entries (user prompts or AI responses), update `selected_entry_id` to the newest entry, and maintain `selected_visible_range` for auto-scrolling.
- [X] T006 Have `src/ai.rs` notify the shared `HistoryPanelState` (via `glib::MainContext::default().spawn_local` callbacks) when responses or prompts are stored so the list and detail panes stay synchronized without blocking GTK.

---

## Phase 3: User Story 1 - Review chat history detail (Priority: P1) 🎯 MVP

**Goal**: Replace the lower AI panel with a read-only detail pane on the left and a chronological history list on the right so the currently selected entry is always visible.

**Independent Test**: Open AI mode with existing history, confirm the right-hand list renders every entry with timestamp/role labels, select different rows, and verify the left pane shows the selected entry’s full content.

### Implementation

- [X] T007 [US1] Build the AI history pane layout in `src/ui.rs` by replacing the bottom result section with a `gtk4::Paned` that hosts a read-only `TextView` on the left and a `ScrolledWindow` containing a `ListBox` on the right.
- [X] T008 [US1] Populate the `ListBox` rows in `src/ui.rs` from `HistoryPanelState.entries`, rendering timestamps/role summaries in chronological order and keeping each row tied to its `ChatHistoryEntry`.
- [X] T009 [US1] Bind the selected row to the left-hand `TextView` in `src/ui.rs`, updating the view with the selected entry’s `content` whenever `selected_entry_id` changes so the detail pane mirrors the highlighted history entry.
- [X] T010 [US1] Ensure selection highlighting and click handlers live in `src/ui.rs` so clicking a row or programmatically changing `selected_entry_id` visually distinguishes the active entry and refreshes the detail text immediately.

---

## Phase 4: User Story 2 - Navigate history with keyboard (Priority: P2)

**Goal**: Let keyboard-first users move through the history list with the arrow keys while focus stays inside the AI panel.

**Independent Test**: Focus the history list, use the Up/Down arrows, and confirm each keystroke updates the selected highlight and the left-hand detail text without touching the mouse.

### Implementation

- [X] T011 [US2] Attach an `EventControllerKey` to the history list widget in `src/ui.rs` and intercept Up/Down arrow keys to trigger selection moves while keeping focus inside the panel.
- [X] T012 [US2] Implement keyboard-driven selection helpers in `src/history_panel.rs` that adjust `selected_entry_id` and `selected_visible_range` when the user presses arrow keys so the UI scrolls and detail text updates within the 1 second budget.
- [X] T013 [US2] Update `src/app.rs` (AI mode activation path) to focus the history list and reconcile `HistoryPanelState.selected_entry_id` whenever the AI panel opens so arrow keys immediately start navigating the current entries.

---

## Phase 5: User Story 3 - Understand empty or single-entry states (Priority: P3)

**Goal**: Provide clear messaging when the AI history is empty so new users know where entries will appear.

**Independent Test**: Launch AI mode with no prior messages, check that the list shows an empty-state placeholder and the left pane explains how selections will appear once entries exist.

### Implementation

- [X] T014 [US3] Render an empty-state placeholder message inside the history list area in `src/ui.rs` when `HistoryPanelState.entries.is_empty()`, describing how entries will populate once a chat starts.
- [X] T015 [US3] Swap the left-hand detail `TextView` content in `src/ui.rs` to display instructional text when there is no selection, and revert to entry content when `selected_entry_id` becomes `Some`.
- [X] T016 [US3] Surface the empty state message stored in `HistoryPanelState.empty_state_message` (in `src/history_panel.rs`) so both the list placeholder and detail pane can read the same copy of the text.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Tie up documentation and observability that span all stories.

- [X] T017 [P] Describe the new AI history layout, keyboard navigation, and empty-state behaviors in `specs/004-add-ai-history-panel/quickstart.md` so testers can validate the feature end-to-end.
- [X] T018 [P] Add tracing spans or logs in `src/history_panel.rs` (or `src/logging.rs`) around history loads, selection changes, and keyboard navigation so success criteria SC-001..SC-003 can be observed in diagnostics.

---

## Dependencies & Execution Order

- **Setup (Phase 1)** → **Foundational (Phase 2)** → User Story phases (Phase 3 US1 → Phase 4 US2 → Phase 5 US3) → **Polish (Phase 6)**
- **User Story 1 → User Story 2 → User Story 3** (US1 provides the shared layout that keyboard handling and empty states augment)

## Parallel Execution Examples

- **User Story 1**: T007 (layout), T008 (list population), and T009 (detail binding) can start together once the panel scaffolding exists because they touch different widgets in `src/ui.rs`.
- **User Story 2**: T011 (key controller) and T012 (selection helper) can be implemented in parallel before wiring focus in T013.
- **User Story 3**: T014 (list placeholder) and T015 (detail placeholder) can be implemented simultaneously while T016 ensures the state message is shared.

## Implementation Strategy

### MVP First (User Story 1 Only)
1. Finish Setup + Foundational phases so the shared `HistoryPanelState` is ready.
2. Deliver Phase 3 (US1) so the basic layout and list/detail binding work.
3. Validate US1 manually (see independent test in Phase 3).
4. Consider the MVP done and stop or ship if further stories are not approved.

### Incremental Delivery
1. After MVP, add keyboard navigation (Phase 4) and test arrow key selection independently.
2. Then add the empty-state messaging (Phase 5) and verify new-user guidance.
3. Finally, polish docs/logging (Phase 6) before announcing completion.

### Parallel Team Strategy
- Phase 1+2 are single-threaded prep.
- Once foundational state exists, US1, US2, and US3 can be worked on in parallel by different developers, followed by the polish phase.
