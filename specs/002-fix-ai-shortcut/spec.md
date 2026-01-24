# Feature Specification: Window-Local AI Mode Shortcut

**Feature Branch**: `002-fix-ai-shortcut`  
**Created**: 2026-01-24  
**Status**: Draft  
**Input**: User description: "fix the shortcut key binding for AI mode switching. Change it to local key binding to the window. And use glib or GTK build-in functions as much as possible"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Switch AI mode in focused window (Priority: P1)

As a user, I can switch AI mode with the keyboard shortcut when the app window is focused so I can change modes quickly without using the mouse.

**Why this priority**: This is the primary workflow described by the feature request.

**Independent Test**: Focus the app window, press the shortcut, and confirm the AI mode changes using the same behavior as the existing UI control.

**Acceptance Scenarios**:

1. **Given** the app window is focused and an AI mode is active, **When** I press the AI mode shortcut, **Then** the AI mode changes according to the existing in-app mode switch behavior.
2. **Given** the app window is focused and a text input is active, **When** I press the AI mode shortcut, **Then** the AI mode changes and no unintended characters appear in the input.

---

### User Story 2 - Shortcut does not act globally (Priority: P2)

As a user, I expect the AI mode shortcut to affect only the focused app window so it does not trigger when I am working in other apps.

**Why this priority**: Prevents unexpected behavior outside the app and matches user expectations for window-local shortcuts.

**Independent Test**: Switch focus to another application and press the shortcut; verify the app does not change mode or steal focus.

**Acceptance Scenarios**:

1. **Given** the app window is not focused, **When** I press the AI mode shortcut, **Then** the AI mode does not change and the app does not take focus.
2. **Given** two app windows are open, **When** I press the AI mode shortcut in the focused window, **Then** only that window changes AI mode.

---

### Edge Cases

- What happens when no app window is open and the shortcut is pressed?
- What happens when the shortcut is pressed repeatedly in quick succession?
- What happens when the shortcut conflicts with a system-reserved key combination?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: When the app window is focused, the shortcut MUST switch AI mode using the same outcome as the existing in-app mode switch control.
- **FR-002**: The shortcut MUST only be active for the focused app window and MUST NOT act as a global shortcut.
- **FR-003**: When the app window is not focused, the shortcut MUST NOT change AI mode or bring the app to the foreground.
- **FR-004**: When multiple app windows are open, the shortcut MUST change AI mode only in the focused window.
- **FR-005**: The UI MUST reflect the new AI mode immediately after the shortcut action.
- **FR-006**: Triggering the shortcut MUST NOT insert characters into any focused text input.

## Assumptions

- The current AI mode shortcut key combination remains unchanged unless a conflict exists.
- AI mode switching already has a defined behavior via the existing UI control, and the shortcut should mirror that behavior.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In usability tests, 95% of users can switch AI mode with the shortcut in a focused window on the first attempt.
- **SC-002**: In 100 attempts with the app unfocused, the shortcut triggers zero AI mode changes and never steals focus.
- **SC-003**: With two app windows open, the shortcut updates only the focused window in 100% of trials.
- **SC-004**: Users report the AI mode change feels instantaneous (under 1 second) in 90% of survey responses.
