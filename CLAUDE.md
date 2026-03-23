# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run

```bash
cargo build --release        # Build optimized binary (size-optimized, LTO, stripped)
cargo install --path .       # Install to ~/.cargo/bin/instran
cargo clippy                 # Lint
cargo test                   # Run tests (none currently)
```

## What This Is

**instran** — a single-purpose Wayland CLI that translates selected text via the Anthropic Claude API. The user selects text (or Ctrl+C), presses a hotkey, and the translation appears in a floating Ghostty popup + clipboard.

Flow: `read clipboard/selection → notify → call Claude API → write clipboard → notify → show popup`

## Architecture

236 lines of Rust across 5 files in `src/`:

- **main.rs** — Orchestration: reads env vars (API key, URL, model), runs the pipeline, handles top-level errors
- **clipboard.rs** — Reads from Wayland clipboard (`wl-paste`), falls back to primary selection (`wl-paste --primary`); writes via `wl-copy`
- **translator.rs** — Builds and sends the Anthropic Messages API request. System prompt is hardened against prompt injection — wraps input in `[TRANSLATE]...[/TRANSLATE]` markers and pre-fills assistant response. Temperature=0, max_tokens=4096
- **notifier.rs** — Desktop notifications via `notify-send`. Uses `--print-id`/`--replace-id` to update a single notification through the loading→success/error lifecycle
- **popup.rs** — Spawns a detached Ghostty terminal with a bash heredoc displaying the translation. Fire-and-forget

All external tool calls use `std::process::Command`. Error handling uses `Result<T, Box<dyn std::error::Error>>` throughout.

## Environment Variables

| Variable | Required | Default |
|----------|----------|---------|
| `INSTRAN_API_KEY` | One of these two | — |
| `ANTHROPIC_API_KEY` | (fallback) | — |
| `INSTRAN_API_URL` | No | `https://api.anthropic.com/v1/messages` |
| `INSTRAN_MODEL` | No | `claude-sonnet-4-20250514` |

## Runtime Dependencies

Wayland-only. Requires: `wl-clipboard`, `notify-send` (libnotify), `ghostty`, `bash`.

## Translation Behavior

Auto-detects language direction: Chinese ↔ English, all other languages → English. The system prompt strictly forbids the model from answering questions or adding explanations — output must be the literal translation only.
