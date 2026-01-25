# session-lifecycle Specification

## Purpose
TBD - created by archiving change update-session-exit-behavior. Update Purpose after archive.
## Requirements
### Requirement: Launch in the foreground with a visible window
The application SHALL display the main search window as soon as the binary runs and shall keep running only while that window is visible.

#### Scenario: Executing Stando brings up the UI immediately
- **WHEN** the user runs `stando` from a terminal or launcher
- **THEN** the search window appears with focus without waiting for a hotkey or tray interaction
- **AND** the process remains active only for the duration that the window stays visible

### Requirement: Escape terminates the session after reusing the hide logic
The application SHALL reuse the existing hide path to clear the window state and then invoke the application quit sequence when the user presses Esc so the process exits instead of persisting in the background.

#### Scenario: Pressing Esc quits the app rather than hiding it
- **WHEN** the user presses Esc while the search window has focus
- **THEN** the window hide logic runs and `gtk4::Application::quit()` is called
- **AND** the operating system reports the `stando` process as stopped (no resident background process)

### Requirement: No daemon or tray persistence after exit
The application SHALL not keep any tray icons, background threads, or daemon-entrypoints running after the window closes and SHALL only be restartable by launching the binary again.

#### Scenario: Esc path leaves no background helpers
- **WHEN** the user presses Esc or otherwise triggers the quit flow
- **THEN** no tray icon is left behind and no daemon-like helpers continue running
- **AND** restarting Stando requires executing the binary anew

### Requirement: Exit once a selected result opens
The application SHALL quit immediately after a user selects and opens a search result so that each invokation either shows the UI or launches the selection once.

#### Scenario: Opening a result closes the UI
- **WHEN** the user activates a search result through the entry or list
- **THEN** the selected item opens, the hide logic runs, and `gtk4::Application::quit()` is invoked
- **AND** the process no longer appears in process listings or system trays without needing additional input

