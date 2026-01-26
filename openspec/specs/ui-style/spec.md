# ui-style Specification

## Purpose
TBD - created by archiving change enable-window-style-classes. Update Purpose after archive.
## Requirements
### Requirement: SearchWindow widgets expose stable CSS classes
Each visible widget that contributes to the floating window’s appearance (the top-level window, layout boxes, search entry, AI toggle/indicator, scrolled container, list box, and individual rows) SHALL set an explicit CSS class name so that user-supplied styles can target them reliably instead of relying on generic GTK selectors.

#### Scenario: Custom style overrides the window and controls
- **WHEN** the user places a `style.css` in `~/.config/stando/style.css` that defines `.search-window`, `.search-main`, `.search-entry`, `.search-ai-button`, `.search-results`, and `.search-result-row`
- **THEN** the floating window renders with the background and margin values from the custom stylesheet rather than the built-in defaults
- **AND** the AI button/indicator, entry, and result rows adopt the colors, padding, and borders provided by those class rules
- **AND** reloading the app picks up the user CSS automatically before the window becomes visible because the existing `style.css` loading path is reused

### Requirement: Result text uses a dedicated class for typography
The text rendered inside each search result row SHALL be wrapped with a GTK widget that has a dedicated CSS class so users can independently adjust label color, weight, and typography without affecting other GTK labels globally.

#### Scenario: Personalized text style takes effect
- **WHEN** the user adds a rule for `.search-result-label` in their custom `style.css` that sets `color: #ff00ff` and `font-weight: bold`
- **THEN** each search result label renders with magenta bold text while the rest of the window continues to obey the user’s general layout styles

