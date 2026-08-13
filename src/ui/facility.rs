use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::data;
use crate::ui::centered;

fn info_popup(f: &mut Frame, title: &str, color: Color, lines: Vec<Line>) {
    let area = centered(f.area(), 56, 9);
    let widget = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(title.to_string(), Style::default().fg(color)))
                .border_style(Style::default().fg(color)),
        );
    f.render_widget(Clear, area);
    f.render_widget(widget, area);
}

pub fn draw_bank_menu(f: &mut Frame, app: &App) {
    super::market::draw(f, app);
    let t = app.texts();
    let area = centered(f.area(), 44, 9);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(t.bank_name, Style::default().fg(Color::Magenta)))
        .border_style(Style::default().fg(Color::Magenta));
    let inner = block.inner(area);

    f.render_widget(Clear, area);
    f.render_widget(block, area);

    let rows = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(2),
        Constraint::Length(1),
    ])
    .split(inner);

    let summary = Paragraph::new(Line::from(Span::styled(
        format!("{}: {}   {}: {}", t.cash, app.game.cash, t.bank, app.game.bank),
        Style::default().fg(Color::Cyan),
    )))
    .alignment(Alignment::Center);
    f.render_widget(summary, rows[0]);

    let items: Vec<ListItem> = [t.deposit, t.withdraw]
        .iter()
        .enumerate()
        .map(|(i, label)| {
            let style = if i == app.bank_cursor {
                Style::default().fg(Color::Black).bg(Color::Magenta)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(format!("  {}  ", label))).style(style)
        })
        .collect();
    f.render_widget(List::new(items), rows[1]);

    let hint = Paragraph::new(Line::from(Span::styled(
        t.hint_settings,
        Style::default().fg(Color::DarkGray),
    )))
    .alignment(Alignment::Center);
    f.render_widget(hint, rows[2]);
}

pub fn draw_hospital(f: &mut Frame, app: &App) {
    super::market::draw(f, app);
    let t = app.texts();
    let healthy = app.game.health >= 100;

    info_popup(
        f,
        t.hospital_name,
        Color::Red,
        vec![
            Line::from(""),
            Line::from(Span::styled(
                if healthy {
                    t.hospital_healthy
                } else {
                    t.hospital_greeting
                },
                Style::default().fg(Color::White),
            )),
            Line::from(""),
            Line::from(Span::styled(
                format!(
                    "{}: {}/100   {}",
                    t.health, app.game.health, t.hospital_cost_per_point
                ),
                Style::default().fg(Color::Cyan),
            )),
            Line::from(""),
            Line::from(Span::styled(
                if healthy { t.hint_back } else { t.hint_confirm },
                Style::default().fg(Color::DarkGray),
            )),
        ],
    );
}

pub fn draw_repay(f: &mut Frame, app: &App) {
    super::market::draw(f, app);
    let t = app.texts();
    let debt_free = app.game.debt == 0;

    let summary = if debt_free {
        t.debt_none.to_string()
    } else {
        format!(
            "{}: {}   {}: {}",
            t.debt, app.game.debt, t.cash, app.game.cash
        )
    };

    info_popup(
        f,
        t.post_office,
        Color::White,
        vec![
            Line::from(""),
            Line::from(Span::styled(summary, Style::default().fg(Color::White))),
            Line::from(""),
            Line::from(Span::styled(
                if debt_free { t.hint_back } else { t.repay_hint },
                Style::default().fg(Color::DarkGray),
            )),
        ],
    );
}

pub fn draw_rent(f: &mut Frame, app: &App) {
    super::market::draw(f, app);
    let t = app.texts();

    info_popup(
        f,
        t.house_agency,
        Color::Blue,
        vec![
            Line::from(""),
            Line::from(Span::styled(
                format!(
                    "{}: {}/{}   {}: {}",
                    t.inventory,
                    app.game.capacity,
                    data::MAX_CAPACITY,
                    t.cash,
                    app.game.cash
                ),
                Style::default().fg(Color::Cyan),
            )),
            Line::from(Span::styled(
                format!("{}: {}", t.house_min_cash, data::HOUSE_MIN_CASH),
                Style::default().fg(Color::Yellow),
            )),
            Line::from(""),
            Line::from(Span::styled(
                if app.game.can_rent_house() {
                    t.house_rent_hint
                } else {
                    t.hint_back
                },
                Style::default().fg(Color::DarkGray),
            )),
        ],
    );
}

pub fn draw_cafe(f: &mut Frame, app: &App) {
    super::market::draw(f, app);
    let t = app.texts();

    info_popup(
        f,
        t.internet_cafe,
        Color::LightGreen,
        vec![
            Line::from(""),
            Line::from(Span::styled(
                format!(
                    "{}: {}/{}   {}: {}",
                    t.cafe_visits,
                    app.game.cafe_visits,
                    data::MAX_CAFE_VISITS,
                    t.cash,
                    app.game.cash
                ),
                Style::default().fg(Color::Cyan),
            )),
            Line::from(Span::styled(
                t.cafe_need_cash,
                Style::default().fg(Color::Yellow),
            )),
            Line::from(""),
            Line::from(Span::styled(
                if app.game.can_visit_cafe() {
                    t.cafe_enter_hint
                } else {
                    t.hint_back
                },
                Style::default().fg(Color::DarkGray),
            )),
        ],
    );
}
