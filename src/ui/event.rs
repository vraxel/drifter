use ratatui::layout::Alignment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::App;
use crate::game::EventMessage;

pub fn draw(f: &mut Frame, app: &App, evt: &EventMessage) {
    let t = app.texts();
    let area = f.area();

    let popup_area = crate::ui::centered(area, 60, 8);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(&evt.title, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
        .border_style(Style::default().fg(Color::Yellow));

    let text = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(&evt.body, Style::default().fg(Color::White))),
        Line::from(""),
        Line::from(Span::styled(t.press_any_key, Style::default().fg(Color::DarkGray))),
    ])
    .block(block)
    .wrap(Wrap { trim: true })
    .alignment(Alignment::Center);

    f.render_widget(ratatui::widgets::Clear, popup_area);
    f.render_widget(text, popup_area);
}
