mod confirm;
mod event;
mod facility;
mod game_over;
mod high_scores;
mod input;
mod main_menu;
pub mod market;
mod name_input;
mod settings;

use ratatui::layout::Rect;
use unicode_width::UnicodeWidthStr;
use ratatui::Frame;

use crate::app::{App, Screen};

pub fn draw(f: &mut Frame, app: &App) {
    match &app.screen {
        Screen::MainMenu => main_menu::draw(f, app),
        Screen::ConfirmNewGame => confirm::draw(f, app),
        Screen::HighScores => high_scores::draw(f, app),
        Screen::Market => market::draw(f, app),
        Screen::BuyInput { good_index } => input::draw_buy(f, app, *good_index),
        Screen::SellInput { good_index } => input::draw_sell(f, app, *good_index),
        Screen::BankMenu => facility::draw_bank_menu(f, app),
        Screen::BankInput { deposit } => input::draw_bank(f, app, *deposit),
        Screen::Hospital => facility::draw_hospital(f, app),
        Screen::HospitalInput => input::draw_hospital(f, app),
        Screen::RepayDebt => facility::draw_repay(f, app),
        Screen::RepayInput => input::draw_repay(f, app),
        Screen::RentHouse => facility::draw_rent(f, app),
        Screen::Cafe => facility::draw_cafe(f, app),
        Screen::EventPopup(evt) => event::draw(f, app, evt),
        Screen::NameInput => name_input::draw(f, app),
        Screen::GameOver => game_over::draw(f, app),
        Screen::Settings => settings::draw(f, app),
    }
}

pub fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);
    let x = area.x + (area.width - width) / 2;
    let y = area.y + (area.height - height) / 2;
    Rect::new(x, y, width, height)
}

/// Thousands separators - balances run into the millions late game.
pub fn money(value: i64) -> String {
    let digits = value.unsigned_abs().to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    if value < 0 {
        format!("-{}", out)
    } else {
        out
    }
}

/// Left-pads to a display width; use for right-aligning a value column.
pub fn pad_start(text: &str, width: usize) -> String {
    let used = UnicodeWidthStr::width(text);
    match width.checked_sub(used) {
        Some(slack) if slack > 0 => format!("{}{}", " ".repeat(slack), text),
        _ => text.to_string(),
    }
}


/// Pads to a display width, so CJK names (two columns per glyph) line up.
pub fn pad_center(text: &str, width: usize) -> String {
    let used = UnicodeWidthStr::width(text);
    if used >= width {
        return text.to_string();
    }
    let slack = width - used;
    let left = slack / 2;
    format!("{}{}{}", " ".repeat(left), text, " ".repeat(slack - left))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_groups_digits_in_threes() {
        assert_eq!(money(0), "0");
        assert_eq!(money(999), "999");
        assert_eq!(money(1000), "1,000");
        assert_eq!(money(13591), "13,591");
        assert_eq!(money(1234567), "1,234,567");
        assert_eq!(money(-2500), "-2,500");
    }

    #[test]
    fn edge_padding_counts_display_columns_too() {
        assert_eq!(pad_start("95", 6), "    95");
        assert_eq!(pad_start("overflowing", 3), "overflowing");
    }

    #[test]
    fn padding_counts_display_columns_not_chars() {
        // Three CJK glyphs occupy six columns, so only two spaces are left.
        assert_eq!(pad_center("建国门", 8), " 建国门 ");
        assert_eq!(pad_center("abc", 7), "  abc  ");
        assert_eq!(pad_center("too-long-name", 4), "too-long-name");
    }
}
