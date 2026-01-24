# Stando

A next-gen selection based search system for Linux. Stando is a keyboard-oriented tool that provides fast file and application search with optional AI integration.

## Features

- **Run as daemon**: Background process that stays running
- **System tray integration**: Available in system tray
- **Global hotkeys**: Win+Space to invoke UI, Esc to hide
- **File search**: Search and open files on the host
- **Application search**: Search and launch applications
- **AI mode**: Toggle with <Control>i, reference files/apps with @ symbol
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

1. Start Stando (it will run as a daemon and appear in system tray)
2. Press **Win+Space** to open the search window
3. Type to search for files or applications
4. Press **Enter** to open the selected result
5. Press **Esc** to hide the window
6. Press **<Control>i** to toggle AI mode
7. In AI mode, use **@filename** or **@appname** to reference files/applications
8. For debug logging, run `stando --verbose` or `RUST_LOG=debug stando`

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

### Running as daemon

```bash
cargo run -- --daemon
```

## License

MIT
