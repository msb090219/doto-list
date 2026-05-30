# Doto - Terminal Todo List App

A modern terminal-based todo list application built with Rust and ratatui. Doto stays entirely within the terminal, providing powerful task management without GUI dependencies.

![Rust](https://img.shields.io/badge/Rust-2021%20Stable-orange)
![License](https://img.shields.io/badge/License-MIT-blue)

## Screenshots

### Main Interface
<img width="800" height="520" alt="image" src="https://github.com/user-attachments/assets/326d0815-256f-470f-a55e-a5eb41707668" />


### Creating Tasks
<img width="800" height="520" alt="image" src="https://github.com/user-attachments/assets/5678414c-20aa-4e39-a423-13703b75fe9f" />


### Managing Tasks
<img width="800" height="520" alt="image" src="https://github.com/user-attachments/assets/9bf88732-ef18-46c0-9cee-6bd68e9fcb79" />


## Features

- **Elegant Nord Frost Theme** - Beautiful, easy-on-eyes color palette
- **Keyboard-Native** - Designed for power users with intuitive shortcuts
- **Modal Interface** - Clean modal-based UI for creating and managing tasks
- **Smart Task Ordering** - Completed tasks move to bottom, new tasks at top
- **Persistent Storage** - Tasks automatically saved to `~/.doto/tasks.json`
- **Cross-Platform** - Works on Windows, Linux, and macOS

## Installation

### Quick Install (Recommended)

The installation scripts will automatically download the latest release and add `doto` to your PATH.

#### Windows

Open PowerShell and run:

```powershell
irm https://github.com/msb090219/doto/releases/latest/download/doto-installer.ps1 | pwsh
```

**After installation:** Restart your terminal and type `doto` to start using the app.

#### macOS

Open Terminal and run:

```bash
curl -sSf https://github.com/msb090219/doto/releases/latest/download/install.sh | sh
```

**After installation:** Restart your terminal or run `source ~/.zshrc` and type `doto`.

#### Linux

Open your terminal and run:

```bash
curl -sSf https://github.com/msb090219/doto/releases/latest/download/install.sh | sh
```

**After installation:** Restart your terminal or run `source ~/.bashrc` and type `doto`.

### Build from Source

If you prefer to build from source or want to customize the code:

#### Prerequisites

- Rust (latest stable): [Install Rust](https://www.rust-lang.org/tools/install)
- Git

#### Build Steps

```bash
# Clone the repository
git clone https://github.com/msb090219/doto.git
cd doto

# Build the release binary
cargo build --release

# The binary will be at target/release/doto (or target/release/doto.exe on Windows)
```

#### Manual PATH Setup (for source builds)

**Windows:**
```powershell
# Copy to a directory in your PATH or add to PATH
copy target\release\doto.exe C:\Users\YourName\.local\bin\doto.exe

# Or add to system PATH through System Properties > Environment Variables
```

**macOS/Linux:**
```bash
# Copy to local bin directory
mkdir -p ~/.local/bin
cp target/release/doto ~/.local/bin/doto

# Add to PATH if not already there
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc
source ~/.bashrc  # or ~/.zshrc
```

## Usage

### Starting the Application

Once installed, simply run:

```bash
doto
```

### Keyboard Controls

#### Creating Tasks

- **`Shift+T`** - Open create modal
- **Type your task text** - Start typing to enter your task
- **`Enter`** - Create the task
- **`Esc`** - Cancel and return to main view
- **`Backspace`** - Delete last character
- **`Ctrl+U`** - Clear all input

#### Navigation

- **`↑` / `↓`** - Move up/down through tasks
- **`q` or `Esc`** - Quit application

#### Managing Tasks

- **`Enter`** - Open action menu for selected task
- **`↑` / `↓`** - Navigate menu options
- **`Enter`** - Select action
- **Options:** ✓ Complete, × Delete, ← Cancel

#### Task Behavior

- **Completed tasks** move to bottom automatically
- **New tasks** appear at top
- **Auto-save** to `~/.doto/tasks.json`

### Example Workflow

1. Run `doto` in your terminal
2. Press `Shift+T` to create your first task
3. Type "Learn Rust" and press `Enter`
4. Use arrow keys to navigate between tasks
5. Press `Enter` on a task to complete or delete it
6. Press `q` when done

## Troubleshooting

### "doto: command not found"

**Windows:**
- Make sure you've restarted your terminal after installation
- Check that `C:\Users\YourName\.local\bin` is in your PATH
- Try running the full path: `C:\Users\YourName\.local\bin\doto.exe`

**macOS/Linux:**
- Run `source ~/.bashrc` or `source ~/.zshrc` 
- Check that `~/.local/bin` is in your PATH: `echo $PATH | grep .local/bin`
- Try running the full path: `~/.local/bin/doto`

### Installation Script Fails

**No releases found:**
- This means no GitHub releases have been created yet
- The installer will work once releases are available
- You can build from source in the meantime

**Permission denied:**
- Make sure the install script is executable: `chmod +x install.sh`
- Try running with bash explicitly: `bash install.sh`

### Terminal Display Issues

**Colors look wrong:**
- Some terminals may not support the Nord Frost color scheme
- Try a different terminal emulator (Windows Terminal, iTerm2, etc.)

**Screen layout issues:**
- Make sure your terminal is at least 80x24 characters
- Resize your terminal window if needed

## Data Storage

Your tasks are stored in:
- **Windows:** `C:\Users\YourName\.doto\tasks.json`
- **macOS/Linux:** `~/.doto/tasks.json`

This file is human-readable JSON, so you can:
- Manually edit your tasks
- Backup your task list
- Transfer between machines

## Development

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

### Building for Release

```bash
cargo build --release
```

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

## Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest features
- Submit pull requests
- Improve documentation

## License

MIT License - see LICENSE file for details

## Acknowledgments

- Built with [ratatui](https://github.com/ratatui-org/ratatui)
- Inspired by [Nord Theme](https://www.nordtheme.com)
- Created for terminal productivity enthusiasts
