# instran

Instant translate — select text, translate to clipboard in one key.

Auto-detects Chinese ↔ English; other languages → English.

## Requirements

- Wayland compositor (Hyprland / Sway / etc.)
- `wl-paste`, `wl-copy` (wl-clipboard)
- `notify-send` (libnotify)
- Anthropic API key

### Arch Linux

```bash
sudo pacman -S wl-clipboard libnotify
```

## Build

```bash
git clone <repo-url> && cd instran
cargo build --release
```

Binary at `target/release/instran` (~1.9MB).

Install to PATH:

```bash
cargo install --path .
```

## Configuration

Set the API key (required):

```bash
export ANTHROPIC_API_KEY="sk-ant-api03-..."
```

Add to your shell profile (`~/.bashrc` / `~/.zshrc` / `~/.config/fish/config.fish`).

Optional environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `ANTHROPIC_API_KEY` | (required) | Anthropic API key |
| `INSTRAN_API_URL` | `https://api.anthropic.com/v1/messages` | API endpoint |
| `INSTRAN_MODEL` | `claude-sonnet-4-20250514` | Model ID |

## Usage

### Basic flow

1. **Select text** with mouse (enters Wayland Primary Selection)
2. Run `instran`
3. **Ctrl+V** to paste the translation

No Ctrl+C needed — mouse selection is enough.

### Manual test

```bash
# Select some English text, then:
instran

# Check clipboard:
wl-paste
```

### Keybinding (Hyprland)

Add to `~/.config/hypr/hyprland.conf`:

```
bind = $mainMod SHIFT, T, exec, instran
```

Then `Super+Shift+T` = translate selected text.

## Architecture

```
src/
├── main.rs          # entry + orchestration
├── clipboard.rs     # wl-paste -p read / wl-copy write
├── notifier.rs      # notify-send --print-id / --replace-id
└── translator.rs    # Claude Messages API (ureq)
```

Flow: `read primary selection → notify loading → call API → write clipboard → notify done`
