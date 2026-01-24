# Data Model: Window-Local AI Mode Shortcut

## Entities

- **Window**
  - **Fields**: window_id (runtime identifier), ai_mode (current mode), is_focused (runtime flag)
  - **Relationships**: Window owns its AI mode state; no cross-window synchronization required.
  - **Validation Rules**: ai_mode must be one of the supported modes already exposed by the UI.
  - **State Transitions**: ai_mode changes only when the focused window triggers the shortcut or uses the existing UI control.

## Notes

- No new persistent storage is introduced for this feature.
