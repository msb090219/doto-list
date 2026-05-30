use crate::app::{App, AppMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key(app: &mut App, key: KeyEvent) {
    match app.mode {
        AppMode::View => handle_view_mode(app, key),
        AppMode::ActionMenu => handle_action_menu(app, key),
        AppMode::CreateModal => handle_create_modal(app, key),
    }
}

fn handle_view_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Up => {
            app.move_selection(-1);
        }
        KeyCode::Down => {
            app.move_selection(1);
        }
        KeyCode::Enter => {
            app.show_action_menu();
        }
        KeyCode::Char('T') if key.modifiers.contains(KeyModifiers::SHIFT) => {
            app.open_create_modal();
        }
        KeyCode::Char('q') | KeyCode::Esc => {
            app.should_quit = true;
        }
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        _ => {}
    }
}

fn handle_action_menu(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Up => {
            app.move_menu_selection(-1);
        }
        KeyCode::Down => {
            app.move_menu_selection(1);
        }
        KeyCode::Enter => {
            app.execute_menu_action();
        }
        KeyCode::Esc => {
            app.mode = AppMode::View;
        }
        _ => {}
    }
}

fn handle_create_modal(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Up => {
            app.move_modal_selection(-1);
        }
        KeyCode::Down => {
            app.move_modal_selection(1);
        }
        KeyCode::Enter => {
            app.execute_create_modal_action();
        }
        KeyCode::Esc => {
            app.close_create_modal();
        }
        KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            // When Create is selected, allow typing
            if app.modal_selection == 0 {
                app.input_buffer.push(c);
            }
        }
        KeyCode::Backspace => {
            if app.modal_selection == 0 {
                app.input_buffer.pop();
            }
        }
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if app.modal_selection == 0 {
                app.input_buffer.clear();
            }
        }
        _ => {}
    }
}
