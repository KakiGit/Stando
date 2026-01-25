# Change: Run Stando in the foreground with Esc exit

## Why
- The current architecture keeps Stando alive in the background with tray hooks, which makes it behave like a daemon instead of a single, focused UI workflow.
- Users launching the binary expect the search window to appear immediately and for Esc to close the app, rather than merely hiding the window and leaving a resident process.

## What Changes
- Remove the daemon/tray plumbing so that the application only runs while the foreground search window is open.
- Show the primary search window unconditionally when the app starts rather than relying on a background hotkey handler.
- Update the Escape key handler to hide the window and then quit the application process so the app terminates instead of staying in the tray.
- **BREAKING:** Stando no longer stays resident in the tray or responds to global hotkeys when the window is closed; the process exits immediately after Esc.

## Impact
- Affected specs: `openspec/specs/session-lifecycle/spec.md`
- Affected code: `src/app.rs`, `src/tray.rs`, `src/daemon.rs`, `src/main.rs`, any configuration or documentation that advertises background behavior
