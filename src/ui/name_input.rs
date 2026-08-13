use ratatui::layout::Alignment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::App;
use crate::ui::centered;

pub fn draw(f: &mut Frame, app: &App) {
    let t = app.texts();
    let area = centered(f.area(), 58, 10);

    let rank = app
        .last_rank
        .map(|r| format!("{}: {}", t.rank_label, r))
        .unwrap_or_default();

    let widget = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(t.enter_name, Style::default().fg(Color::White))),
        Line::from(Span::styled(rank, Style::default().fg(Color::Cyan))),
        Line::from(""),
        Line::from(Span::styled(
            format!("> {}_", app.name_buffer),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(t.name_hint, Style::default().fg(Color::DarkGray))),
    ])
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(t.high_scores, Style::default().fg(Color::Yellow)))
            .border_style(Style::default().fg(Color::Yellow)),
    );

    f.render_widget(Clear, area);
    f.render_widget(widget, area);
}
