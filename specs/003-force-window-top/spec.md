# Feature Specification: Force Window Top

**Feature Branch**: `003-force-window-top`  
**Created**: 2026-01-25  
**Status**: Draft  
**Input**: User description: "make the window float on top of other windows. It should be floating even in WMs like i3, sway and hyprland"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Always-visible monitor (Priority: P1)

As a person running Stando inside a tiling window manager such as i3, sway, or hyprland, I need the main window to stay visually above every other tiled client so I can continuously monitor my workspace without hunting through hidden panes.

**Why this priority**: The primary value of Stando is keeping key information in sight; losing it to other tiled windows would defeat the feature request.

**Independent Test**: Enable the floating flag, open or focus several other tiled windows, and swap workspaces to confirm the Stando window never becomes fully obscured.

**Acceptance Scenarios**:

1. **Given** the user has enabled floating mode, **When** they open a new window or focus an existing tiled window, **Then** the Stando window continues to render above the new window and remains tappable.
2. **Given** floating mode is active, **When** the user changes workspaces and later returns, **Then** the Stando window reappears above other clients on the destination workspace without requiring manual repositioning.

---

### User Story 2 - Multi-display focus (Priority: P2)

As a multi-monitor operator, I want to pin the Stando window on the screen where I am currently tracking information so that dragging other windows around or interacting with non-Stando apps does not hide the important visual.

**Why this priority**: Maintaining control over which monitor hosts the floating window reduces fumbling during multi-app workflows, improving productivity.

**Independent Test**: Toggle the floating flag while the window sits on each monitor and check that the window stays topmost on the selected display even as focus shifts elsewhere.

**Acceptance Scenarios**:

1. **Given** the floating toggle is on, **When** the user drags the Stando window to another display or enters fullscreen on a different screen, **Then** the Stando window remains perched above any stacks on that monitor until the toggle is explicitly turned off.

---

### User Story 3 - Graceful degradation (Priority: P3)

As someone running in an environment where the compositor or window manager rejects stacking overrides, I want the application to explain the limitation and fall back to a normal window so that I understand what happened and can continue using the tool.

**Why this priority**: Users cannot rely on always-on-top if the environment forbids it, so they should receive clear feedback when the request fails.

**Independent Test**: Attempt to enable floating mode on a workspace known to block stacking overrides and verify that the UI shows the explanation while the window remains controllable.

**Acceptance Scenarios**:

1. **Given** the WM refuses the stacking override, **When** the user activates floating mode, **Then** the system informs them that the environment blocked the request and keeps Stando as a regular window until the environment changes or they disable the toggle.

---

### Edge Cases

- When the tiling window manager reorders clients quickly (for example during tiling resizes), the floating window should reassert itself within the next frame to avoid blinking behind other windows.
- When multiple floating applications exist, the user should still be able to interact with Stando without manually rearranging other forced-top windows.
- When remote desktop, nested compositors, or Wayland compatibility layers remove override capabilities, the feature should quietly default to normal stacking after notifying the user.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Users MUST have a persistent "Hold on top" control that pins the main Stando window above every other window when enabled.

#### Scenario: Activating floating mode
- **Given** the user wants constant visibility, **When** they toggle the control into the active position, **Then** the Stando window remains above every other open window or tiled client without resetting after adjacent windows open.

- **FR-002**: The system MUST reassert the floating state after workspace changes, fullscreen transitions, and focus switches so the window never drifts behind other clients while floating mode is active.

#### Scenario: Workspace and focus changes
- **Given** floating mode is active, **When** the user switches workspaces, maximizes another app, or shifts focus, **Then** the window automatically returns to the top of the stack and is interactable without requiring manual rearrangement.

- **FR-003**: The application MUST remember the last floating preference and restore it when the user reopens the session so they are not forced to toggle it again.

#### Scenario: Preference persistence
- **Given** the user left floating mode enabled at shutdown, **When** they reopen the application on the same machine, **Then** the window resumes floating mode without further input, matching their previous preference.

- **FR-004**: The user interface MUST display a clear indicator (icon, badge, or similar) when the window is in floating mode and the indicator MUST also serve as a toggle so the state can be changed without digging through menus.

#### Scenario: Indicator and toggle
- **Given** the window is floating, **When** the user clicks the indicator, **Then** the indicator reflects the new disabled state and the window returns to normal stacking, confirming the toggle works from that affordance.

- **FR-005**: If the window manager or compositor declines stacking requests, the system MUST surface a concise explanation and keep the window in its default stacking order while the user continues to work.

#### Scenario: Degraded environments
- **Given** a display server rejects stacking overrides, **When** the user enables floating mode, **Then** the system explains the limitation and leaves the window in the standard stacking order until the environment changes or the user disables the toggle.

### Key Entities *(include if feature involves data)*

- **Stando Window State**: Represents the current stacking and focus status of the window, including whether floating mode is active and which workspace or display it currently occupies.
- **Floating Preference**: Captures the user's desire to keep Stando on top across sessions, including whether the setting is enabled and any metadata about the last known workspace or monitor.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With floating mode enabled, 98% of workspace-change tests (at least ten sequential switches) show the Stando window remaining visually on top of other windows without requiring user repositioning.
- **SC-002**: Users can toggle the floating mode and observe the state change within five seconds across supported window managers, keeping the time from intent to confirmation low.
- **SC-003**: At most one support ticket per release cycle references confusion about hidden Stando windows when floating mode is on, indicating that the feature behaves predictably.
- **SC-004**: 90% of users running tiling window managers report that their Stando window stayed accessible throughout a monitoring session of at least five minutes.

## Assumptions

- The application already has a main window that can accept stacking hints from the operating system.
- The UI can display a persistent toggle or indicator without requiring a complete redesign of existing controls.
- Users expect floating mode to persist until they explicitly disable it or change their environment.

## Dependencies

- Access to the host window manager or compositor APIs so the application can request stacking overrides and monitor workspace/display changes.
