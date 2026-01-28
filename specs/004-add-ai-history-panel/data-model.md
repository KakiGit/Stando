# Data Model: AI History Panel

## Entities

- **ChatHistoryEntry**
  - **Fields**: `id` (UUID or monotonic counter), `timestamp` (ISO-8601 string), `role` (`user` or `assistant`), `summary` (short label for the list), `content` (full message text), `source` (e.g., `prompt` vs `response`).
  - **Relationships**: Stored in `HistoryPanelState.entries`; each entry corresponds to one row in the right-hand list and provides the left-hand detail text when selected.
  - **Validation Rules**: Entries must be ordered chronologically (older timestamps first), `summary` must not be empty when `content` exists, and `role` must map to a known label so the list can communicate user vs assistant context.
  - **State Transitions**: New entries append to the end whenever the user submits a prompt or the AI returns a response; entries remain immutable once appended.

- **HistoryPanelState**
  - **Fields**: `entries` (`Vec<ChatHistoryEntry>`), `selected_entry_id` (`Option<Uuid>`), `selected_visible_range` (`RangeInclusive<usize>` describing the visible window in the list), `empty_state_message` (`String` describing why history is empty).
  - **Relationships**: Owned by the AI mode session (`App`/`SearchWindow`) and consumed by the GTK panel; selection events mutate `selected_entry_id` while leaving `entries` untouched.
  - **Validation Rules**: When `entries` is non-empty, `selected_entry_id` must reference one of the entries; when `entries` is empty, the empty-state message must be shown and no selection is allowed.
  - **State Transitions**: On AI prompt submission the newest entry becomes selected; when the user presses Up/Down or clicks a row, `selected_entry_id` moves within bounds; when history is cleared, the panel transitions to an empty-state message.

## Notes

- The shared state lives behind an `Arc<RwLock<HistoryPanelState>>` so the AI service (background tasks) and GTK UI (main context) can both read/write safely.
- The `selected_visible_range` helps keep keyboard navigation and auto-scroll behavior aligned with the currently focused history entries.
