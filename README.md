# Doto - Terminal Todo List App

A modern terminal-based todo list application built with Rust and ratatui. Doto stays entirely within the terminal, providing powerful task management without GUI dependencies.

![Rust](https://img.shields.io/badge/Rust-2021%20Stable-orange)
![License](https://img.shields.io/badge/License-MIT-blue)

## Screenshots

### Main Interface
<img width="2372" height="1511" alt="image" src="https://github.com/user-attachments/assets/326d0815-256f-470f-a55e-a5eb41707668" />


### Creating Tasks
<img width="2304" height="1558" alt="image" src="https://github.com/user-attachments/assets/5678414c-20aa-4e39-a423-13703b75fe9f" />


### Managing Tasks
<img width="2254" height="1420" alt="image" src="https://github.com/user-attachments/assets/ffc2634e-c85d-4edb-bcbc-6356a0e03309" />


## Features

- Elegant Nord Frost Theme - Beautiful, easy-on-eyes color palette
- Keyboard-Native - Designed for power users with intuitive shortcuts
- Modal Interface - Clean modal-based UI for creating and managing tasks
- Smart Task Ordering - Completed tasks move to bottom, new tasks at top
- Persistent Storage - Tasks automatically saved to `~/.doto/tasks.json`
- Cross-Platform - Works on Windows, Linux, and macOS

## Installation

### Quick Install (Recommended)

**Windows (PowerShell):**
```powershell
 irm https://github.com/msb090219/doto/releases/latest/download/doto-installer.ps1 | pwsh
```

**macOS/Linux:**
```bash
curl -sSf https://github.com/msb090219/doto/releases/latest/download/install.sh | sh
```

Then add to PATH if needed and run `doto`!

### Build from Source

```bash
git clone https://github.com/msb090219/doto.git
cd doto
cargo build --release
```

The binary will be at `target/release/doto`. Add it to your PATH to use `doto` command anywhere.

## Usage

### Start the Application

```bash
doto
```

### Keyboard Controls

**Creating Tasks:**
- `Shift+T` - Open create modal
- Type your task text
- `Enter` to create
- `Esc` to cancel
- `Backspace` to delete characters
- `Ctrl+U` to clear input

**Navigation:**
- `↑` / `↓` - Move up/down through tasks
- `q` or `Esc` - Quit application

**Managing Tasks:**
- `Enter` - Open action menu for selected task
- `↑` / `↓` - Navigate menu options
- `Enter` - Select action
- Options: ✓ Complete, × Delete, ← Cancel

**Task Behavior:**
- Completed tasks move to bottom automatically
- New tasks appear at top
- Auto-saves to `~/.doto/tasks.json`

### Example Workflow

1. Run `doto` in your terminal
2. Press `Shift+T` to create your first task
3. Type "Learn Rust" and press `Enter`
4. Use arrow keys to navigate
5. Press `Enter` on a task to complete or delete it
6. Press `q` when done

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
