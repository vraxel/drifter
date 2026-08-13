use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Gauge, Paragraph, Row, Table};
use ratatui::Frame;

use crate::app::{App, Panel};
use crate::data;
use crate::ui::{money, pad_center, pad_start};

/// Floor for either panel band once the terminal gets short.
const TRADE_MIN_HEIGHT: u16 = 6;
const HEADER_HEIGHT: u16 = 3;
const FACILITY_HEIGHT: u16 = 3;
/// Readouts per row in the status grid.
const STATUS_COLS: usize = 2;
/// Label line plus value line; anything spare becomes breathing room.
const STATUS_TILE_MIN: u16 = 2;
/// Upper bound on the value slab so it tracks its caption.
const STATUS_SLAB_WIDTH: usize = 13;

pub fn draw(f: &mut Frame, app: &App) {
    // The station/status band takes Min, so it absorbs whatever height the
    // terminal has spare and the layout always reaches the bottom edge.
    let frame = f.area();
    let chrome = HEADER_HEIGHT + FACILITY_HEIGHT + 2;
    let panels = frame.height.saturating_sub(chrome);

    let rows = Layout::vertical([
        Constraint::Length(HEADER_HEIGHT),
        Constraint::Length(panels * 6 / 10),
        Constraint::Min(TRADE_MIN_HEIGHT),
        Constraint::Length(FACILITY_HEIGHT),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .split(frame);

    draw_header(f, app, rows[0]);

    let trade = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[1]);
    draw_market(f, app, trade[0]);
    draw_bag(f, app, trade[1]);

    let lower = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[2]);
    draw_status(f, app, lower[0]);
    draw_stations(f, app, lower[1]);

    draw_facilities(f, app, rows[3]);
    draw_message(f, app, rows[4]);

    let hint = Paragraph::new(Line::from(Span::styled(
        app.texts().hint_market,
        Style::default().fg(Color::DarkGray),
    )));
    f.render_widget(hint, rows[5]);
}

fn panel(title: String, focused: bool, accent: Color) -> Block<'static> {
    let border = if focused {
        Style::default().fg(accent).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border)
        .title(Span::styled(
            format!(" {} ", title),
            Style::default().fg(if focused { accent } else { Color::Gray }),
        ))
}

/// Day counter as a filling bar - 40 days is a full bar.
fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let t = app.texts();
    let g = &app.game;

    let location = match g.current_location {
        Some(i) => data::location_names(g.subway_mode, app.lang)[i],
        None => "---",
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(Span::styled(
            format!(" {} ", t.title),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ))
        .title(
            Line::from(Span::styled(
                format!(" {} ", location),
                Style::default().fg(Color::Green),
            ))
            .alignment(Alignment::Right),
        );
    let inner = block.inner(area);
    f.render_widget(block, area);

    let elapsed = g.days_elapsed().clamp(0, data::TOTAL_DAYS);
    let ratio = elapsed as f64 / data::TOTAL_DAYS as f64;
    let remaining = data::TOTAL_DAYS - elapsed;
    let colour = if remaining <= 5 {
        Color::Red
    } else if remaining <= 12 {
        Color::Yellow
    } else {
        Color::Cyan
    };

    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(colour).bg(Color::Black))
        .ratio(ratio)
        .label(Span::styled(
            format!("{} {}/{}", t.day, elapsed, data::TOTAL_DAYS),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ));
    f.render_widget(gauge, inner);
}

