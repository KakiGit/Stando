## ADDED Requirements
### Requirement: Center the search window at half-screen size on launch
The application SHALL size the main search window to 50% of the monitor's width and height and position it so the window is centered on the primary display before the search UI is shown.

#### Scenario: Launching the app shows a centered half-screen window
- **WHEN** `stando` starts on a display with detectable bounds
- **THEN** the search window receives a width equal to half of the primary monitor width and a height equal to half of its height
- **AND** the window is moved so its center aligns with the monitor's center point before calling `show()`/`present()`

#### Scenario: Window still opens in a reasonable size when bounds are unavailable
- **WHEN** the display subsystem does not expose monitor metrics (e.g., headless or early init)
- **THEN** the search window falls back to the previous default dimensions and still becomes visible in the foreground
