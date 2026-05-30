# Doto - Terminal Todo List App

A modern terminal-based todo list application built with Rust and ratatui. Doto stays entirely within the terminal, providing powerful task management without GUI dependencies.

![Rust](https://img.shields.io/badge/Rust-2021%20Stable-orange)
![License](https://img.shields.io/badge/License-MIT-blue)

## Features

- Elegant Nord Frost Theme - Beautiful, easy-on-eyes color palette
- Keyboard-Native - Designed for power users with intuitive shortcuts
- Modal Interface - Clean modal-based UI for creating and managing tasks
- Smart Task Ordering - Completed tasks move to bottom, new tasks at top
- Persistent Storage - Tasks automatically saved to `~/.doto/tasks.json`
- Cross-Platform - Works on Windows, Linux, and macOS

## Installation

### Build from Source

```bash
git clone https://github.com/msb090219/doto.git
cd doto
cargo build --release
```

The binary will be at `target/release/doto`.

### Running

```bash
cargo run
```

## Usage

| Key | Action |
|-----|--------|
| `Shift+T` | Open create modal |
| `↑` / `↓` | Navigate tasks |
| `Enter` | Open action menu |
| `q` / `Esc` | Quit / Cancel |

## Architecture

```
src/
├── main.rs      # Entry point, terminal setup, event loop
├── app.rs       # Application state and business logic
├── doto.rs      # Doto struct definition and operations
├── ui.rs        # Rendering logic with ratatui widgets
├── event.rs     # Input handling (keyboard events)
└── colors.rs    # Nord Frost color palette constants
```

### Key Design Decisions

- **Immediate-mode rendering** - Simple state-driven UI updates
- **JSON persistence** - Human-readable data format
- **Modal-based workflow** - Reduces cognitive load
- **Event-driven architecture** - Clean separation between input handling and state updates

## Technical Details

### Dependencies
- `ratatui` 0.29 - TUI framework
- `crossterm` 0.28 - Terminal backend
- `serde` + `serde_json` - JSON serialization
- `anyhow` - Error handling
- `chrono` - Timestamps
- `dirs` - Cross-platform directory paths

### Windows-Specific Handling
The application filters events to only process `KeyEventKind::Press` to fix Windows double-key input issues.

## Development

```bash
cargo test
cargo fmt
cargo clippy
```

## License

MIT License - see LICENSE file for details
