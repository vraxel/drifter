use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::data;
use crate::game::{self, BuyBlock, EventMessage, GameState, HighScore};
use crate::i18n::{self, Lang};

fn step_cursor(cursor: &mut usize, len: usize, delta: isize) {
    if len == 0 {
        *cursor = 0;
        return;
    }
    *cursor = (*cursor as isize + delta).clamp(0, len as isize - 1) as usize;
}

/// The three areas the cursor can live in on the trading screen.
#[derive(Clone, Copy, PartialEq)]
pub enum Panel {
    Market,
    Inventory,
    Locations,
}

impl Panel {
    fn next(self) -> Self {
        match self {
            Panel::Market => Panel::Inventory,
            Panel::Inventory => Panel::Locations,
            Panel::Locations => Panel::Market,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum MenuItem {
    Continue,
    NewGame,
    HighScores,
    Settings,
    Language,
    Quit,
}

#[derive(Clone, PartialEq)]
pub enum Screen {
    MainMenu,
    ConfirmNewGame,
    HighScores,
    Market,
    BuyInput { good_index: usize },
    SellInput { good_index: usize },
    BankMenu,
    BankInput { deposit: bool },
    Hospital,
    HospitalInput,
    RepayDebt,
    RepayInput,
    RentHouse,
    Cafe,
    EventPopup(EventMessage),
    NameInput,
    GameOver,
    Settings,
}

pub struct App {
    pub lang: Lang,
    pub screen: Screen,
    pub game: GameState,
    pub running: bool,
    pub game_started: bool,

    pub high_scores: Vec<HighScore>,
    pub last_rank: Option<usize>,
    pub name_buffer: String,

    pub menu_cursor: usize,
    pub market_cursor: usize,
    pub inventory_cursor: usize,
    pub travel_cursor: usize,
    pub bank_cursor: usize,
    pub settings_cursor: usize,
    pub focus: Panel,
    pub settings_from_market: bool,

    pub input_value: i64,
    pub input_max: i64,

    pub hospital_points: i32,
    pub hospital_max: i32,

    pub message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            lang: Lang::Zh,
            screen: Screen::MainMenu,
            game: GameState::new(),
            running: true,
            game_started: false,
            high_scores: game::load_high_scores(),
            last_rank: None,
            name_buffer: String::new(),
            menu_cursor: 0,
            market_cursor: 0,
            inventory_cursor: 0,
            travel_cursor: 0,
            bank_cursor: 0,
            settings_cursor: 0,
            focus: Panel::Market,
            settings_from_market: false,
            input_value: 0,
            input_max: 0,
            hospital_points: 0,
            hospital_max: 0,
            message: None,
        }
    }

    pub fn texts(&self) -> &'static i18n::Texts {
        i18n::texts(self.lang)
    }

    pub fn menu_items(&self) -> Vec<MenuItem> {
        let mut items = Vec::new();
        if self.game_started && !self.game.game_ended {
            items.push(MenuItem::Continue);
        }
        items.push(MenuItem::NewGame);
        items.push(MenuItem::HighScores);
        items.push(MenuItem::Settings);
        items.push(MenuItem::Language);
        items.push(MenuItem::Quit);
        items
    }

    pub fn available_goods(&self) -> Vec<(usize, i64)> {
        self.game
            .prices
            .iter()
            .enumerate()
            .filter(|(_, &p)| p > 0)
            .map(|(i, &p)| (i, p))
            .collect()
    }

    pub fn inventory_goods(&self) -> Vec<(usize, i32)> {
        self.game
            .inventory
            .iter()
            .enumerate()
            .filter(|(_, &q)| q > 0)
            .map(|(i, &q)| (i, q))
            .collect()
    }

    fn start_new_game(&mut self) {
        let hacker = self.game.hacker_enabled;
        self.game = GameState::new();
        self.game.hacker_enabled = hacker;
        self.game.start();
        self.game_started = true;
        self.market_cursor = 0;
        self.inventory_cursor = 0;
        self.travel_cursor = 0;
        self.focus = Panel::Market;
        self.last_rank = None;
        self.screen = Screen::Market;
    }

