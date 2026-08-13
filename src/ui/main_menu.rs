use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;

use crate::app::{App, MenuItem};

pub fn draw(f: &mut Frame, app: &App) {
    let t = app.texts();
    let area = f.area();

    let chunks = Layout::vertical([
        Constraint::Min(3),
        Constraint::Length(10),
        Constraint::Min(2),
    ])
    .split(area);

    let center = Layout::horizontal([
        Constraint::Min(1),
        Constraint::Length(40),
        Constraint::Min(1),
    ])
    .split(chunks[1]);

    let title = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            t.title,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
    ])
    .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let items: Vec<ListItem> = app
        .menu_items()
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let label = match item {
                MenuItem::Continue => t.continue_game,
                MenuItem::NewGame => t.new_game,
                MenuItem::HighScores => t.high_scores,
                MenuItem::Settings => t.settings,
                MenuItem::Language => t.language_label,
                MenuItem::Quit => t.quit,
            };
            let style = if i == app.menu_cursor {
                Style::default().fg(Color::Black).bg(Color::Yellow)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(format!("  {}  ", label))).style(style)
        })
        .collect();

    let menu = List::new(items).block(Block::default().borders(Borders::ALL).title(t.title));
    f.render_widget(menu, center[1]);

    let hint = Paragraph::new(Line::from(Span::styled(
        t.hint_menu,
        Style::default().fg(Color::DarkGray),
    )))
    .alignment(Alignment::Center);
    f.render_widget(hint, chunks[2]);
}
