# Feature Specification: AI Mode Chat History Panel — Chat Grouping

**Feature Branch**: `005-fix-chat-history-panel`  
**Created**: 2026-01-29  
**Status**: Draft  
**Input**: User description: "please fix 004-add-ai-history-panel **This is for the AI mode** 1. Each item in the list should contain one chat. Each chat can have several rounds in the conversation. One or several conversations form a chat. Conversations can have hash-id which can be used to construct the chat history. 2. The content box should show all conversations in a chat from the earliest to the latest."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Inspect chat spans when reviewing past work (Priority: P1)

As an AI mode user, I need each row in the history list to represent a whole chat so I can see where each grouping of conversations starts and ends before picking one to re-open.

**Why this priority**: Correct chat grouping ensures I do not mistake a single conversation round for an entire discussion, which could lead to sending redundant prompts or losing context.

**Independent Test**: Populate the panel with multiple chats, each made of multiple conversation rounds, then verify that the list shows each chat only once (grouped by chat hash) and that the detail pane only updates when a new chat row is selected.

**Acceptance Scenarios**:

1. **Given** there are several chats in history, **When** the panel renders, **Then** the list contains one entry per chat hash and shows a summary (e.g., last speaker + timestamp) for that chat.
2. **Given** a chat has multiple rounds, **When** I select the chat’s entry, **Then** the content box shows the entire sequence of rounds in chronological order from the earliest submission to the latest response.

---

### User Story 2 - Reopen threads via hash-aligned list entries (Priority: P2)

As a user relying on hashed conversation IDs, I want to re-open a chat by selecting its entry so I can resume a thread that spans several conversation rounds.

**Why this priority**: Users may use AI sessions for multi-step investigations; grouping by hash allows continuing where they left off without reconstructing context manually.

**Independent Test**: Create two chats that share no hash and verify that selecting the first chat’s entry updates the detail pane with its concatenated rounds and selecting the second chat replaces the content entirely with the second group.

**Acceptance Scenarios**:

1. **Given** history contains chats distinguished by hash IDs, **When** I select a chat entry, **Then** the detail pane’s text box lists every conversation round belonging to that hash, each round labeled with role (user or assistant) and timestamp.

---

### User Story 3 - Surface grouping expectations when history is empty or minimal (Priority: P3)

As a first-time AI mode visitor, I want to see clear messaging when there are zero or only one chat so I know the grouping logic and timeline are working even before I create multiple chats.

**Why this priority**: Prevents confusion in the new layout by explaining that chats and their rounds will appear here once generated.

**Independent Test**: Launch AI mode after clearing history, confirm placeholder messaging about chat groups appears, then send a single prompt/response pair and verify the list shows one chat entry and the content box orders the round earliest first.

**Acceptance Scenarios**:

1. **Given** no chats exist, **When** the panel opens, **Then** the list shows an empty-state message describing that each entry becomes a chat and explains that the detail pane will show the selected chat.
2. **Given** only one chat exists, **When** the view renders, **Then** the list shows exactly that chat, and the content box shows the rounds in time order.

---

### Edge Cases

- What happens when the chat hash is unavailable (null or missing) for some conversations—do we break them into a default chat or mark them as orphaned?
- How does the system behave when a single chat contains dozens of rounds that exceed the size of the detail pane?
- How should the history panel react if new conversation rounds arrive while another chat entry remains selected?
- How will keyboard navigation (up/down) be limited to chats instead of individual rounds once the grouping changes?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The history list MUST treat each entry as a single chat that groups all conversations sharing the same hash identifier, even when those conversations span multiple rounds.
- **FR-002**: Each chat entry in the list MUST display metadata (e.g., chat hash, last speaker, most recent timestamp) so I can differentiate chats before I open them.
- **FR-003**: Selecting a chat entry MUST populate the content box with every round from that chat, ordered chronologically from earliest to latest and visibly segmented by role (user vs assistant).
- **FR-004**: The content box MUST render a chat’s rounds in a scrollable timeline so users can read the full sequence without losing context, even when the chat contains many rounds.
- **FR-005**: When conversation rounds arrive or the chat hash stream re-syncs, the list MUST refresh its chat-level entries while preserving the currently selected chat (if still present) rather than switching focus mid-stream.
- **FR-006**: The history panel MUST gracefully handle conversations with missing or non-matching hash IDs by grouping them into a “Recent Chat” fallback entry so no rounds are orphaned.

### Key Entities *(include if feature involves data)*

- **Chat**: A collection of conversation rounds united by a hash identifier; tracks metadata such as hash ID, summary, and latest timestamp.
- **Conversation Round**: A single message exchange (user prompt or AI response) belonging to a chat; has fields for role, timestamp, content, and ordering position.
- **History Panel State**: Represents the selected chat, the list of available chat entries, and the ordering rules used to determine which chat is shown in the detail view.

## Assumptions

- The existing chat history storage already exposes hash IDs or deterministic keys that group related conversation rounds; if not available, the “Recent Chat” fallback is acceptable.
- The content box is read-only and only used to present previously exchanged text; no editing or message-sending occurs from this view.
- Keyboard navigation (up/down) continues to move between chat-level entries once grouping is enforced.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 95% of desktop sessions with history show a list where each entry corresponds to a unique chat hash and selecting that entry populates the content box with that chat’s rounds.
- **SC-002**: 95% of selections on the history list update the content box within 1 second with the chat’s rounds in earliest-to-latest order.
- **SC-003**: At least 90% of users can distinguish between two similarly timed chats by relying on the chat metadata shown in the list (hash, last actor, timestamp) during usability testing.
- **SC-004**: The panel never leaves conversation rounds orphaned—meaning after loading history, at least 99% of rounds are displayed inside some chat entry (including the fallback group) during a sync test.