fn draw_market(f: &mut Frame, app: &App, area: Rect) {
    let t = app.texts();
    let focused = app.focus == Panel::Market;
    let bold = Style::default().add_modifier(Modifier::BOLD);

    let rows: Vec<Row> = app
        .available_goods()
        .iter()
        .enumerate()
        .map(|(i, &(good_index, price))| {
            // Against what you paid, so a profitable exit is visible at a glance.
            let price_color = match app.game.avg_cost(good_index) {
                Some(cost) if price > cost => Color::Green,
                Some(cost) if price < cost => Color::Red,
                _ => Color::White,
            };
            let row = Row::new(vec![
                Cell::from(data::good_name(good_index, app.lang)),
                Cell::from(Line::from(money(price)).alignment(Alignment::Right))
                    .style(Style::default().fg(price_color)),
            ]);
            if focused && i == app.market_cursor {
                row.style(Style::default().bg(Color::Cyan).fg(Color::Black))
            } else {
                row
            }
        })
        .collect();

    let table = Table::new(rows, [Constraint::Min(14), Constraint::Length(12)])
        .header(Row::new(vec![
            Cell::from(t.goods_name).style(bold),
            Cell::from(Line::from(t.price).alignment(Alignment::Right)).style(bold),
        ]))
        .block(panel(t.market.to_string(), focused, Color::Cyan));
    f.render_widget(table, area);
}

fn draw_bag(f: &mut Frame, app: &App, area: Rect) {
    let t = app.texts();
    let focused = app.focus == Panel::Inventory;
    let bold = Style::default().add_modifier(Modifier::BOLD);

    let rows: Vec<Row> = app
        .inventory_goods()
        .iter()
        .enumerate()
        .map(|(i, &(good_index, qty))| {
            let cost = app
                .game
                .avg_cost(good_index)
                .map(money)
                .unwrap_or_else(|| "-".into());
            let row = Row::new(vec![
                Cell::from(data::good_name(good_index, app.lang)),
                Cell::from(Line::from(cost).alignment(Alignment::Right)),
                Cell::from(Line::from(qty.to_string()).alignment(Alignment::Right)),
            ]);
            if focused && i == app.inventory_cursor {
                row.style(Style::default().bg(Color::Green).fg(Color::Black))
            } else {
                row
            }
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Min(12),
            Constraint::Length(11),
            Constraint::Length(7),
        ],
    )
    .header(Row::new(vec![
        Cell::from(t.goods_name).style(bold),
        Cell::from(Line::from(t.avg_cost).alignment(Alignment::Right)).style(bold),
        Cell::from(Line::from(t.quantity).alignment(Alignment::Right)).style(bold),
    ]))
    .block(panel(t.my_goods.to_string(), focused, Color::Green));
    f.render_widget(table, area);
}

struct Readout {
    label: &'static str,
    value: String,
    color: Color,
}

fn readouts(app: &App) -> Vec<Readout> {
    let t = app.texts();
    let g = &app.game;

    let debt_color = if g.debt > data::DEBT_PUNISHMENT_THRESHOLD {
        Color::Red
    } else if g.debt > 0 {
        Color::Yellow
    } else {
        Color::Green
    };
    let health_color = if g.health < 20 {
        Color::Red
    } else if g.health < 50 {
        Color::Yellow
    } else {
        Color::Green
    };
    let fame_color = if g.fame < 60 { Color::Red } else { Color::Green };

    vec![
        Readout { label: t.cash, value: money(g.cash), color: Color::Green },
        Readout { label: t.bank, value: money(g.bank), color: Color::Cyan },
        Readout { label: t.debt, value: money(g.debt), color: debt_color },
        Readout { label: t.health, value: g.health.to_string(), color: health_color },
        Readout { label: t.fame, value: g.fame.to_string(), color: fame_color },
        Readout {
            label: t.inventory,
            value: format!("{}/{}", g.total_goods(), g.capacity),
            color: Color::White,
        },
    ]
}

fn draw_status(f: &mut Frame, app: &App, area: Rect) {
    let t = app.texts();
    let block = panel(t.status.to_string(), false, Color::Yellow);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let items = readouts(app);
    let grid_rows = items.len().div_ceil(STATUS_COLS) as u16;
    if grid_rows == 0 || inner.height == 0 {
        return;
    }

    // One tile shape for all six, so cash and bag capacity read as one family
    // instead of headline numbers over footnotes.
    let tile_height = (inner.height / grid_rows).max(1);
    let bands = Layout::vertical(vec![Constraint::Length(tile_height); grid_rows as usize])
        .split(inner);

    for (row, chunk) in items.chunks(STATUS_COLS).enumerate() {
        let columns = Layout::horizontal(vec![
            Constraint::Ratio(1, STATUS_COLS as u32);
            STATUS_COLS
        ])
        .split(bands[row]);

        for (item, cell) in chunk.iter().zip(columns.iter()) {
            draw_readout(f, item, *cell);
        }
    }
}

