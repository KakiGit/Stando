# Change: center main window at half screen size

## Why
- The current search window defaults to 800×600 pixels and opens where the window manager decides, which can leave it awkwardly sized or off-center on large displays.
- The user wants the window to be more predictable and centered at a comfortable size, improving usability especially on high-resolution screens.

## What Changes
- **Spec:** extend the `session-lifecycle` capability with a requirement that the main window measures 50% of the monitor's width and height and is centered before being shown.
- **Code:** when constructing the search window, compute the primary monitor bounds, size the window to half of that area, and center the window so it stays visible regardless of the window manager.
- **Validation:** ensure the new behavior has a deterministic result by adjusting the window initialization logic and confirming spec alignment.

## Impact
- Affected specs: `session-lifecycle`
- Affected code: `src/ui.rs` (search window creation and sizing logic, any helper modules that surface display metrics)
