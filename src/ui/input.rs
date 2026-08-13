use ratatui::layout::Alignment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::data;
use crate::ui::centered;

/// The number being edited, plus what it will cost if confirmed.
struct Amount {
    value: i64,
    max: i64,
    total: Option<i64>,
}

fn draw_popup(f: &mut Frame, app: &App, title: String, label: &str, amount: Amount, hint: &str) {
    let Amount { value, max, total } = amount;
    let t = app.texts();
    let area = centered(f.area(), 54, 9);

    let total_line = match total {
        Some(amount) => Line::from(vec![
            Span::raw(format!("{}: ", t.total)),
            Span::styled(amount.to_string(), Style::default().fg(Color::Cyan)),
        ]),
        None => Line::from(""),
    };

    let widget = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::raw(format!("{}: ", label)),
            Span::styled(
                value.to_string(),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(" / {}", max)),
        ]),
        total_line,
        Line::from(""),
        Line::from(Span::styled(hint, Style::default().fg(Color::DarkGray))),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(title, Style::default().fg(Color::Yellow)))
            .border_style(Style::default().fg(Color::Yellow)),
    );

    f.render_widget(Clear, area);
    f.render_widget(widget, area);
}

pub fn draw_buy(f: &mut Frame, app: &App, good_index: usize) {
    super::market::draw(f, app);
    let t = app.texts();
    let title = format!("{} - {}", t.buy_quantity, data::good_name(good_index, app.lang));
    let total = app.input_value * app.game.prices[good_index];
    draw_popup(
        f,
        app,
        title,
        t.quantity,
        Amount {
            value: app.input_value,
            max: app.input_max,
            total: Some(total),
        },
        t.hint_qty,
    );
}

pub fn draw_sell(f: &mut Frame, app: &App, good_index: usize) {
    super::market::draw(f, app);
    let t = app.texts();
    let title = format!(
        "{} - {}",
        t.sell_quantity,
        data::good_name(good_index, app.lang)
    );
    let total = app.input_value * app.game.prices[good_index];
    draw_popup(
        f,
        app,
        title,
        t.quantity,
        Amount {
            value: app.input_value,
            max: app.input_max,
            total: Some(total),
        },
        t.hint_qty,
    );
}

pub fn draw_bank(f: &mut Frame, app: &App, deposit: bool) {
    super::market::draw(f, app);
    let t = app.texts();
    let title = if deposit {
        t.deposit_amount
    } else {
        t.withdraw_amount
    };
    let label = if deposit { t.deposit } else { t.withdraw };
    draw_popup(
        f,
        app,
        title.to_string(),
        label,
        Amount {
            value: app.input_value,
            max: app.input_max,
            total: None,
        },
        t.hint_amount,
    );
}

pub fn draw_hospital(f: &mut Frame, app: &App) {
    super::market::draw(f, app);
    let t = app.texts();
    let cost = app.hospital_points as i64 * data::HOSPITAL_COST_PER_POINT;
    draw_popup(
        f,
        app,
        t.hospital_name.to_string(),
        t.hospital_heal_points,
        Amount {
            value: app.hospital_points as i64,
            max: app.hospital_max as i64,
            total: Some(cost),
        },
        t.hint_heal,
    );
}

pub fn draw_repay(f: &mut Frame, app: &App) {
    super::market::draw(f, app);
    let t = app.texts();
    draw_popup(
        f,
        app,
        t.post_office.to_string(),
        t.repay_amount,
        Amount {
            value: app.input_value,
            max: app.input_max,
            total: None,
        },
        t.hint_amount,
    );
}
