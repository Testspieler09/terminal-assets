# terminal-assets (ta)

A CLI tool for rendering text-based animations to video. Define scenes in code, render them to frames, and encode the result to MP4, WebM, or GIF - with full control over fonts, colors, and output format.

## Defining Scenes

Scenes are defined directly in Rust source and compiled into the binary. Adding or editing a scene requires editing the source and recompiling with `cargo b` / `cargo b --release`.
Otherwise running it directly with `cargo run -- <args>` is also an option.

## Features

- **Scene rendering** - render one scene by name, a selection, or all scenes at once
- **Interactive mode** - TUI scene selector for picking what to render without memorizing names
- **Multiple output formats** - H.264 (MP4), VP9 (WebM), and GIF via ffmpeg
- **Font loading with fallback** - specify a font by filename; the tool searches your provided font directories and falls back to OS font directories automatically (macOS user and system folders, Linux system fonts, Windows Fonts)
- **Font caching** - fonts shared across targets in a scene are loaded from disk once and reused
- **Configurable output directory** - all rendered frames and encoded videos are written to a directory of your choice
- **Environment check script** - `check_ffmpeg.sh` verifies your ffmpeg installation, codec support (H.264, VP9, GIF, palette filters), and shared library linkage before you run a render

## Usage

```
ta --output <DIR> [--scenes <SCENE>...] [--interactive] [--all] [--font-dirs <DIR>...]
```

| Flag | Description |
|---|---|
| `--output` | Directory to write rendered output into (required) |
| `--scenes` | One or more scene names to render |
| `--interactive` | Open TUI to select scenes |
| `--all` | Render all scenes (default when no mode flag is given) |
| `--font-dirs` | Font search directories (defaults to OS font paths) |

## Requirements

- ffmpeg with libx264, libvpx-vp9, and palette filter support
- Run `./check_ffmpeg.sh` to verify your environment before first use
