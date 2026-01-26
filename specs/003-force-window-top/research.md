# Research: Force Window Top

## Decision: rely on native GTK/window manager stacking hints
**Rationale**: GTK4 already exposes the window-stacking APIs needed for occupancy across tiling WMs, and the existing app already negotiates GTK/Wayland/GLib state, so reusing these hooks keeps the feature lean while satisfying the spec’s focus on max availability.  
**Alternatives considered**: embedding a floating overlay inside the compositor or launching a companion process to keep a proxy window above others—both add complexity and would still need stacking hints to work inside Wayland/tiling environments, so they were ruled out.

## Decision: persist floating preference in local config
**Rationale**: The spec explicitly wants the window to remember floating mode between sessions; storing a single boolean (and optional metadata about the last workspace/display) in `config.toml` keeps persistence simple and aligns with existing preference storage patterns.  
**Alternatives considered**: storing the preference in a separate file or relying on session restoration from the desktop environment—both are overkill for a single toggle and harder to integrate with the existing config loader.

## Decision: expose a UI indicator that doubles as a toggle and surface failure messaging
**Rationale**: Users need quick visibility into floating mode status and a way to deactivate it, plus the spec demands graceful handling when stacking overrides fail. An indicator affordance meets both requirements while keeping the control near the window frame.  
**Alternatives considered**: hiding the control deep in menus or relying solely on notifications; those would make the state harder to discover and retry.
