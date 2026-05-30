# Doto - Terminal Todo List App

A modern, elegant terminal-based todo list application built with Rust and ratatui. Doto stays entirely within the terminal, providing powerful task management without GUI dependencies.

![Rust](https://img.shields.io/badge/Rust-2021%20Stable-orange)
![License](https://img.shields.io/badge/License-MIT-blue)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey)

## ✨ Features

- **Elegant Nord Frost Theme** - Beautiful, easy-on-eyes color palette inspired by the Nord theme
- **Keyboard-Native** - Designed for power users with intuitive keyboard shortcuts
- **Modal Interface** - Clean modal-based UI for creating and managing tasks
- **Smart Task Ordering** - Completed tasks automatically move to the bottom; new tasks appear at the top
- **Persistent Storage** - Tasks are automatically saved to `~/.doto/tasks.json`
- **Cross-Platform** - Works on Windows, Linux, and macOS

## 🎯 Key Functionality

- **Create Tasks**: Press `Shift+T` to open the creation modal
- **Navigate**: Use `↑` and `↓` arrow keys to move between tasks
- **Task Actions**: Press `Enter` on a task to see options (Complete, Delete, Cancel)
- **Quick Quit**: Press `q` or `Ctrl+C` to exit

## 🚀 Installation

### Prerequisites
- Rust toolchain (1.70+)
- Terminal with ANSI support

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/doto.git
cd doto

# Build the release binary
cargo build --release

# The binary will be at target/release/doto
```

### Running

```bash
# From the project directory
cargo run

# Or use the built binary
./target/release/doto
```

## 🎨 Design Philosophy

Doto was built with three core principles:

1. **Terminal-First** - A true TUI experience that feels natural in the CLI
2. **Visual Clarity** - The Nord Frost color scheme provides excellent contrast while remaining easy on the eyes
3. **Immediate Feedback** - Every action provides instant visual confirmation

## 🏗️ Architecture

```
src/
├── main.rs      # Entry point, terminal setup, event loop
├── app.rs       # Application state and business logic
├── doto.rs      # Doto struct definition and operations
├── ui.rs        # All rendering logic with ratatui widgets
├── event.rs     # Input handling (keyboard events)
└── colors.rs    # Nord Frost color palette constants
```

### Key Design Decisions

- **Immediate-mode rendering** - Simple state-driven UI updates
- **JSON persistence** - Human-readable data format for easy inspection
- **Modal-based workflow** - Reduces cognitive load by focusing on one action at a time
- **Event-driven architecture** - Clean separation between input handling and state updates

## 📸 Screenshots

Coming soon...

## 🔧 Technical Details

### Dependencies
- `ratatui` 0.29 - TUI framework
- `crossterm` 0.28 - Terminal backend
- `serde` + `serde_json` - JSON serialization
- `anyhow` - Error handling
- `chrono` - Timestamps
- `dirs` - Cross-platform directory paths

### Windows-Specific Handling
The application includes a fix for the Windows double-key input issue by filtering events to only process `KeyEventKind::Press` events.

## 🛠️ Development

```bash
# Run with auto-rebuild (requires cargo-watch)
cargo watch -x run

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## 📝 Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Shift+T` | Open create modal |
| `↑` / `↓` | Navigate tasks |
| `Enter` | Open action menu |
| `q` / `Esc` | Quit / Cancel |
| `Ctrl+C` | Force quit |

## 🔮 Future Enhancements

Potential features for future iterations:
- Task priorities
- Due dates with natural language parsing
- Task categories/tags
- Search functionality
- Undo/redo system
- Export to markdown

## 📄 License

MIT License - see LICENSE file for details

## 👤 Author

Built as a portfolio project to demonstrate Rust and TUI development skills.

---

**Note**: Doto is a learning project focused on clean architecture, modern Rust practices, and creating polished terminal user interfaces.