    /// Shows queued news one popup at a time, then lands on the right screen.
    fn advance(&mut self) {
        if let Some(evt) = self.game.pop_event() {
            self.screen = Screen::EventPopup(evt);
        } else if self.game.game_ended {
            self.enter_game_over();
        } else {
            self.screen = Screen::Market;
        }
    }

    fn enter_game_over(&mut self) {
        self.last_rank = game::rank_for(&self.high_scores, self.game.score());
        if self.last_rank.is_some() {
            self.name_buffer.clear();
            self.screen = Screen::NameInput;
        } else {
            self.screen = Screen::GameOver;
        }
    }

    fn clamp_cursors(&mut self) {
        let market_len = self.available_goods().len();
        if market_len == 0 {
            self.market_cursor = 0;
        } else {
            self.market_cursor = self.market_cursor.min(market_len - 1);
        }

        let inv_len = self.inventory_goods().len();
        if inv_len == 0 {
            self.inventory_cursor = 0;
            if self.focus == Panel::Inventory {
                self.focus = Panel::Market;
            }
        } else {
            self.inventory_cursor = self.inventory_cursor.min(inv_len - 1);
        }
    }

    pub fn handle_event(&mut self) -> std::io::Result<()> {
        let Event::Key(key) = event::read()? else {
            return Ok(());
        };
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }
        self.message = None;

