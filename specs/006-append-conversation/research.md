# Research

## Decision: Keep AI responses within the selected conversation
**Rationale**: The feature request explicitly asks for the newly typed AI input to appear in the currently selected chat. Reusing the existing chat-selection + history append code reduces risk and aligns with the requirement that conversations preserve order.
**Alternatives considered**:
- Open a new conversation when the AI text arrives (rejected because the user explicitly wants the message appended to the selected chat).
- Rely on a background service to move messages post-facto (rejected because it would break the one-second append success criterion).

## Decision: Validate selection and ignore empty messages before persistence
**Rationale**: FR-002/FR-005/FR-006 demand that messages target an explicit chat when one exists, that the system auto-creates the first chat when history is empty, and that whitespace-only submissions are dropped. Running these checks before persisting keeps the UI consistent and guards against data loss.
**Alternatives considered**:
- Let the UI always append and retroactively delete invalid entries (dangerous and harder to test).
- Default to a fallback “untitled” conversation when no chat is selected (violates the explicit error requirement and duplicates the empty-history creation path that already exists for FR-005).

## Open Questions
None; the specification is clear enough for implementation.
