use crate::app::{App, AppMode};
use crate::colors::*;
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame<'_>, app: &App) {
    let size = f.area();

    // Always render dotos first
    let dotos_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(size);

    render_dotos::<B>(f, app, dotos_area[0]);
    render_help::<B>(f, app, dotos_area[1]);

    // Then render modal overlays on top
    match app.mode {
        AppMode::CreateModal => {
            render_create_modal::<B>(f, app, size);
        }
        AppMode::ActionMenu => {
            render_action_menu::<B>(f, app, size);
        }
        AppMode::View => {}
    }
}

fn render_dotos<B: Backend>(f: &mut Frame<'_>, app: &App, area: Rect) {
    if app.dotos.is_empty() {
        // Empty state
        let empty_text = vec![
            Line::from(""),
            Line::from("No dotos yet!"),
            Line::from("Press Shift+T to create one"),
            Line::from(""),
        ];

        let paragraph = Paragraph::new(Text::from(empty_text))
            .block(
                Block::default()
                    .title(" Dotos ")
                    .title_style(Style::default().fg(FROST_POLAR_WATER))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(FROST_FROZEN))
                    .style(Style::default().bg(NIGHT_0)),
            )
            .style(Style::default().fg(NIGHT_3).bg(NIGHT_0))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false });

        f.render_widget(paragraph, area);
        return;
    }

    let visible_indices = app.visible_dotos();

    let items: Vec<ListItem> = visible_indices
        .iter()
        .enumerate()
        .map(|(idx, &doto_idx)| {
            let doto = &app.dotos[doto_idx];
            let is_selected = idx == app.selected_idx;

            let status = if doto.completed { "[✓]" } else { "[ ]" };
            let prefix = if is_selected { "▶ " } else { "  " };

            let content = format!("{} {} {}", prefix, status, doto.text);

            let style = if is_selected {
                Style::default()
                    .fg(AURORA_GREEN)
                    .bg(NIGHT_2)
                    .add_modifier(Modifier::BOLD)
            } else if doto.completed {
                Style::default().fg(FROST_POLAR_WATER).bg(NIGHT_0)
            } else {
                Style::default().fg(SNOW_STORM_1).bg(NIGHT_0)
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Dotos ")
                .title_style(Style::default().fg(FROST_POLAR_WATER))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(FROST_FROZEN))
                .style(Style::default().bg(NIGHT_0)),
        )
        .style(Style::default().bg(NIGHT_0));

    f.render_widget(list, area);
}

fn render_create_modal<B: Backend>(f: &mut Frame<'_>, app: &App, area: Rect) {
    let options = vec!["✓ Create", "✗ Cancel"];

    let modal_width = 40.min(area.width.saturating_sub(4) as usize);
    let modal_height = 8;

    let modal_area = Rect {
        x: (area.width - modal_width as u16) / 2,
        y: (area.height - modal_height) / 2,
        width: modal_width as u16,
        height: modal_height,
    };

    // Split modal into input area and menu
    let modal_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(modal_area);

    let input_area = modal_chunks[0];
    let menu_area = modal_chunks[1];

    // Render input box
    let cursor = "█";
    let input_text = vec![
        Span::styled("New doto: ", Style::default().fg(FROST_POLAR_WATER)),
        Span::styled(&app.input_buffer, Style::default().fg(SNOW_STORM_1)),
        Span::styled(cursor, Style::default().fg(FROST_ICEBERG)),
    ];

    let paragraph = Paragraph::new(Line::from(input_text))
        .block(
            Block::default()
                .title(" Create Doto ")
                .title_style(Style::default().fg(FROST_ICEBERG))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(FROST_ICEBERG))
                .style(Style::default().bg(NIGHT_1)),
        )
        .style(Style::default().bg(NIGHT_1))
        .wrap(Wrap { trim: false });

    // Render menu options
    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(idx, option)| {
            let is_selected = idx == app.modal_selection;
            let prefix = if is_selected { "▶ " } else { "  " };

            let style = if is_selected {
                Style::default()
                    .fg(AURORA_GREEN)
                    .bg(NIGHT_2)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(SNOW_STORM_1).bg(NIGHT_1)
            };

            ListItem::new(format!("{} {}", prefix, option)).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(FROST_FROZEN))
                .style(Style::default().bg(NIGHT_1)),
        )
        .style(Style::default().bg(NIGHT_1));

    // Clear modal area and render content
    f.render_widget(Clear, modal_area);
    f.render_widget(paragraph, input_area);
    f.render_widget(list, menu_area);
}

fn render_action_menu<B: Backend>(f: &mut Frame<'_>, app: &App, area: Rect) {
    let actions = vec!["✓ Complete", "× Delete", "← Cancel"];

    // Get the selected doto text for context
    let doto_context = if let Some(doto_idx) = app.visible_dotos().get(app.selected_idx) {
        if let Some(doto) = app.dotos.get(*doto_idx) {
            format!("Actions for: {}", doto.text)
        } else {
            "Actions".to_string()
        }
    } else {
        "Actions".to_string()
    };

    let menu_width = 35.min(area.width.saturating_sub(4) as usize);
    let menu_height = 6;

    let menu_area = Rect {
        x: (area.width - menu_width as u16) / 2,
        y: (area.height - menu_height) / 2,
        width: menu_width as u16,
        height: menu_height,
    };

    let items: Vec<ListItem> = actions
        .iter()
        .enumerate()
        .map(|(idx, action)| {
            let is_selected = idx == app.menu_selection;
            let prefix = if is_selected { "▶ " } else { "  " };

            let style = if is_selected {
                Style::default()
                    .fg(AURORA_GREEN)
                    .bg(NIGHT_2)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(SNOW_STORM_1).bg(NIGHT_1)
            };

            ListItem::new(format!("{} {}", prefix, action)).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(format!(" {} ", doto_context))
                .title_style(Style::default().fg(FROST_ICEBERG))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(FROST_ICEBERG))
                .style(Style::default().bg(NIGHT_1)),
        )
        .style(Style::default().bg(NIGHT_1));

    f.render_widget(Clear, menu_area);
    f.render_widget(list, menu_area);
}

fn render_help<B: Backend>(f: &mut Frame<'_>, _app: &App, area: Rect) {
    let help_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Shift+T", Style::default().fg(AURORA_GREEN).add_modifier(Modifier::BOLD)),
            Span::styled(" create  ", Style::default().fg(SNOW_STORM_1)),
            Span::styled("↑↓", Style::default().fg(AURORA_GREEN).add_modifier(Modifier::BOLD)),
            Span::styled(" navigate  ", Style::default().fg(SNOW_STORM_1)),
            Span::styled("Enter", Style::default().fg(AURORA_GREEN).add_modifier(Modifier::BOLD)),
            Span::styled(" actions  ", Style::default().fg(SNOW_STORM_1)),
            Span::styled("q", Style::default().fg(AURORA_GREEN).add_modifier(Modifier::BOLD)),
            Span::styled(" quit", Style::default().fg(SNOW_STORM_1)),
        ]),
    ];

    let paragraph = Paragraph::new(Text::from(help_text))
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(FROST_FROZEN))
                .style(Style::default().bg(NIGHT_1)),
        )
        .style(Style::default().fg(SNOW_STORM_1).bg(NIGHT_1))
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}
