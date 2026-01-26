# Data Model: Force Window Top

## Stando Window State
- **Purpose**: Tracks the runtime stacking state of the main application window and its visibility relative to other clients.  
- **Fields**:
  - `is_floating` (bool): Whether floating mode is currently assigned to the window.  
  - `workspace_id` (string|int): Identifier of the workspace the floating window is currently on; used to reapply stacking hints after switches.  
  - `display_id` (string|int): Display/monitor identifier where the window resides; used to monitor multi-monitor drags.  
  - `last_override_success` (bool): Flag indicating whether the last stacking override request was honored; guides whether fallback messaging is needed.  
- **Validations**: Cannot reapply floating mode if `workspace_id` or `display_id` is unknown; if override requests fail consecutively, throttle retries to avoid burning CPU.  
- **State transitions**:
  - Normal → Floating on user toggle.  
  - Floating → Normal on toggle-off or when WM rejects overrides (with messaging).  
  - On workspace/display change while floating: reapply stacking and optionally update `workspace_id`/`display_id`.

## Floating Preference
- **Purpose**: Captures the persisted user preference for floating mode, including contextual hints for reapplying the state.  
- **Fields**:
  - `enabled` (bool): Whether the user last left floating mode enabled.  
  - `preferred_workspace` (string|int, optional): Workspace metadata saved when the preference was captured, if available.  
  - `preferred_display` (string|int, optional): Display metadata to prefer after restart.  
- **Validations**: Values stored in the config loader must default to `false` for `enabled` and allow missing workspace/display entries.  
- **State transitions**:
  - Updated whenever the user toggles the floating mode or when the system detects a new primary display/workspace for the floating window.
