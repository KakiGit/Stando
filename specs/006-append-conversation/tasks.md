---
description: "Task list for AI Conversation Append implementation"
---

# Tasks: AI Conversation Append

**Input**: Design documents from `/specs/006-append-conversation/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Manual verification steps live in `quickstart.md`. Automated tests are optional unless explicitly requested by the specification; rely on targeted unit tests per story when clarity or regression risk warrants.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Align the manual verification notes with the refined acceptance criteria so QA can run the right scenarios before and after coding.

- [ ] T001 [P] Expand `specs/006-append-conversation/quickstart.md` with explicit steps covering the multi-sentence append scenario, persistent storage verification for `~/.stando/chat-history.json`, the "No chat selected" error path, whitespace-only message filtering, and the empty-history chat creation path so manual tests match the spec’s stories.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Add shared helpers that let story-specific logic reuse the same trimmed text and currently selected chat identifier without holding locks longer than necessary.

- [ ] T002 Implement `App::selected_history_chat_id` in `src/app.rs` to clone the currently selected chat ID from `HistoryPanelState` while the lock is held, but also to return a freshly minted chat ID when the history is empty so downstream AI code can append to that new conversation; cover both behaviors with async unit tests.
- [ ] T003 Implement `App::normalized_ai_query` in `src/app.rs` (returning `Some(trimmed_string)` or `None` when the input is empty/whitespace) and wire the current AI entry activation path to read through this helper so every story sees consistent, trimmed text.

---

## Phase 3: User Story 1 - Append new input in AI mode (Priority: P1) 🎯 MVP

**Goal**: Tie the AI submission pipeline to the currently selected conversation so the new entry appears at the tail of that chat and data-model invariants stay intact.

**Independent Test**: Send "Hello" while AI mode is active, verify the selected chat immediately shows "Hello" as the newest round, and confirm a multi-sentence message is stored and displayed as a single block at the bottom of the same chat.

- [ ] T004 [US1] Update the AI entry activation branch in `src/app.rs` to call `selected_history_chat_id`, skip generating a new chat hash, and create the user `ChatHistoryRecord` with that selected ID so the message is appended to the intended `ChatGroup`.
- [ ] T005 [US1] Ensure all assistant responses in `src/app.rs` (success, API error, and no-AI-service fallback) reuse the same chat identifier and refresh the history panel after pushing the response so every reply lives in the selected conversation.
- [ ] T006 [US1] Add a unit test in `src/history_panel.rs` that seeds a state with a known chat, calls `append_record` with another `ChatHistoryRecord` for that chat, and asserts the `ChatGroup` still orders rounds chronologically, proving the `Chat` entity’s `messages` vector keeps its sequence.

---

## Phase 4: User Story 2 - Error handling when no chat selected (Priority: P2)

**Goal**: Prevent AI submissions when no chat is selected and surface a UI error that mirrors the `/ai/chats/{chatId}/messages` contract's 400 response.

**Independent Test**: (a) Enable AI mode with no chat selected while chats exist, send text, and confirm a "No chat selected" notification appears with no appended message. (b) Start the app with zero chats persisted, enter AI mode, send text, and confirm a new chat appears with the message while no error is shown.

- [ ] T007 [US2] Add `SearchWindow::show_ai_error_notification` in `src/ui.rs` (e.g., via a `gtk4::MessageDialog`) so the UI can notify users when a submission would violate the selected-chat precondition.
- [ ] T008 [US2] Before submitting the trimmed query in `src/app.rs`, guard on `selected_history_chat_id`; if it returns `None`, call the new error notification, log the contract failure (matching `contracts/ai-append.yaml` 400 case), and return without recording history or calling the AI service.

---

## Phase 5: User Story 3 - Handling empty or whitespace input (Priority: P3)

**Goal**: Ignore empty or whitespace-only submissions so the `Message` entity’s `content` remains non-empty and conversations stay uncluttered.

**Independent Test**: Send a message composed only of spaces while AI mode is active and confirm the chat stays unchanged and the history JSON does not get a new entry.

- [ ] T009 [US3] Use `normalized_ai_query` inside `src/app.rs` so whitespace-only inputs short-circuit before any user or assistant `ChatHistoryRecord` is created, satisfying FR-006 and keeping `Message.content` non-blank.
- [ ] T010 [US3] Add unit tests to `src/app.rs` that exercise `normalized_ai_query`, confirming whitespace-only strings return `None` while meaningful text still returns the trimmed value, covering the new validation requirement.

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Capture diagnostics and rationale that span multiple stories after the core work is done.

- [ ] T011 Add targeted `tracing::info!` instrumentation in `src/app.rs` immediately after a message is appended so the logs record which chat ID gained a new round and the trimmed preview for debugging.
- [ ] T012 [P] Update `specs/006-append-conversation/research.md` with the final decisions that keeping the AI response tied to the selected chat and trimming invalid input are intentional safeguards, documenting the rationale for future contributors.

---

## Dependencies & Execution Order

### Phase Dependencies
- **Setup (Phase 1)**: Nothing blocks this phase, so it can happen immediately and in parallel with any documentation or review work.
- **Foundational (Phase 2)**: Requires Phase 1, and no user story should begin until the helper methods are in place to avoid copy-pasted locking logic.
- **User Stories (Phases 3-5)**: All depend on the helpers from Phase 2. Within this block, stories are independent, but for MVP delivery complete US1 first, followed by US2 and US3. 
- **Polish (Final Phase)**: Depends on all desired stories finishing so diagnostics and research notes reflect the implemented behavior.

### User Story Dependencies
- **User Story 1 (P1)**: Depends only on the foundational helpers; can be completed without US2/US3.
- **User Story 2 (P2)**: Depends on Phase 2 helpers and the UI error surface; it can run alongside US3 after the foundation is ready.
- **User Story 3 (P3)**: Depends on Phase 2 helpers to trim input; independent of the other stories after that.

### Within Each User Story
- Tests (T006, T010, etc.) should be written alongside the logic so they can fail before implementation.
- Models/services (if any) precede UI updates; here the helper methods and history panel updates feed the UI logic.
- Each story should pass its independent acceptance test before moving to the next priority.

---

## Parallel Execution Examples

- **User Story 1**: While T004/T005 work through `src/app.rs`, T006 can be built in parallel because it only touches `src/history_panel.rs`, letting one engineer handle UI plumbing and another cover the history invariant test.
- **User Story 2**: T007 (`src/ui.rs`) is independent of T008 (`src/app.rs`), so the error dialog can be implemented while the AI submission guard is wired in the app logic.
- **User Story 3**: The helper-focused T009 can happen alongside T010’s unit tests since they operate in the same module but validate different concerns.

---

## Implementation Strategy

### MVP First (User Story 1 Only)
1. Complete Phase 1 (docs) and Phase 2 helpers so the runtime can read the selected chat and normalized query.
2. Implement User Story 1 (T004-T006) to append messages to the selected conversation and prove the history order invariant.
3. Validate with the independent test described for US1 before stopping.
4. Ship or demo if the MVP criteria are met.

### Incremental Delivery
1. Add User Story 2 (T007-T008); test the error notification and contract alignment independently and deploy.
2. Add User Story 3 (T009-T010); verify whitespace filtering, then deploy.
3. Each story should deliver value without invalidating prior stories or requiring new dependencies.

### Parallel Team Strategy
1. Team finishes Phase 1 and Phase 2 together to ensure the helper surface is ready.
2. Developer A works on US1 tasks and related tests (T004-T006).
3. Developer B implements the error notification (T007-T008) and validates the contract requirement.
4. Developer C owns the whitespace-trimming helper and helper tests (T009-T010).
5. After stories finish, any engineer can handle the polish tasks (T011-T012).
