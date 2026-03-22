# instran

Instant translate — select text, translate to clipboard in one key.

Auto-detects Chinese ↔ English; other languages → English.

## Requirements

- Wayland compositor (Hyprland / Sway / etc.)
- `wl-paste`, `wl-copy` (wl-clipboard)
- `notify-send` (libnotify)
- [Ghostty](https://ghostty.org/) (for popup window)
- Anthropic API key

### Arch Linux

```bash
sudo pacman -S wl-clipboard libnotify ghostty
```

## Build

```bash
git clone https://github.com/leitai231/instran.git && cd instran
cargo build --release
```

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
| `INSTRAN_API_KEY` | falls back to `ANTHROPIC_API_KEY` | API key |
| `INSTRAN_API_URL` | `https://api.anthropic.com/v1/messages` | API endpoint |
| `INSTRAN_MODEL` | `claude-sonnet-4-20250514` | Model ID |

## Usage

### Basic flow

1. **Copy text** with `Ctrl+C` / `Super+C`, or just **select** with mouse
2. Run `instran`
3. A floating popup shows the translation; result is also copied to clipboard
4. **Ctrl+V** to paste anywhere

Clipboard (Ctrl+C) is checked first; if empty, falls back to mouse selection (primary).

### Keybinding (Hyprland)

Add to `~/.config/hypr/bindings.conf`:

```
bind = $mainMod SHIFT, T, exec, instran
```

Add to `~/.config/hypr/hyprland.conf` for the floating popup:

```
windowrule = float on, match:title instran-popup
windowrule = size 600 400, match:title instran-popup
windowrule = center on, match:title instran-popup
```

Then `Super+Shift+T` = translate selected text.

## Architecture

```
src/
├── main.rs          # entry + orchestration
├── clipboard.rs     # wl-paste read (clipboard → primary) / wl-copy write
├── notifier.rs      # notify-send --print-id / --replace-id
├── popup.rs         # ghostty floating popup display
└── translator.rs    # Claude Messages API (ureq, 10s/30s timeout)
```

Flow: `read clipboard/selection → notify loading → call API → write clipboard → notify done → show popup`
