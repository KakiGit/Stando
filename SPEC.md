# Stando Spec

## UI Style

A style blended with Neumorphism (soft shadow realism) or Skeuomorphic light effects, but the overall foundation is Glassmorphism—a visual trend popularized by modern OS designs.

* Translucent / Frosted Glass Effect – Elements have a semi-transparent background with a subtle blur that lets you see the shapes and colors behind them, mimicking frosted glass.
* Soft Shadows and Glow – Gentle shadows and inner/outer glow effects are used to create depth, giving the interface a floating or layered appearance.
* Rounded Corners – The cards, buttons, and containers feature smooth, rounded edges.
* Minimalist Layout – It uses simple typography and icons with plenty of spacing, leading to a clean and modern look.
* Muted, Gradient Backgrounds – Often gradients or soft blurs with purples, pinks, or blues in pastel tones provide contrast for the translucent foreground.
* Light or Neon Accents – Small touches of highlighted colors (like white or cyan) enhance the futuristic look.

## Configuration Storage

* `~/.config/stando/config.toml` for main [Configuration](#configuration).
* `~/.config/stando/style.css` for custom UI styling.


## App Data Storage

* `~/.stando/chat-history.json` for AI chat history.
* `~/.stando/search-history.json` for search usage history.

## CLI Options

* `--ai-mode` or `-a`: Launch Stando with AI mode enabled by default.
* `--help` or `-h`: Show help information.
* `--verbose` or `-V`: Enable verbose logging for debugging purposes.

## Configuration Options
* `ai_base_url`: Base URL for AI service endpoints.
* `ai_model`: AI model to use for generating responses.
* `ai_api_keys`: Store API keys for AI services (e.g., OpenAI).
* `app_search_paths`: List of directories to search for applications.
* `hotkey_ai_toggle`: Keybinding to toggle AI mode.

## Window Behavior

* Size: Default to 50% of screen width and 50% of screen height.
* Floating Window: The search window floats above other applications and can be moved around the screen.
* Focus on Launch: The search entry is focused automatically when the window opens.
* Dismiss on Escape: Pressing the Escape key closes the search window.
* Resizable: The window can be resized by dragging its edges or corners.
* Always on Top: The window remains above other windows when active.
* Focus On Entry: The text entry field is focused automatically when the window opens.

## Modes

Two primary modes of operation: **Normal Mode** and **AI Mode**.

### Normal Mode

* Standard fuzzy application search.

### AI Mode

* Chat-based interface for AI interactions.
* Activated via toggle button or CLI flag.

## Window Elements

### Common Window Elements

* Search Entry: Text input for user queries.
* AI Toggle Button:
    - Button to switch between Normal and AI modes.
    - The button visually indicates the current mode.
    - The button displays an "AI" label and changes appearance when AI mode is active.
    - Hides when AI features are disabled.

### Normal Mode Specific Elements

* Search Results List: Displays fuzzy-matched applications based on user input.

### AI Mode Specific Elements

* Chat List: Scrollable list of chat messages.
* Chat Content Pane: Scrollable list that displays the full conversation history for the selected chat session.

## Key Binding Behavior

### Common Key Bindings

* `<AI_MODE_TOGGLE>`: Toggle between Normal and AI modes.
* `<ESCAPE>`: Close the search window.
* `<ENTER>`: Open the selected application or send the message to a selected chat.
* `<UP>` / `<DOWN>`: Navigate through search results or chat entries.

### Normal Mode Specific Key Bindings

None.

### AI Mode Specific Key Bindings

* `<NEW_CHAT>`: Start a new chat session.
* `<DELETE_CHAT>`: Delete the selected chat session.
* `<PAGE_UP>` / `<PAGE_DOWN>`: Scroll through chat history.
