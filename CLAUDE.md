# Dotolist - TUI Todo List App

A terminal-based todo list application built with Rust and ratatui, featuring a Nord Frost color scheme and two-pane layout.

## Project Overview

**Goal:** Build a fully-featured TUI todo app that stays entirely within the terminal, providing powerful task management without GUI dependencies.

**Tech Stack:**
- Rust (latest stable)
- ratatui 0.29 (TUI framework)
- crossterm 0.28 (terminal backend)

**Design Philosophy:**
- Clean, modern terminal aesthetic using Nord Frost colors
- Vim-inspired navigation for developer familiarity
- Immediate-mode rendering for simplicity
- JSON persistence for human-editable data

## Architecture

### Core Components

```
main.rs    → App entry, terminal setup, event loop
app.rs     → Application state (todos, filters, selection)
ui.rs      → All rendering logic (widgets, layout, drawing)
event.rs   → Input handling (key events, mouse clicks)
todo.rs    → Todo struct and CRUD operations
colors.rs  → Nord Frost color palette constants
```

### Data Flow

```
User Input → event.rs → app.rs (state update) → ui.rs (render) → Terminal
                                                      ↓
                                               JSON file (persistence)
```

### State Management

The `App` struct owns all state:
- `todos: Vec<Todo>` - All todo items
- `selected_idx: usize` - Currently selected todo index
- `filter: FilterType` - Current filter (All/Active/Completed)
- `mode: AppMode` - Current app mode (Normal, Insert, Search)
- `undo_stack: Vec<Todo>` - Undo history for deletions

### Layout Structure

```
Terminal (100%)
├── Header (1 row) - Title and stats
├── Main Body (remaining - 1)
│   ├── Left Panel (20% width)
│   │   ├── Filters section
│   │   └── Categories section
│   └── Right Panel (80% width)
│       └── Scrollable todo list
└── Footer (1 row) - Keyboard shortcuts
```

## Design System

### Nord Frost Color Palette

**Primary accents (blue gradient):**
- `#8fbcbb` (Frost Polar Water) - subtle highlights, completed
- `#88c0d0` (Frost Iceberg) - primary accent, selected items
- `#81a1c1` (Frost Frozen) - borders, dividers
- `#5e81ac` (Frost Deep) - muted accents

**Backgrounds (dark):**
- `#2E3440` (Night 0) - main background
- `#3B4252` (Night 1) - panel background
- `#4C566A` (Night 3) - dimmed text, inactive

**Status colors (Aurora):**
- `#BF616A` (Red) - high priority, delete
- `#EBCB8B` (Yellow) - medium priority
- `#A3BE8C` (Green) - low priority, success

**Text:**
- `#D8DEE9` (Snow Storm 1) - primary text
- `#E5E9F0` (Snow Storm 2) - headings

### Widget Hierarchy

```
Layout (full screen)
├── Block (header)
├── Split (horizontal)
│   ├── Block (left panel)
│   │   ├── List (filters)
│   │   └── List (categories)
│   └── Block (right panel)
│       └── List (todos)
└── Block (footer)
```

## Code Conventions

### Rust Style
- Use `cargo fmt` for formatting
- Prefer idiomatic Rust patterns (iterators, `?` operator, etc.)
- Keep functions under 50 lines when possible
- Use `derive(Debug)` for debugging structs

### Error Handling
- Use `Result<T, Box<dyn Error>>` for main fallible functions
- Use `anyhow` for application errors if needed
- Never `unwrap()` in user-facing code paths

### Naming
- Structs: `PascalCase` (e.g., `Todo`, `AppState`)
- Functions: `snake_case` (e.g., `render_todos`, `handle_input`)
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `NIGHT_0`, `FROST_ICEBERG`)

### File Organization
- One public struct per file where logical
- `pub(crate)` for internal module APIs
- Clear module re-exports in `main.rs`

## Development Workflow

### Running the App
```bash
cargo run
```

### Development Mode
```bash
cargo run  # Standard run
cargo watch -x run  # Auto-rebuild on changes (if cargo-watch installed)
```

### Testing
```bash
cargo test
```

### Building for Release
```bash
cargo build --release
# Binary at target/release/dotolist
```

## Key Features Implementation Notes

### Persistence
- File location: `~/.dotolist/tasks.json`
- Auto-save on every state change
- Manual save command: `Ctrl+S`
- Format: JSON array of todo objects

### Input Modes
- **Normal:** Navigation, commands (default)
- **Insert:** Adding/editing todo text
- **Search:** Filtering todos by text

### Filtering Logic
```rust
fn visible_todos(&self) -> Vec<&Todo> {
    self.todos.iter()
        .filter(|t| match self.filter {
            FilterType::All => true,
            FilterType::Active => !t.completed,
            FilterType::Completed => t.completed,
        })
        .filter(|t| self.search_query.is_empty() || t.text.contains(&self.search_query))
        .collect()
}
```

### Priority System
- `H` / `M` / `L` suffix in UI
- Cycle priority: `p` key or `1/2/3` keys
- Colors: Red → Yellow → Frost Deep (low)

## Common Tasks

### Adding a New Widget
1. Define widget struct in `ui.rs`
2. Implement `Widget` trait
3. Add `render_widget` call in main draw closure
4. Apply Nord colors via `colors.rs` constants

### Adding a New Key Command
1. Add key handler in `event.rs`
2. Update `App` state accordingly
3. Add shortcut hint to footer in `ui.rs`
4. Document in `product.md`

### Adding Persistence
1. Add `serde` derive to `Todo` struct
2. Implement `save()` and `load()` functions
3. Call `save()` after every state mutation
4. Call `load()` on app startup

## File Reference

### `src/main.rs`
- Terminal initialization (alt screen, raw mode)
- Main event loop
- Clean shutdown (restore terminal state)

### `src/app.rs`
- `App` struct definition
- State mutation methods
- Business logic (add, delete, toggle, filter)

### `src/ui.rs`
- `render()` function
- Individual component renderers
- Layout calculation logic

### `src/event.rs`
- `handle_key()` function
- Key-to-action mapping
- Input mode switching

### `src/colors.rs`
- Color constants (hex values)
- Helper functions for ratatui `Color` conversion
- Theme preset export

### `src/todo.rs`
- `Todo` struct definition
- Todo operations (new, complete, uncomplete)
- Display formatting

## Known Limitations

- Terminal size: Requires at least 80x24 for usable layout
- Unicode: Limited terminal emoji support varies
- Mouse: Basic click support planned, not priority

## Future Enhancements

Potential features for later iterations:
- Due dates with natural language parsing
- Recurring tasks
- Task dependencies
- Multi-file project support
- Export to markdown
- Git-style staging areas for tasks

## Resources

- [ratatui documentation](https://docs.rs/ratatui)
- [crossterm documentation](https://docs.rs/crossterm)
- [Nord theme](https://www.nordtheme.com)
- [Vim keybinding conventions](https://vim.fandom.com/wiki/Mapping_keys_in_Vim_-_Tutorial_(Part_3))

---

**Last updated:** 2026-05-30
