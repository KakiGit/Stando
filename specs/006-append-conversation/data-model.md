# Data Model

## Entity: Chat
- **Fields**:
  - `id: String` – unique conversation identifier used in UI selection and persistence.
  - `title: Option<String>` – user-visible label from the navigation tree.
  - `messages: Vec<Message>` – ordered log of `Message` records in this chat.
  - `updated_at: DateTime<Utc>` – timestamp that can power sorting in the UI.
- **Validation**: AI submissions SHALL target a selected chat when chat history exists; if the history is empty, the first submission SHALL create and select a new chat before appending (per FR-002/FR-005).
- **State transitions**: `selected` → `submit message` → `messages.push(new_message)` → `history persisted`.

## Entity: Message
- **Fields**:
  - `id: Uuid` – stable identifier for deduplication.
  - `chat_id: String` – foreign key (business rule) into the recording chat.
  - `author: MessageAuthor` – distinguishes user vs. AI.
  - `content: String` – trimmed text that will be rendered inline; must not be empty or whitespace-only (FR-006).
  - `timestamp: DateTime<Utc>` – used for ordering and meeting SC-002.
  - `message_type: MessageType` – e.g., `UserInput`, `AIResponse`.
- **Validation**: `content` is trimmed before persistence and rejected if empty; `timestamp` generated at send time.

## Relationships
- `Chat` has many `Message` entries (1:N).
- Each `Message` belongs to exactly one `Chat` by `chat_id`.

## Additional Notes
- The existing `UsageHistory` serialization handles `ChatHistoryRecord` snapshots where `messages` is a list of message DTOs; append operations reuse that path so we do not duplicate storage code.