fn draw_readout(f: &mut Frame, item: &Readout, area: Rect) {
    if area.height < STATUS_TILE_MIN {
        // Too short for two lines - keep the value, drop the caption.
        f.render_widget(Paragraph::new(value_line(item, area.width)), area);
        return;
    }

    let lines = vec![
        Line::from(Span::styled(
            format!("  {}", item.label),
            Style::default().fg(Color::Gray),
        )),
        value_line(item, area.width),
    ];
    f.render_widget(Paragraph::new(lines), area);
}

fn value_line(item: &Readout, width: u16) -> Line<'static> {
    // Capped so the slab stays next to its caption instead of drifting to the
    // far edge of a wide panel; digits stay right-aligned inside it.
    let slab = (width as usize).saturating_sub(4).clamp(4, STATUS_SLAB_WIDTH);
    Line::from(vec![
        Span::raw("  "),
        Span::styled(
            format!(" {} ", pad_start(&item.value, slab)),
            Style::default()
                .bg(Color::Black)
                .fg(item.color)
                .add_modifier(Modifier::BOLD),
        ),
    ])
}

fn draw_stations(f: &mut Frame, app: &App, area: Rect) {
    let t = app.texts();
    let focused = app.focus == Panel::Locations;
    let map = if app.game.subway_mode {
        t.subway_map
    } else {
        t.surface_map
    };

    let block = panel(format!("{}  [T]", map), focused, Color::Yellow);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let rows = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(inner);

    let names = data::location_names(app.game.subway_mode, app.lang);
    let cols = data::location_grid_cols(app.lang);
    // Capped so the names read as a cluster of buttons rather than drifting apart.
    let cell_width = (rows[0].width as usize / cols)
        .saturating_sub(1)
        .clamp(4, 18);

    let lines: Vec<Line> = names
        .chunks(cols)
        .enumerate()
        .map(|(row, chunk)| {
            let spans: Vec<Span> = chunk
                .iter()
                .enumerate()
                .flat_map(|(col, name)| {
                    let index = row * cols + col;
                    let here = app.game.current_location == Some(index);
                    let selected = focused && index == app.travel_cursor;

                    let style = if selected {
                        Style::default()
                            .bg(Color::Yellow)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD)
                    } else if here {
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    [
                        Span::raw(" "),
                        Span::styled(pad_center(name, cell_width), style),
                    ]
                })
                .collect();
            Line::from(spans)
        })
        .collect();
    f.render_widget(Paragraph::new(lines), rows[0]);

    let hint_style = if focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let hint = Paragraph::new(Line::from(Span::styled(t.station_hint, hint_style)))
        .alignment(Alignment::Center);
    f.render_widget(hint, rows[1]);
}

/// The original's bottom button row: one key per place you can walk into.
fn draw_facilities(f: &mut Frame, app: &App, area: Rect) {
    let t = app.texts();
    let block = panel(t.facilities.to_string(), false, Color::Magenta);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let entries = [
        ("T", t.depart, Color::Yellow),
        ("B", t.bank_name, Color::Magenta),
        ("H", t.hospital_name, Color::Red),
        ("P", t.post_office, Color::White),
        ("R", t.house_agency, Color::Blue),
        ("N", t.internet_cafe, Color::LightGreen),
    ];

    let spans: Vec<Span> = entries
        .iter()
        .flat_map(|(key, label, color)| {
            [
                Span::styled(
                    format!("  [{}] ", key),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(*label, Style::default().fg(*color)),
            ]
        })
        .collect();

    f.render_widget(Paragraph::new(Line::from(spans)), inner);
}

fn draw_message(f: &mut Frame, app: &App, area: Rect) {
    let line = match &app.message {
        Some(msg) => Line::from(Span::styled(
            format!(" {}", msg),
            Style::default().fg(Color::Red),
        )),
        None => Line::from(""),
    };
    f.render_widget(Paragraph::new(line), area);
}
