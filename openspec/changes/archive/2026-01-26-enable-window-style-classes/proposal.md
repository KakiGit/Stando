# Change: Expose window components to configurable styling

## Why
Users should be able to tune the appearance of every widget that makes up Stando's floating window (container, entry, AI toggle, results list, rows, and text) through their own `style.css` instead of being locked to the built-in look and feel.

## What Changes
- Assign meaningful CSS classes to each top-level widget inside `SearchWindow` (window, boxes, entry, AI button/indicator, list, rows, and labels) so custom styles can target them without relying on generic selectors.
- Expand the bundled `assets/style.css` to cover those classes, including dedicated text style rules, and document how users can override them via their config-provided stylesheet.
- Update user documentation to explain where the custom style file lives and which selectors are available for overriding the window and text styling.

## Impact
- Affected specs: `ui-style` (new capability for configurable UI styling)
- Affected code: `src/ui.rs`, `assets/style.css`, `README.md`
