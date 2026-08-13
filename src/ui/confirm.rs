use ratatui::layout::Alignment;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::App;
use crate::ui::centered;

pub fn draw(f: &mut Frame, app: &App) {
    super::main_menu::draw(f, app);

    let t = app.texts();
    let area = centered(f.area(), 56, 8);

    let widget = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            t.confirm_new_game,
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            t.hint_confirm,
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(t.new_game, Style::default().fg(Color::Yellow)))
            .border_style(Style::default().fg(Color::Yellow)),
    );

    f.render_widget(Clear, area);
    f.render_widget(widget, area);
}
