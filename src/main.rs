mod app;
mod data;
mod game;
mod i18n;
mod ui;

use std::io;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;

fn main() -> io::Result<()> {
    install_panic_hook();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let result = run(&mut terminal);

    restore_terminal();
    terminal.show_cursor()?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = App::new();

    while app.running {
        terminal.draw(|f| ui::draw(f, &app))?;
        app.handle_event()?;
    }

    Ok(())
}

fn restore_terminal() {
    let _ = disable_raw_mode();
    let _ = execute!(io::stdout(), LeaveAlternateScreen);
}

/// Without this a panic leaves the shell in raw mode on the alternate screen.
fn install_panic_hook() {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore_terminal();
        previous(info);
    }));
}

#[cfg(test)]
mod render_tests {
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    use crate::app::{App, Panel, Screen};
    use crate::game::EventMessage;
    use crate::i18n::Lang;

    fn every_screen() -> Vec<Screen> {
        vec![
            Screen::MainMenu,
            Screen::ConfirmNewGame,
            Screen::HighScores,
            Screen::Market,
            Screen::BuyInput { good_index: 0 },
            Screen::SellInput { good_index: 0 },
            Screen::BankMenu,
            Screen::BankInput { deposit: true },
            Screen::Hospital,
            Screen::HospitalInput,
            Screen::RepayDebt,
            Screen::RepayInput,
            Screen::RentHouse,
            Screen::Cafe,
            Screen::EventPopup(EventMessage {
                title: "news".into(),
                body: "something happened".into(),
            }),
            Screen::NameInput,
            Screen::GameOver,
            Screen::Settings,
        ]
    }

    fn render(screen: Screen, lang: Lang, width: u16, height: u16) -> String {
        let mut app = App::new();
        app.lang = lang;
        app.game.start();
        app.game.current_location = Some(0);
        app.game.inventory[0] = 3;
        app.focus = Panel::Locations;
        app.input_max = 10;
        app.input_value = 1;
        app.hospital_max = 10;
        app.hospital_points = 1;
        app.screen = screen;

        let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
        terminal.draw(|f| crate::ui::draw(f, &app)).unwrap();
        terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect()
    }

    #[test]
    fn every_screen_draws_something_in_both_languages() {
        for lang in [Lang::Zh, Lang::En] {
            for screen in every_screen() {
                let out = render(screen, lang, 120, 40);
                assert!(!out.trim().is_empty());
            }
        }
    }

    /// A dragged-in window must not panic the layout on any screen.
    #[test]
    fn every_screen_survives_a_cramped_window() {
        for (width, height) in [(20, 5), (40, 10), (80, 24), (200, 60)] {
            for screen in every_screen() {
                render(screen, Lang::Zh, width, height);
            }
        }
    }

    /// A wide CJK glyph owns two cells and the trailing one reads back as a
    /// space, so compare with all whitespace squeezed out.
    fn squeezed(text: &str) -> String {
        text.chars().filter(|c| !c.is_whitespace()).collect()
    }

    #[test]
    fn the_main_menu_shows_the_title() {
        let zh = squeezed(&render(Screen::MainMenu, Lang::Zh, 120, 40));
        assert!(zh.contains(&squeezed("北京浮生记")));

        let en = squeezed(&render(Screen::MainMenu, Lang::En, 120, 40));
        assert!(en.contains(&squeezed("Beijing Drifter")));
    }

}



