---

description: "Task list template for feature implementation"
---

# Tasks: Add Debug Logging

**Input**: Design documents from `/specs/001-add-debug-logging/`  
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are optional for this feature; follow quickstart.md for manual verification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Ensure logging configuration and project context are ready

- [X] T001 Review existing logging initialization in src/main.rs and document current debug toggles in specs/001-add-debug-logging/quickstart.md

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Define shared logging conventions and privacy rules used across all stories

- [X] T002 Define function-level logging conventions (entry/exit/outcome, privacy rules) in specs/001-add-debug-logging/plan.md
- [X] T003 [P] Create a logging helper module for function entry/exit events in src/logging.rs
- [X] T004 Wire the logging helper module into src/main.rs (module registration, no behavior changes yet)

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Trace execution for a user action (Priority: P1) 🎯 MVP

**Goal**: Provide end-to-end function-level debug logs for a typical user action

**Independent Test**: Run with debug logging enabled and confirm logs show function entry/exit and error outcomes for a single user action

### Implementation for User Story 1

- [X] T005 [P] [US1] Add function entry/exit debug logs in src/app.rs
- [X] T006 [P] [US1] Add function entry/exit debug logs in src/ui.rs
- [X] T007 [P] [US1] Add function entry/exit debug logs in src/search.rs
- [X] T008 [P] [US1] Add function entry/exit debug logs in src/ai.rs (avoid logging prompts or secrets)
- [X] T009 [P] [US1] Add function entry/exit debug logs in src/hotkeys.rs
- [X] T010 [P] [US1] Add function entry/exit debug logs in src/tray.rs
- [X] T011 [P] [US1] Add function entry/exit debug logs in src/daemon.rs
- [X] T012 [P] [US1] Add function entry/exit debug logs in src/config.rs (avoid logging secrets)
- [X] T013 [US1] Add error-outcome logging for fallible functions across src/*.rs
- [ ] T014 [US1] Validate logging coverage for a single user action and capture notes in specs/001-add-debug-logging/quickstart.md

**Checkpoint**: User Story 1 is fully functional and testable independently

---

## Phase 4: User Story 2 - Control log verbosity (Priority: P2)

**Goal**: Ensure debug logs only appear when explicitly enabled

**Independent Test**: Run without debug logging enabled and confirm function-level debug entries are absent

### Implementation for User Story 2

- [X] T015 [US2] Verify debug logs are gated behind EnvFilter/--verbose behavior in src/main.rs
- [X] T016 [US2] Update logging helper to respect debug gating and avoid emitting debug entries when disabled in src/logging.rs
- [ ] T017 [US2] Confirm disabled logging behavior and update specs/001-add-debug-logging/quickstart.md

**Checkpoint**: User Story 2 is fully functional and testable independently

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Ensure documentation and privacy guarantees are clear

- [X] T018 [P] Update README.md with debug logging enablement details
- [X] T019 Review all new debug logs for sensitive data exposure and remove or redact as needed in src/*.rs
- [ ] T020 Run quickstart.md validation steps and note any gaps in specs/001-add-debug-logging/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: Depend on Foundational phase completion
- **Polish (Final Phase)**: Depends on completion of targeted user stories

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Independent of US1

### Parallel Opportunities

- US1 logging tasks across different modules can run in parallel (T005-T012)
- US2 verification and helper updates can run in parallel with other non-overlapping tasks once foundational work is done
- Documentation updates can run in parallel with implementation tasks when files do not overlap

---

## Parallel Example: User Story 1

```bash
Task: "Add function entry/exit debug logs in src/app.rs"
Task: "Add function entry/exit debug logs in src/ui.rs"
Task: "Add function entry/exit debug logs in src/search.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Run quickstart.md validation for US1

### Incremental Delivery

1. Complete Setup + Foundational
2. Add User Story 1 → Validate independently
3. Add User Story 2 → Validate independently
4. Polish and document
