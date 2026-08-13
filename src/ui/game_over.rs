use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::App;

pub fn draw(f: &mut Frame, app: &App) {
    let t = app.texts();
    let g = &app.game;
    let area = f.area();

    let chunks = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(16),
        Constraint::Min(1),
    ])
    .split(area);

    let center = Layout::horizontal([
        Constraint::Min(1),
        Constraint::Length(56),
        Constraint::Min(1),
    ])
    .split(chunks[1]);

    let score = g.score();
    let score_color = if score > 0 { Color::Green } else { Color::Red };

    let outcome = if g.dead {
        t.death
    } else if score <= 0 {
        t.broke
    } else {
        t.going_home
    };

    let rank_line = match app.last_rank {
        Some(rank) => Line::from(Span::styled(
            format!("{}: {}", t.rank_label, rank),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        None => Line::from(Span::styled(t.no_rank, Style::default().fg(Color::DarkGray))),
    };

    let widget = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(outcome, Style::default().fg(Color::White))),
        Line::from(""),
        Line::from(vec![
            Span::raw(format!("{}: ", t.final_score)),
            Span::styled(
                score.to_string(),
                Style::default()
                    .fg(score_color)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(format!(
            "{}: {}   {}: {}   {}: {}",
            t.cash, g.cash, t.bank, g.bank, t.debt, g.debt
        )),
        Line::from(format!(
            "{}: {}   {}: {}",
            t.health, g.health, t.fame, g.fame
        )),
        Line::from(""),
        rank_line,
        Line::from(""),
        Line::from(Span::styled(t.play_again, Style::default().fg(Color::Cyan))),
    ])
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                t.game_over,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
    );

    f.render_widget(widget, center[1]);
}
