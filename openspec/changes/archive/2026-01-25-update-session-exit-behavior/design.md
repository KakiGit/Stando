## Context
Stando currently keeps running behind the scenes with tray integration and daemon scaffolding. That makes the binary behave more like a background service than a single-shot search window.

## Goals / Non-Goals
- Goals: run Stando entirely in the foreground, show the search window on launch, and fully exit when Esc is pressed. Hide the window only as part of the Esc flow and ensure the process terminates afterwards.
- Non-Goals: re-architecting the hotkey subsystem or recreating a persistent tray menu; we accept that closing the window also closes the app.

## Decisions
- Decision: Remove the daemon/tray scaffolding so the app has no resident components once the window is closed. This keeps the process lifetime tied to visible UI.
- Decision: Replace the current Escape handler (which merely hides the window) with a sequence that reuses the hide logic but also calls `gtk4::Application::quit()` so the entire process shuts down.

## Risks / Trade-offs
- Without a tray or daemon, users cannot keep Stando running to respond to global hotkeys after closing the window. The trade-off is a simpler foreground-only experience that matches the requested behavior.
- If future requirements demand background persistence again, the previous daemon design can be revisited, but for now this change prefers straightforward visibility.

## Open Questions
- Should configuration still expose global hotkeys if the app only runs while open? (Ephemeral; we keep them for the running session but they no longer make sense post-exit.)