        match self.screen.clone() {
            Screen::MainMenu => self.handle_main_menu(key.code),
            Screen::ConfirmNewGame => self.handle_confirm_new_game(key.code),
            Screen::HighScores => self.handle_high_scores(key.code),
            Screen::Market => self.handle_market(key.code),
            Screen::BuyInput { good_index } => self.handle_buy_input(key.code, good_index),
            Screen::SellInput { good_index } => self.handle_sell_input(key.code, good_index),
            Screen::BankMenu => self.handle_bank_menu(key.code),
            Screen::BankInput { deposit } => self.handle_bank_input(key.code, deposit),
            Screen::Hospital => self.handle_hospital(key.code),
            Screen::HospitalInput => self.handle_hospital_input(key.code),
            Screen::RepayDebt => self.handle_repay(key.code),
            Screen::RepayInput => self.handle_repay_input(key.code),
            Screen::RentHouse => self.handle_rent(key.code),
            Screen::Cafe => self.handle_cafe(key.code),
            Screen::EventPopup(_) => self.advance(),
            Screen::NameInput => self.handle_name_input(key.code),
            Screen::GameOver => self.handle_game_over(key.code),
            Screen::Settings => self.handle_settings(key.code),
        }
        Ok(())
    }

    fn handle_main_menu(&mut self, key: KeyCode) {
        let items = self.menu_items();
        match key {
            KeyCode::Char('w') | KeyCode::Up => {
                self.menu_cursor = self.menu_cursor.checked_sub(1).unwrap_or(items.len() - 1);
            }
            KeyCode::Char('s') | KeyCode::Down => {
                self.menu_cursor = (self.menu_cursor + 1) % items.len();
            }
            KeyCode::Char('j') | KeyCode::Enter => match items[self.menu_cursor] {
                MenuItem::Continue => {
                    self.screen = Screen::Market;
                }
                MenuItem::NewGame => {
                    // The original only nags once you are three days in.
                    if self.game_started && !self.game.game_ended && self.game.days_elapsed() >= 3 {
                        self.screen = Screen::ConfirmNewGame;
                    } else {
                        self.start_new_game();
                    }
                }
                MenuItem::HighScores => {
                    self.screen = Screen::HighScores;
                }
                MenuItem::Settings => {
                    self.settings_cursor = 0;
                    self.settings_from_market = false;
                    self.screen = Screen::Settings;
                }
                MenuItem::Language => {
                    self.lang = self.lang.toggle();
                }
                MenuItem::Quit => {
                    self.running = false;
                }
            },
            KeyCode::Esc => self.running = false,
            _ => {}
        }
    }

    fn handle_confirm_new_game(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('j') | KeyCode::Enter => self.start_new_game(),
            KeyCode::Char('k') | KeyCode::Esc => self.screen = Screen::MainMenu,
            _ => {}
        }
    }

    fn handle_high_scores(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('k') | KeyCode::Esc | KeyCode::Enter | KeyCode::Char('j') => {
                self.screen = Screen::MainMenu;
            }
            _ => {}
        }
    }

    fn handle_market(&mut self, key: KeyCode) {
        self.clamp_cursors();

        match key {
            KeyCode::Tab => self.focus = self.focus.next(),
            KeyCode::Char('w') | KeyCode::Up => self.move_vertical(-1),
            KeyCode::Char('s') | KeyCode::Down => self.move_vertical(1),
            KeyCode::Char('a') | KeyCode::Left => self.move_horizontal(-1),
            KeyCode::Char('d') | KeyCode::Right => self.move_horizontal(1),

            // J acts on whatever panel holds the cursor.
            KeyCode::Char('j') | KeyCode::Enter => match self.focus {
                Panel::Market => self.open_buy(),
                Panel::Inventory => self.open_sell(),
                Panel::Locations => self.depart(),
            },
            KeyCode::Char('q') => self.open_buy(),
            KeyCode::Char('e') => self.open_sell(),
            KeyCode::Char('t') => self.focus = Panel::Locations,

            KeyCode::Char('b') => {
                self.bank_cursor = 0;
                self.screen = Screen::BankMenu;
            }
            KeyCode::Char('h') => self.screen = Screen::Hospital,
            KeyCode::Char('p') => self.screen = Screen::RepayDebt,
            KeyCode::Char('r') => self.screen = Screen::RentHouse,
            KeyCode::Char('n') => self.screen = Screen::Cafe,
            KeyCode::Char('g') => self.game.subway_mode = !self.game.subway_mode,
            KeyCode::Char('l') => self.lang = self.lang.toggle(),
            KeyCode::Char('o') => {
                self.settings_cursor = 0;
                self.settings_from_market = true;
                self.screen = Screen::Settings;
            }
            KeyCode::Esc => {
                self.menu_cursor = 0;
                self.screen = Screen::MainMenu;
            }
            _ => {}
        }
    }

    fn move_vertical(&mut self, delta: isize) {
        let market_len = self.available_goods().len();
        let bag_len = self.inventory_goods().len();
        let grid_cols = data::location_grid_cols(self.lang);

        match self.focus {
            Panel::Market => {
                if delta > 0 && (market_len == 0 || self.market_cursor + 1 >= market_len) {
                    self.focus = Panel::Locations;
                } else if delta < 0 && self.market_cursor == 0 {
                    // already at top, nowhere to go
                } else {
                    step_cursor(&mut self.market_cursor, market_len, delta);
                }
            }
            Panel::Inventory => {
                if delta > 0 && (bag_len == 0 || self.inventory_cursor + 1 >= bag_len) {
                    self.focus = Panel::Locations;
                } else if delta < 0 && self.inventory_cursor == 0 {
                    // already at top
                } else {
                    step_cursor(&mut self.inventory_cursor, bag_len, delta);
                }
            }
            Panel::Locations => {
                let old = self.travel_cursor;
                step_cursor(
                    &mut self.travel_cursor,
                    data::LOCATION_COUNT,
                    delta * grid_cols as isize,
                );
                if delta < 0 && self.travel_cursor == old && old < grid_cols {
                    self.focus = Panel::Market;
                }
            }
        }
    }

    fn move_horizontal(&mut self, delta: isize) {
        match self.focus {
            Panel::Market => {
                if delta > 0 && !self.inventory_goods().is_empty() {
                    self.focus = Panel::Inventory;
                }
            }
            Panel::Inventory => {
                if delta < 0 {
                    self.focus = Panel::Market;
                }
            }
            Panel::Locations => {
                step_cursor(&mut self.travel_cursor, data::LOCATION_COUNT, delta);
            }
        }
    }

    fn open_buy(&mut self) {
        let available = self.available_goods();
        let Some(&(good_index, _)) = available.get(self.market_cursor) else {
            return;
        };
        match self.game.buy_block(good_index) {
            BuyBlock::None => {
                self.input_value = 1;
                self.input_max = self.game.max_can_buy(good_index) as i64;
                self.screen = Screen::BuyInput { good_index };
            }
            BuyBlock::NoSpace => self.message = Some(self.texts().not_enough_space.to_string()),
            BuyBlock::NoCash => self.message = Some(self.texts().not_enough_cash.to_string()),
        }
    }

    fn open_sell(&mut self) {
        let inventory = self.inventory_goods();
        let Some(&(good_index, qty)) = inventory.get(self.inventory_cursor) else {
            return;
        };
        if self.game.prices[good_index] == 0 {
            return;
        }
        self.input_value = 1;
        self.input_max = qty as i64;
        self.screen = Screen::SellInput { good_index };
    }

    fn depart(&mut self) {
        if self.game.current_location == Some(self.travel_cursor) {
            self.message = Some(self.texts().already_here.to_string());
            return;
        }
        self.game.travel_to(self.travel_cursor, self.lang);
        self.market_cursor = 0;
        self.inventory_cursor = 0;
        self.focus = Panel::Market;
        self.advance();
    }

    fn step_input(&mut self, key: KeyCode, small: i64, large: i64) -> Option<bool> {
        match key {
            KeyCode::Char('d') | KeyCode::Right => {
                self.input_value = (self.input_value + small).min(self.input_max);
                None
            }
            KeyCode::Char('a') | KeyCode::Left => {
                self.input_value = (self.input_value - small).max(1);
                None
            }
            KeyCode::Char('w') | KeyCode::Up => {
                self.input_value = (self.input_value + large).min(self.input_max);
                None
            }
            KeyCode::Char('s') | KeyCode::Down => {
                self.input_value = (self.input_value - large).max(1);
                None
            }
            KeyCode::Char('j') | KeyCode::Enter => Some(true),
            KeyCode::Char('k') | KeyCode::Esc => Some(false),
            _ => None,
        }
    }

    fn handle_buy_input(&mut self, key: KeyCode, good_index: usize) {
        match self.step_input(key, 1, 10) {
            Some(true) => {
                self.game.buy(good_index, self.input_value as i32);
                self.screen = Screen::Market;
                self.clamp_cursors();
            }
            Some(false) => self.screen = Screen::Market,
            None => {}
        }
    }

    fn handle_sell_input(&mut self, key: KeyCode, good_index: usize) {
        match self.step_input(key, 1, 10) {
            Some(true) => {
                self.game.sell(good_index, self.input_value as i32, self.lang);
                self.screen = Screen::Market;
                self.clamp_cursors();
                self.advance();
            }
            Some(false) => self.screen = Screen::Market,
            None => {}
        }
    }

    fn handle_bank_menu(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('w') | KeyCode::Up => {
                self.bank_cursor = self.bank_cursor.saturating_sub(1);
            }
            KeyCode::Char('s') | KeyCode::Down => {
                self.bank_cursor = (self.bank_cursor + 1).min(1);
            }
            KeyCode::Char('j') | KeyCode::Enter => {
                let deposit = self.bank_cursor == 0;
                let max = if deposit { self.game.cash } else { self.game.bank };
                if max > 0 {
                    self.input_value = 1;
                    self.input_max = max;
                    self.screen = Screen::BankInput { deposit };
                }
            }
            KeyCode::Char('k') | KeyCode::Esc => self.screen = Screen::Market,
            _ => {}
        }
    }

    fn handle_bank_input(&mut self, key: KeyCode, deposit: bool) {
        match self.step_input(key, 100, 1000) {
            Some(true) => {
                if deposit {
                    self.game.bank_deposit(self.input_value);
                } else {
                    self.game.bank_withdraw(self.input_value);
                }
                self.screen = Screen::Market;
            }
            Some(false) => self.screen = Screen::BankMenu,
            None => {}
        }
    }

    fn handle_hospital(&mut self, key: KeyCode) {
        if self.game.health >= 100 {
            if matches!(
                key,
                KeyCode::Char('j') | KeyCode::Char('k') | KeyCode::Enter | KeyCode::Esc
            ) {
                self.screen = Screen::Market;
            }
            return;
        }
        match key {
            KeyCode::Char('j') | KeyCode::Enter => {
                self.hospital_max = 100 - self.game.health;
                self.hospital_points = 1;
                self.screen = Screen::HospitalInput;
            }
            KeyCode::Char('k') | KeyCode::Esc => self.screen = Screen::Market,
            _ => {}
        }
    }

    fn handle_hospital_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('d') | KeyCode::Right => {
                self.hospital_points = (self.hospital_points + 1).min(self.hospital_max);
            }
            KeyCode::Char('a') | KeyCode::Left => {
                self.hospital_points = (self.hospital_points - 1).max(1);
            }
            KeyCode::Char('w') | KeyCode::Up => {
                self.hospital_points = (self.hospital_points + 5).min(self.hospital_max);
            }
            KeyCode::Char('s') | KeyCode::Down => {
                self.hospital_points = (self.hospital_points - 5).max(1);
            }
            KeyCode::Char('j') | KeyCode::Enter => {
                if !self.game.visit_hospital(self.hospital_points) {
                    self.message = Some(self.texts().hospital_no_money.to_string());
                }
                self.screen = Screen::Market;
            }
            KeyCode::Char('k') | KeyCode::Esc => self.screen = Screen::Hospital,
            _ => {}
        }
    }

    fn handle_repay(&mut self, key: KeyCode) {
        if self.game.debt == 0 {
            if matches!(
                key,
                KeyCode::Char('j') | KeyCode::Char('k') | KeyCode::Enter | KeyCode::Esc
            ) {
                self.screen = Screen::Market;
            }
            return;
        }
        match key {
            KeyCode::Char('j') | KeyCode::Enter => {
                let max = self.game.cash.min(self.game.debt);
                if max > 0 {
                    self.input_value = 1;
                    self.input_max = max;
                    self.screen = Screen::RepayInput;
                } else {
                    self.message = Some(self.texts().debt_not_enough.to_string());
                }
            }
            KeyCode::Char('k') | KeyCode::Esc => self.screen = Screen::Market,
            _ => {}
        }
    }

    fn handle_repay_input(&mut self, key: KeyCode) {
        match self.step_input(key, 100, 1000) {
            Some(true) => {
                if !self.game.repay_debt(self.input_value) {
                    self.message = Some(self.texts().debt_not_enough.to_string());
                }
                self.screen = Screen::Market;
            }
            Some(false) => self.screen = Screen::RepayDebt,
            None => {}
        }
    }

    fn handle_rent(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('j') | KeyCode::Enter => {
                self.message = Some(match self.game.rent_house(self.lang) {
                    Ok(msg) => msg.to_string(),
                    Err(msg) => msg.to_string(),
                });
                self.screen = Screen::Market;
            }
            KeyCode::Char('k') | KeyCode::Esc => self.screen = Screen::Market,
            _ => {}
        }
    }

    fn handle_cafe(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('j') | KeyCode::Enter => {
                self.message = Some(match self.game.visit_cafe(self.lang) {
                    Ok(msg) => msg,
                    Err(msg) => msg.to_string(),
                });
                self.screen = Screen::Market;
            }
            KeyCode::Char('k') | KeyCode::Esc => self.screen = Screen::Market,
            _ => {}
        }
    }

    fn handle_name_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                if self.name_buffer.chars().count() < 16 {
                    self.name_buffer.push(c);
                }
            }
            KeyCode::Backspace => {
                self.name_buffer.pop();
            }
            KeyCode::Enter => {
                let name = if self.name_buffer.trim().is_empty() {
                    self.texts().default_name.to_string()
                } else {
                    self.name_buffer.trim().to_string()
                };
                game::insert_high_score(
                    &mut self.high_scores,
                    HighScore {
                        name,
                        score: self.game.score(),
                        health: self.game.health,
                        fame: self.game.fame,
                    },
                );
                game::save_high_scores(&self.high_scores);
                self.screen = Screen::GameOver;
            }
            KeyCode::Esc => self.screen = Screen::GameOver,
            _ => {}
        }
    }

    fn handle_game_over(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('j') | KeyCode::Enter => self.start_new_game(),
            KeyCode::Char('k') | KeyCode::Esc => {
                self.menu_cursor = 0;
                self.screen = Screen::MainMenu;
            }
            _ => {}
        }
    }

    fn handle_settings(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('w') | KeyCode::Up | KeyCode::Char('s') | KeyCode::Down => {
                self.settings_cursor = 0;
            }
            KeyCode::Char('j') | KeyCode::Enter => {
                self.game.hacker_enabled = !self.game.hacker_enabled;
            }
            KeyCode::Char('k') | KeyCode::Esc => {
                self.screen = if self.settings_from_market {
                    Screen::Market
                } else {
                    Screen::MainMenu
                };
            }
            _ => {}
        }
    }
}
