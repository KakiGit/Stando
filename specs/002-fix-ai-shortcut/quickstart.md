# Quickstart: Window-Local AI Mode Shortcut

## Goal

Verify that the AI mode shortcut only affects the focused window and behaves like the existing mode switch control.

## Preconditions

- Stando builds and launches on the target system.
- At least one AI mode is available through the UI.

## Steps

1. Launch Stando and focus its main window.
2. Press the AI mode shortcut.
3. Confirm the mode indicator changes as if the UI control was used.
4. Place cursor in a text input inside the window and press the shortcut.
5. Confirm the mode changes and no characters appear in the input.
6. Open a second Stando window and focus it.
7. Press the shortcut and confirm only the focused window updates.
8. Switch focus to another application and press the shortcut.
9. Confirm Stando does not change mode and does not steal focus.

## Expected Results

- The shortcut only affects the focused Stando window.
- No global shortcut behavior remains for AI mode switching.
- The UI reflects the new mode immediately.
