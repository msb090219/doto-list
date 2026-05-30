mod app;
mod colors;
mod event;
mod doto;
mod ui;

use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new()?;

    // Initial draw
    terminal.draw(|f| ui::render::<CrosstermBackend<std::io::Stdout>>(f, &app))?;

    // Very simple event loop
    loop {
        // Wait for input
        if !crossterm::event::poll(Duration::from_millis(1000))? {
            terminal.draw(|f| ui::render::<CrosstermBackend<std::io::Stdout>>(f, &app))?;
            continue;
        }

        // Read ONE event
        let event = crossterm::event::read()?;

        if app.should_quit {
            break;
        }

        match event {
            crossterm::event::Event::Key(key) => {
                // Only process KeyPress events, ignore KeyRelease (Windows fix)
                if key.kind == crossterm::event::KeyEventKind::Press {
                    event::handle_key(&mut app, key);
                }
            }
            _ => {}
        }

        terminal.draw(|f| ui::render::<CrosstermBackend<std::io::Stdout>>(f, &app))?;
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}
