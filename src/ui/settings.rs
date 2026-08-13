use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::ui::centered;

pub fn draw(f: &mut Frame, app: &App) {
    if app.settings_from_market {
        super::market::draw(f, app);
    }

    let t = app.texts();
    let area = centered(f.area(), 44, 7);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(t.settings, Style::default().fg(Color::Yellow)))
        .border_style(Style::default().fg(Color::Yellow));
    let inner = block.inner(area);

    f.render_widget(Clear, area);
    f.render_widget(block, area);

    let rows = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).split(inner);

    let status = if app.game.hacker_enabled { t.on } else { t.off };
    let item = ListItem::new(Line::from(format!(
        "  {}: {}  ",
        t.hacker_events, status
    )))
    .style(Style::default().fg(Color::Black).bg(Color::Yellow));
    f.render_widget(List::new(vec![item]), rows[0]);

    let hint = Paragraph::new(Line::from(Span::styled(
        t.hint_settings,
        Style::default().fg(Color::DarkGray),
    )))
    .alignment(Alignment::Center);
    f.render_widget(hint, rows[1]);
}
