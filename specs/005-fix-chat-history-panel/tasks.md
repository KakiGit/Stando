---

description: "Task list for AI Mode Chat History Panel — Chat Grouping"
---

# Tasks: AI Mode Chat History Panel — Chat Grouping

**Input**: Design documents from `/specs/005-fix-chat-history-panel/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Tests**: Required (plan.md calls for targeted unit tests for grouping logic and history panel state)

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish shared data structures and constants used by all stories

- [x] T001 Add ChatGroup/ConversationRound/ChatSummary structs and RECENT_CHAT_ID constant in src/history.rs
- [x] T002 Extend ChatHistoryRecord with optional chat_hash field + serde defaults in src/history.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core history grouping and selection plumbing required before any user story work

- [x] T003 Implement grouping helper to aggregate ConversationRound into ChatGroup + metadata in src/history.rs
- [x] T004 Expose grouped history load API returning Vec<ChatGroup> + default selected_chat_id in src/history.rs
- [x] T005 Update HistoryPanelState to track selected_chat_id and chat-level visible_range in src/history_panel.rs
- [x] T006 Preserve selected chat hash across history refreshes in src/history_panel.rs

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Inspect chat spans when reviewing past work (Priority: P1) 🎯 MVP

**Goal**: Show one list entry per chat hash with metadata, and render the full chronological timeline when selected.

**Independent Test**: Populate with multiple multi-round chats; list shows each chat once and detail pane only updates on new chat selection.

### Tests for User Story 1

- [x] T007 [P] [US1] Add unit tests for grouping order + metadata derivation in src/history.rs
- [x] T008 [P] [US1] Add unit tests for timeline ordering/formatting in src/history_panel.rs

### Implementation for User Story 1

- [x] T009 [US1] Build ChatSummary list entries (hash, last actor, last timestamp, round count) in src/history_panel.rs
- [x] T010 [US1] Render chat summary metadata in list rows for the history sidebar in src/history_panel.rs
- [x] T011 [US1] Render full chat timeline (earliest→latest, role/timestamp labels) in detail pane in src/history_panel.rs

**Checkpoint**: User Story 1 is functional and testable independently

---

## Phase 4: User Story 2 - Reopen threads via hash-aligned list entries (Priority: P2)

**Goal**: Selecting a chat entry reopens its hash-aligned timeline and preserves selection during sync.

**Independent Test**: Create two chats with distinct hashes; selecting each swaps the detail pane to its full timeline without mixing rounds.

### Tests for User Story 2

- [x] T012 [P] [US2] Add unit test for selection preservation across refresh in src/history_panel.rs

### Implementation for User Story 2

- [x] T013 [US2] Update selection change handling to reload timeline by chat hash in src/history_panel.rs
- [x] T014 [US2] Add helper to fetch a chat timeline by hash in src/history.rs

**Checkpoint**: User Story 2 is functional and testable independently

---

## Phase 5: User Story 3 - Surface grouping expectations when history is empty or minimal (Priority: P3)

**Goal**: Provide clear empty/single-chat messaging so users understand grouped chats and ordering.

**Independent Test**: Clear history, open panel and verify empty-state messaging; add one chat and confirm it appears with ordered rounds.

### Implementation for User Story 3

- [x] T015 [US3] Update empty/single-chat placeholder messaging to describe grouped chats in src/history_panel.rs

**Checkpoint**: User Story 3 is functional and testable independently

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Quality, diagnostics, and validation across all stories

- [x] T016 [P] Add/adjust tracing around grouping + selection refresh in src/history.rs
- [x] T017 [P] Run quickstart validation steps and update docs if needed in specs/005-fix-chat-history-panel/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - blocks all user stories
- **User Stories (Phase 3+)**: Depend on Foundational completion
- **Polish (Phase 6)**: Depends on desired user stories being complete

### User Story Dependencies

- **US1 (P1)**: No dependencies beyond Foundational
- **US2 (P2)**: No dependencies beyond Foundational
- **US3 (P3)**: No dependencies beyond Foundational

### Within Each User Story

- Tests (if included) must be written and fail before implementation
- Grouping/model utilities before UI rendering
- Selection logic before timeline rendering
- Story complete before moving to next priority

### Dependency Graph (Story Order)

Foundational → US1 → US2 → US3 → Polish

---

## Parallel Execution Examples

### User Story 1

```bash
# Run in parallel (different files):
Task: "Add unit tests for grouping order + metadata derivation in src/history.rs"
Task: "Add unit tests for timeline ordering/formatting in src/history_panel.rs"
```

### User Story 2

```bash
# Run in parallel (different files):
Task: "Add unit test for selection preservation across refresh in src/history_panel.rs"
Task: "Add helper to fetch a chat timeline by hash in src/history.rs"
```

### Cross-story parallelism (after Foundational)

```bash
# US1, US2, and US3 can proceed in parallel once Phase 2 completes
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. Validate User Story 1 independently using the quickstart scenarios

### Incremental Delivery

1. Setup + Foundational
2. US1 (MVP) → Validate
3. US2 → Validate
4. US3 → Validate
5. Polish & cross-cutting concerns
