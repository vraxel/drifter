use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};
use ratatui::Frame;

use crate::app::App;

pub fn draw(f: &mut Frame, app: &App) {
    let t = app.texts();
    let area = f.area();

    let chunks = Layout::vertical([
        Constraint::Min(3),
        Constraint::Length(15),
        Constraint::Min(2),
    ])
    .split(area);

    let center = Layout::horizontal([
        Constraint::Min(1),
        Constraint::Length(56),
        Constraint::Min(1),
    ])
    .split(chunks[1]);

    if app.high_scores.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(t.no_scores, Style::default().fg(Color::DarkGray))),
        ])
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title(t.high_scores));
        f.render_widget(empty, center[1]);
    } else {
        let bold = Style::default().add_modifier(Modifier::BOLD);
        let rows: Vec<Row> = app
            .high_scores
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                Row::new(vec![
                    Cell::from(format!("{}", i + 1)),
                    Cell::from(entry.name.clone()),
                    Cell::from(entry.score.to_string()),
                    Cell::from(entry.health.to_string()),
                    Cell::from(entry.fame.to_string()),
                ])
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(4),
                Constraint::Min(12),
                Constraint::Length(14),
                Constraint::Length(8),
                Constraint::Length(8),
            ],
        )
        .header(Row::new(vec![
            Cell::from(t.rank_label).style(bold),
            Cell::from(t.name_col).style(bold),
            Cell::from(t.score_col).style(bold),
            Cell::from(t.health).style(bold),
            Cell::from(t.fame).style(bold),
        ]))
        .block(Block::default().borders(Borders::ALL).title(t.high_scores));
        f.render_widget(table, center[1]);
    }

    let hint = Paragraph::new(Line::from(Span::styled(
        t.hint_back,
        Style::default().fg(Color::DarkGray),
    )))
    .alignment(Alignment::Center);
    f.render_widget(hint, chunks[2]);
}
