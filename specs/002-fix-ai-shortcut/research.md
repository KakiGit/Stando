# Research: Window-Local AI Mode Shortcut

## Decisions

- **Decision**: Use `GtkShortcutController` attached to each application window with `GtkShortcutTrigger` and `GtkShortcutAction`.
  - **Rationale**: GTK4 provides first-class, window-scoped shortcuts that respect focus and avoid global registration; this aligns with the requirement to keep shortcuts local.
  - **Alternatives considered**: `GtkEventControllerKey` on the window; custom key event handling. These are more manual and less consistent with built-in GTK behavior.

- **Decision**: Bind the shortcut to a window-scoped action (e.g., `win.toggle-ai-mode`) that reuses the existing AI mode switch behavior.
  - **Rationale**: Actions centralize behavior and keep keyboard and UI controls consistent; GTK shortcut actions can activate named actions without extra key event handling.
  - **Alternatives considered**: Directly toggling mode from a key event callback. This duplicates logic and makes it harder to keep UI and shortcut behavior in sync.

- **Decision**: Treat the shortcut as window-local only (no `global-hotkey` registration) and allow GTK to consume the event so text inputs do not receive characters when the shortcut fires.
  - **Rationale**: Shortcut controllers are designed to handle key events and stop propagation when activated, preventing unintended text input.
  - **Alternatives considered**: Leaving the global hotkey and filtering by focus. This risks global conflicts and does not meet the feature requirement.
