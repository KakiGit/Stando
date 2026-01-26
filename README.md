# Stando

A next-gen selection based search system for Linux. Stando is a keyboard-oriented tool that provides fast file and application search with optional AI integration.

## Features

- **Foreground UI**: Launching Stando immediately displays the search window and keeps the process alive for that window only.
- **Esc to exit**: Pressing Esc hides the window and terminates the app instead of leaving it in the background.
- **Global hotkeys**: Win+Space to invoke UI
- **File search**: Search and open files on the host
- **Application search**: Search and launch applications
- **AI mode**: Toggle with <Control>i, launch with `stando --ai-mode` (alias `-a`) to start with the AI indicator active, and reference files/apps with @ symbol
- **OpenAI integration**: Uses OpenAI API for AI-powered assistance

## Requirements

- Linux (Arch Linux for first version)
- GTK4
- Rust toolchain
- pkg-config

## Building

```bash
cargo build --release
```

## Installation

### Arch Linux (AUR)

```bash
# Install from AUR (when available)
yay -S stando
```

### Manual Installation

1. Build the project:
   ```bash
   cargo build --release
   ```

2. Install the binary:
   ```bash
   sudo cp target/release/stando /usr/local/bin/
   ```

3. Install desktop file:
   ```bash
   cp com.stando.App.desktop ~/.local/share/applications/
   ```

4. Create config directory and file:
   ```bash
   mkdir -p ~/.config/stando
   # Edit ~/.config/stando/config.toml to add your OpenAI API key
   ```

## Configuration

Configuration file location: `~/.config/stando/config.toml`

Example configuration:

```toml
openai_api_key = "your-api-key-here"
search_paths = ["/home/user"]
max_results = 20
hotkey_show = "Super+Space"
hotkey_ai_toggle = "<Control>i"
```

## Usage

1. Start Stando to open the search window immediately (the process runs only while that window is visible)
2. Press **Win+Space** to open the search window
3. Type to search for files or applications
4. Press **Enter** to open the selected result
5. Press **Esc** to close Stando
6. Press **<Control>i** to toggle AI mode
7. In AI mode, use **@filename** or **@appname** to reference files/applications
8. Run `stando --ai-mode` (or `stando -a`) to open directly in AI mode so the UI behaves as if the toggle was activated before the window appeared
9. For debug logging, run `stando --verbose` or `RUST_LOG=debug stando`

## Development

### Dependencies

Arch Linux:
```bash
sudo pacman -S gtk4 libadwaita pkg-config rust
```

### Running

```bash
cargo run
```

## License

MIT
