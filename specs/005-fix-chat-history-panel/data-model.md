# Data Model — AI Mode Chat Grouping

## Entities

### ChatGroup
- **id**: `String` — the chat hash (normalized). For rounds lacking a hash, this becomes the literal `"recent-chat"` fallback value.
- **metadata**: struct containing `last_speaker: ChatRole`, `last_timestamp: DateTime<Utc>`, `summary: String` (e.g., last assistant response or last round preview).
- **rounds**: `Vec<ConversationRound>` sorted by `timestamp` ascending.
- **selected**: `bool` — derived flag to indicate whether this chat is the currently focused entry in `HistoryPanelState`.

### ConversationRound
- **round_id**: `Uuid` (matches existing `ChatHistoryEntry.id`).
- **role**: `ChatRole` (enum: `User`, `Assistant`).
- **timestamp**: `DateTime<Utc>`.
- **content**: `String` (full round text shown in timeline).
- **source**: `Option<String>` (carries over from existing records for filtering).

### HistoryPanelState
- **chats**: `Vec<ChatGroup>` (reverse chronological by `metadata.last_timestamp`).
- **selected_chat_id**: `Option<String>` (chat hash of the active entry).
- **empty_state_message**: `String` (unchanged constant text when no chats exist).
- **visible_range**: `RangeInclusive<usize>` (for keyboard navigation, derived from the selected chat index).

### ChatSummary (contract-facing payload)
- **chat_hash**: `String` (hash or fallback key).
- **display_label**: `String` (e.g., `Assistant · 2026-01-29 14:07`).
- **last_actor**: `ChatRole`.
- **last_timestamp**: `DateTime<Utc>`.
- **round_count**: `usize`.

## Relationships
- A `ChatGroup` is a one-to-many aggregation of `ConversationRound`s sharing the same `chat_hash` value.
- `HistoryPanelState` is responsible for managing the ordered list of `ChatGroup`s and which chat is currently selected.
- `ChatSummary` is derived from each `ChatGroup` for display and API purposes.

## Validation Rules
- `chat_hash` must be non-empty; if the incoming hash is `None`, convert it to `"recent-chat"` before grouping.
- Each `ConversationRound` must provide a `timestamp` for chronological ordering; missing timestamps cause the round to be excluded from ordering updates (logged for diagnostics).
- Keyboard navigation only operates on `ChatGroup`s — row indexes and `visible_range` calculations refer to chats, not rounds.

## State Transitions
1. **Load history**: `UsageHistory` returns persisted `ChatHistoryRecord`s → convert records into `ConversationRound`s → group them by computed `chat_hash` → instantiate `ChatGroup`s → set `HistoryPanelState.chats` and default `selected_chat_id` to the most recent chat.
2. **Append new round**: create a `ConversationRound`, append to the matching `ChatGroup` (or create fallback), update `metadata` (last speaker/timestamp/summary), refresh view while holding selection on the existing chat hash.
3. **Chat selection change**: set `selected_chat_id` to the chosen hash, recompute `visible_range`, and refresh GTK widgets to highlight the row and load timeline data.
