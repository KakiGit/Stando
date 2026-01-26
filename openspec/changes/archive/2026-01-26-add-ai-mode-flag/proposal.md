# Change: Add CLI flag for AI mode on startup

## Why
Users currently have to open Stando then toggle AI mode manually, which slows workflows that depend on the AI assistant. A CLI flag lets automation or keyboard-focused users launch directly into AI mode so they can start with `@`-prefixed queries without extra key presses.

## What Changes
- Add a `--ai-mode` (alias `-a`) CLI flag and propagate it through the startup path so the app can honor the request before displaying the window.
- Initialize the AI mode state and UI indicator from the flag prior to showing the main window so the search entry behaves as if the toggle been activated.
- Document the new flag in usage docs and update any relevant user-facing descriptions of AI mode.

## Impact
- Affected specs: `ai-mode` (new capability covering CLI-controlled AI mode state)
- Affected code: `src/main.rs`, `src/app.rs`, `README.md`
