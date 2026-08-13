use std::fs;
use std::path::PathBuf;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::data::*;
use crate::i18n::{texts, Lang};

#[derive(Clone, PartialEq)]
pub struct EventMessage {
    pub title: String,
    pub body: String,
}

#[derive(Clone, PartialEq)]
pub struct HighScore {
    pub name: String,
    pub score: i64,
    pub health: i32,
    pub fame: i32,
}

pub enum BuyBlock {
    None,
    NoCash,
    NoSpace,
}

pub struct GameState {
    pub cash: i64,
    pub debt: i64,
    pub bank: i64,
    pub health: i32,
    pub fame: i32,
    pub days_left: i32,
    pub capacity: i32,
    pub current_location: Option<usize>,
    pub subway_mode: bool,
    pub cafe_visits: i32,
    pub hacker_enabled: bool,

    pub prices: [i64; GOOD_COUNT],
    pub inventory: [i32; GOOD_COUNT],
    /// Running cost basis per good, so the bag can show what you paid.
    inventory_cost: [i64; GOOD_COUNT],

    pub pending_events: Vec<EventMessage>,

    /// Original shows the reputation warning only on the first sale of each kind.
    fame_warned_book: bool,
    fame_warned_liquor: bool,

    rng: StdRng,

    pub dead: bool,
    pub game_ended: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self::with_seed(None)
    }

    pub fn with_seed(seed: Option<u64>) -> Self {
        Self {
            cash: INITIAL_CASH,
            debt: INITIAL_DEBT,
            bank: INITIAL_BANK,
            health: INITIAL_HEALTH,
            fame: INITIAL_FAME,
            days_left: TOTAL_DAYS,
            capacity: INITIAL_CAPACITY,
            current_location: None,
            subway_mode: true,
            cafe_visits: 0,
            hacker_enabled: false,
            prices: [0; GOOD_COUNT],
            inventory: [0; GOOD_COUNT],
            inventory_cost: [0; GOOD_COUNT],
            pending_events: Vec::new(),
            fame_warned_book: false,
            fame_warned_liquor: false,
            rng: match seed {
                Some(s) => StdRng::seed_from_u64(s),
                None => StdRng::from_entropy(),
            },
            dead: false,
            game_ended: false,
        }
    }

    /// Day 0: prices and one interest tick are applied before the first trip,
    /// so the opening "buy before you travel" play works as in the original.
    pub fn start(&mut self) {
        self.make_prices(3);
        self.apply_interest();
    }

    pub fn days_elapsed(&self) -> i32 {
        TOTAL_DAYS - self.days_left
    }

    pub fn total_goods(&self) -> i32 {
        self.inventory.iter().sum()
    }

    pub fn free_space(&self) -> i32 {
        self.capacity - self.total_goods()
    }

    pub fn score(&self) -> i64 {
        self.cash + self.bank - self.debt
    }

    fn push(&mut self, title: &str, body: String) {
        self.pending_events.push(EventMessage {
            title: title.to_string(),
            body,
        });
    }

    fn make_prices(&mut self, leaveout: usize) {
        for (i, def) in GOODS.iter().enumerate() {
            self.prices[i] = def.base_price + self.rng.gen_range(0..def.price_range);
        }
        for _ in 0..leaveout {
            let j = self.rng.gen_range(0..GOOD_COUNT);
            self.prices[j] = 0;
        }
    }

    fn apply_interest(&mut self) {
        self.debt += (self.debt as f64 * DEBT_INTEREST_RATE) as i64;
        self.bank += (self.bank as f64 * BANK_INTEREST_RATE) as i64;
    }

    // ---------------------------------------------------------------- turn

    pub fn travel_to(&mut self, location: usize, lang: Lang) {
        if self.current_location == Some(location) {
            return;
        }
        self.current_location = Some(location);
        self.run_turn(lang);
    }

    /// Phase order is taken verbatim from HandleNormalEvents / DoRandomEvent:
    /// prices, interest, market news, health (may end the run), theft, debt
    /// punishment, then the day ticks over.
    fn run_turn(&mut self, lang: Lang) {
        let leaveout = if self.days_left <= 2 { 0 } else { 3 };
        self.make_prices(leaveout);
        self.apply_interest();
        self.do_market_events(lang);

        self.do_health_phase(lang);
        if self.dead {
            self.game_ended = true;
            return;
        }

        self.do_steal_phase(lang);
        self.check_debt_punishment(lang);

        self.days_left -= 1;

        let t = texts(lang);
        if self.days_left == 1 {
            self.push(t.diary, t.last_day_warning.to_string());
        }
        if self.days_left <= 0 {
            self.force_sell_remaining(lang);
            self.game_ended = true;
        }
    }

    /// Every entry rolls independently - the original loop has no `break`, so a
    /// single day can carry several headlines. A full bag aborts the rest.
    fn do_market_events(&mut self, lang: Lang) {
        let t = texts(lang);

        for evt in MARKET_EVENTS.iter() {
            if self.rng.gen_range(0..950) % evt.freq as i64 != 0 {
                continue;
            }
            if self.prices[evt.good_index] == 0 {
                continue;
            }

            let msg = match lang {
                Lang::Zh => evt.msg_zh,
                Lang::En => evt.msg_en,
            };
            self.push(t.news, msg.to_string());

            if evt.extra_debt > 0 {
                self.debt += evt.extra_debt;
            }
            if evt.multiply > 0 {
                self.prices[evt.good_index] *= evt.multiply;
            }
            if evt.divide > 0 {
                self.prices[evt.good_index] /= evt.divide;
            }

            if evt.add_items > 0 {
                let addable = evt.add_items.min(self.free_space());
                if addable <= 0 {
                    let capacity = self.capacity;
                    let unit = match lang {
                        Lang::Zh => format!("{}{}个物品。", t.bag_too_small, capacity),
                        Lang::En => format!("{}{} items.", t.bag_too_small, capacity),
                    };
                    self.push(t.diary, unit);
                    return;
                }
                self.inventory[evt.good_index] += addable;
            }
        }
    }

    /// Health events, then the forced hospital stay. A hospital stay returns
    /// early, which is why the original never kills you on the same day it
    /// carts you off.
    fn do_health_phase(&mut self, lang: Lang) {
        let t = texts(lang);

        for evt in HEALTH_EVENTS.iter() {
            if self.rng.gen_range(0..1000) % evt.freq as i64 != 0 {
                continue;
            }
            let msg = match lang {
                Lang::Zh => evt.msg_zh,
                Lang::En => evt.msg_en,
            };
            let body = match lang {
                Lang::Zh => format!("{} 俺的健康减少了{}点。", msg, evt.damage),
                Lang::En => format!("{} Health -{}", msg, evt.damage),
            };
            self.push(t.diary, body);
            self.health -= evt.damage;
            break;
        }

        if self.health < AUTO_HOSPITAL_HEALTH_THRESHOLD && self.days_left > AUTO_HOSPITAL_MIN_DAYS_LEFT {
            self.forced_hospital_stay(lang);
            return;
        }

        if self.health < 20 && self.health > 0 {
            self.push(t.diary, t.health_critical.to_string());
            return;
        }

        if self.health < 0 {
            self.push(t.diary, t.death.to_string());
            self.dead = true;
        }
    }

    fn forced_hospital_stay(&mut self, lang: Lang) {
        let t = texts(lang);

        let delay_days = 1 + self.rng.gen_range(0..2);
        let cost = delay_days as i64 * (1000 + self.rng.gen_range(0..8500));

        let base = if self.subway_mode { 0 } else { LOCATION_COUNT };
        let index = base + self.current_location.unwrap_or(0);
        let where_at = AUTO_HOSPITAL_LOCATIONS
            .get(index)
            .map(|l| match lang {
                Lang::Zh => l.zh,
                Lang::En => l.en,
            })
            .unwrap_or(match lang {
                Lang::Zh => "街头",
                Lang::En => "the street",
            });

        let spot_index = self.rng.gen_range(0..COLLAPSE_PLACES_ZH.len());
        let spot = match lang {
            Lang::Zh => COLLAPSE_PLACES_ZH[spot_index],
            Lang::En => COLLAPSE_PLACES_EN[spot_index],
        };

        let body = match lang {
            Lang::Zh => format!(
                "由于不注意身体,我被人发现昏迷在{}附近的{}。好心的市民把我抬到医院，医生让我治疗{}天。村长让人为我垫付了住院费用{}元。",
                where_at, spot, delay_days, cost
            ),
            Lang::En => format!(
                "Found unconscious in {} near {}. Kind citizens carried me to the hospital; the doctor kept me {} days. The village chief covered the {} yuan bill.",
                spot, where_at, delay_days, cost
            ),
        };
        self.push(t.diary, body);

        self.debt += cost;
        self.health = (self.health + AUTO_HOSPITAL_HEAL).min(100);
        self.days_left -= delay_days;
    }

    /// Theft, the optional hacker, then the negative-cash clamp.
    fn do_steal_phase(&mut self, lang: Lang) {
        let t = texts(lang);

        for evt in STEAL_EVENTS.iter() {
            if self.rng.gen_range(0..1000) % evt.freq as i64 != 0 {
                continue;
            }
            let msg = match lang {
                Lang::Zh => evt.msg_zh,
                Lang::En => evt.msg_en,
            };

            if evt.targets_bank {
                if self.bank > 0 {
                    self.bank = apply_percent_loss(self.bank, evt.loss_percent);
                    let body = match lang {
                        Lang::Zh => format!("{} 俺的存款减少了{}%，哎呀!", msg, evt.loss_percent),
                        Lang::En => format!("{} Savings -{}%", msg, evt.loss_percent),
                    };
                    self.push(t.diary, body);
                }
            } else {
                self.cash = apply_percent_loss(self.cash, evt.loss_percent);
                let body = match lang {
                    Lang::Zh => format!("{} 俺的银子减少了{}%。", msg, evt.loss_percent),
                    Lang::En => format!("{} Cash -{}%", msg, evt.loss_percent),
                };
                self.push(t.diary, body);
            }
            break;
        }

        self.do_hacker_event(lang);

        if self.cash < 0 {
            self.cash = 0;
            self.push(t.diary, t.cash_zero.to_string());
        }
    }

    fn do_hacker_event(&mut self, lang: Lang) {
        if !self.hacker_enabled {
            return;
        }
        if self.rng.gen_range(0..1000) % 25 != 0 {
            return;
        }
        if self.bank < 1000 {
            return;
        }

        let t = texts(lang);
        if self.bank > 100000 {
            let num = self.bank / (2 + self.rng.gen_range(0..20));
            if self.rng.gen_range(0..20) % 3 != 0 {
                self.bank -= num;
                self.push(t.diary, format!("{}{}", t.hacker_decrease, num));
            } else {
                self.bank += num;
                self.push(t.diary, format!("{}{}", t.hacker_increase, num));
            }
        } else {
            let num = self.bank / (1 + self.rng.gen_range(0..15));
            self.bank += num;
            self.push(t.diary, format!("{}{}", t.hacker_increase, num));
        }
    }

    fn check_debt_punishment(&mut self, lang: Lang) {
        if self.debt <= DEBT_PUNISHMENT_THRESHOLD {
            return;
        }
        let t = texts(lang);
        self.push(t.diary, t.debt_punishment.to_string());
        self.health -= DEBT_PUNISHMENT_DAMAGE;
    }

    fn force_sell_remaining(&mut self, lang: Lang) {
        let t = texts(lang);
        self.push(t.diary, t.going_home.to_string());

        let mut sold = false;
        for i in 0..GOOD_COUNT {
            if self.inventory[i] > 0 && self.prices[i] > 0 {
                self.cash += self.inventory[i] as i64 * self.prices[i];
                self.inventory[i] = 0;
                self.inventory_cost[i] = 0;
                sold = true;
            }
        }
        if sold {
            self.push(t.diary, t.sell_remaining.to_string());
        }
    }

    // ------------------------------------------------------------- trading

    pub fn max_can_buy(&self, good_index: usize) -> i32 {
        if self.prices[good_index] == 0 {
            return 0;
        }
        let by_cash = (self.cash / self.prices[good_index]) as i32;
        by_cash.min(self.free_space()).max(0)
    }

    pub fn buy_block(&self, good_index: usize) -> BuyBlock {
        if self.free_space() <= 0 {
            BuyBlock::NoSpace
        } else if self.max_can_buy(good_index) <= 0 {
            BuyBlock::NoCash
        } else {
            BuyBlock::None
        }
    }

    pub fn buy(&mut self, good_index: usize, qty: i32) -> bool {
        if qty <= 0 || self.prices[good_index] == 0 {
            return false;
        }
        let cost = qty as i64 * self.prices[good_index];
        if cost > self.cash || qty > self.free_space() {
            return false;
        }
        self.cash -= cost;
        self.inventory[good_index] += qty;
        self.inventory_cost[good_index] += cost;
        true
    }

    /// Weighted average paid per unit; `None` once the position is closed.
    /// Goods handed over by an event cost nothing and drag this down.
    pub fn avg_cost(&self, good_index: usize) -> Option<i64> {
        let held = self.inventory[good_index];
        if held <= 0 {
            return None;
        }
        Some(self.inventory_cost[good_index] / held as i64)
    }

    /// Reputation is spent on the way out, not the way in.
    pub fn sell(&mut self, good_index: usize, qty: i32, lang: Lang) -> bool {
        if qty <= 0 || qty > self.inventory[good_index] || self.prices[good_index] == 0 {
            return false;
        }
        self.cash += qty as i64 * self.prices[good_index];
        let held_before = self.inventory[good_index];
        self.inventory[good_index] -= qty;
        self.inventory_cost[good_index] =
            self.inventory_cost[good_index] * self.inventory[good_index] as i64
                / held_before as i64;

        let penalty = GOODS[good_index].fame_penalty;
        if penalty > 0 {
            let t = texts(lang);
            let first_time = match good_index {
                3 if !self.fame_warned_liquor => {
                    self.fame_warned_liquor = true;
                    Some(t.fame_msg_liquor)
                }
                4 if !self.fame_warned_book => {
                    self.fame_warned_book = true;
                    Some(t.fame_msg_book)
                }
                _ => None,
            };
            if let Some(msg) = first_time {
                self.push(t.diary, msg.to_string());
            }
            self.fame = (self.fame - penalty).max(0);
        }
        true
    }

    // ---------------------------------------------------------- facilities

    pub fn bank_deposit(&mut self, amount: i64) -> bool {
        if amount <= 0 || amount > self.cash {
            return false;
        }
        self.cash -= amount;
        self.bank += amount;
        true
    }

    pub fn bank_withdraw(&mut self, amount: i64) -> bool {
        if amount <= 0 || amount > self.bank {
            return false;
        }
        self.bank -= amount;
        self.cash += amount;
        true
    }

    pub fn repay_debt(&mut self, amount: i64) -> bool {
        if amount <= 0 || amount > self.cash || self.debt == 0 {
            return false;
        }
        let actual = amount.min(self.debt);
        self.cash -= actual;
        self.debt -= actual;
        true
    }

    pub fn visit_hospital(&mut self, points: i32) -> bool {
        if self.health >= 100 || points <= 0 {
            return false;
        }
        let actual = points.min(100 - self.health);
        let cost = actual as i64 * HOSPITAL_COST_PER_POINT;
        if cost > self.cash {
            return false;
        }
        self.cash -= cost;
        self.health += actual;
        true
    }

    pub fn can_rent_house(&self) -> bool {
        self.capacity < MAX_CAPACITY && self.cash >= HOUSE_MIN_CASH
    }

    pub fn rent_house(&mut self, lang: Lang) -> Result<&'static str, &'static str> {
        let t = texts(lang);
        if self.capacity >= MAX_CAPACITY {
            return Err(t.house_too_large);
        }
        if self.cash < HOUSE_MIN_CASH {
            return Err(t.house_no_money);
        }
        if self.cash <= 30000 {
            self.cash -= 25000;
        } else {
            self.cash = self.cash / 2 - 2000;
        }
        self.capacity += 10;
        Ok(t.house_expanded)
    }

    pub fn can_visit_cafe(&self) -> bool {
        self.cafe_visits < MAX_CAFE_VISITS && self.cash >= CAFE_COST
    }

    /// The 15 yuan is a doorman check, never actually deducted.
    pub fn visit_cafe(&mut self, lang: Lang) -> Result<String, &'static str> {
        let t = texts(lang);
        if self.cafe_visits >= MAX_CAFE_VISITS {
            return Err(t.cafe_too_many);
        }
        if self.cash < CAFE_COST {
            return Err(t.cafe_no_money);
        }
        self.cafe_visits += 1;
        let reward = 1 + self.rng.gen_range(0..10) as i64;
        self.cash += reward;
        Ok(format!("{}{}", t.cafe_reward, reward))
    }

    pub fn pop_event(&mut self) -> Option<EventMessage> {
        if self.pending_events.is_empty() {
            None
        } else {
            Some(self.pending_events.remove(0))
        }
    }
}

/// The truncating division runs before the scaling, so the remainder under 100
/// is lost too - that is where the original's integer maths ends up.
fn apply_percent_loss(amount: i64, percent: i64) -> i64 {
    amount / 100 * (100 - percent)
}

// ------------------------------------------------------------- high scores

fn scores_path() -> Option<PathBuf> {
    std::env::var("HOME")
        .ok()
        .map(|home| PathBuf::from(home).join(".drifter").join("scores"))
}

pub fn load_high_scores() -> Vec<HighScore> {
    let Some(path) = scores_path() else {
        return Vec::new();
    };
    let Ok(raw) = fs::read_to_string(&path) else {
        return Vec::new();
    };

    let mut out: Vec<HighScore> = raw
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('\t');
            let name = parts.next()?.to_string();
            let score = parts.next()?.parse().ok()?;
            let health = parts.next()?.parse().ok()?;
            let fame = parts.next()?.parse().ok()?;
            Some(HighScore { name, score, health, fame })
        })
        .collect();

    out.sort_by_key(|entry| std::cmp::Reverse(entry.score));
    out.truncate(10);
    out
}

pub fn save_high_scores(scores: &[HighScore]) {
    let Some(path) = scores_path() else {
        return;
    };
    if let Some(dir) = path.parent() {
        if fs::create_dir_all(dir).is_err() {
            return;
        }
    }
    let body: String = scores
        .iter()
        .map(|s| format!("{}\t{}\t{}\t{}\n", s.name, s.score, s.health, s.fame))
        .collect();
    let _ = fs::write(&path, body);
}

/// `None` means the run did not place; the original only ranks positive scores.
pub fn rank_for(scores: &[HighScore], score: i64) -> Option<usize> {
    if score <= 0 {
        return None;
    }
    let ahead = scores.iter().filter(|s| s.score >= score).count();
    if ahead < 10 {
        Some(ahead + 1)
    } else {
        None
    }
}

pub fn insert_high_score(scores: &mut Vec<HighScore>, entry: HighScore) {
    scores.push(entry);
    scores.sort_by_key(|entry| std::cmp::Reverse(entry.score));
    scores.truncate(10);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board(scores: &[i64]) -> Vec<HighScore> {
        scores
            .iter()
            .map(|&score| HighScore {
                name: "x".into(),
                score,
                health: 100,
                fame: 100,
            })
            .collect()
    }

    #[test]
    fn day_zero_charges_interest_before_the_first_trip() {
        let mut g = GameState::new();
        g.start();
        assert_eq!(g.debt, 5500);
        assert!(g.prices.iter().any(|&p| p > 0));
    }

    #[test]
    fn prices_stay_inside_the_original_bands() {
        let mut g = GameState::new();
        for _ in 0..200 {
            g.make_prices(0);
            for (price, def) in g.prices.iter().zip(GOODS.iter()) {
                assert!(*price >= def.base_price);
                assert!(*price < def.base_price + def.price_range);
            }
        }
    }

    #[test]
    fn reputation_is_charged_on_the_sale_not_the_purchase() {
        let mut g = GameState::new();
        g.cash = 100_000;
        g.prices[4] = 6000;
        g.buy(4, 2);
        assert_eq!(g.fame, 100);
        g.sell(4, 1, Lang::Zh);
        assert_eq!(g.fame, 93);
    }

    #[test]
    fn liquor_costs_more_reputation_than_the_banned_book() {
        let mut g = GameState::new();
        g.cash = 100_000;
        g.prices[3] = 2000;
        g.buy(3, 1);
        g.sell(3, 1, Lang::Zh);
        assert_eq!(g.fame, 90);
    }

    #[test]
    fn the_reputation_warning_only_fires_once_per_kind() {
        let mut g = GameState::new();
        g.cash = 100_000;
        g.prices[3] = 2000;
        g.buy(3, 2);

        g.sell(3, 1, Lang::Zh);
        assert_eq!(g.pending_events.len(), 1);
        g.pending_events.clear();

        g.sell(3, 1, Lang::Zh);
        assert!(g.pending_events.is_empty());
    }

    #[test]
    fn theft_truncates_before_it_scales() {
        assert_eq!(apply_percent_loss(199, 10), 90);
        assert_eq!(apply_percent_loss(1000, 40), 600);
        assert_eq!(apply_percent_loss(99, 5), 0);
    }

    #[test]
    fn death_requires_negative_health() {
        for _ in 0..500 {
            let mut g = GameState::new();
            g.health = 0;
            g.days_left = 2;
            g.do_health_phase(Lang::Zh);
            assert_eq!(g.dead, g.health < 0);
        }
    }

    #[test]
    fn the_debt_beating_never_kills_on_the_same_day() {
        let mut g = GameState::new();
        g.debt = 200_000;
        g.health = 10;
        g.check_debt_punishment(Lang::Zh);
        assert!(g.health < 0);
        assert!(!g.dead);
    }

    #[test]
    fn the_last_day_liquidates_the_bag() {
        let mut g = GameState::new();
        g.cash = 0;
        g.prices[0] = 200;
        g.inventory[0] = 5;
        g.force_sell_remaining(Lang::Zh);
        assert_eq!(g.cash, 1000);
        assert_eq!(g.inventory[0], 0);
    }

    #[test]
    fn the_cafe_checks_for_cash_but_never_takes_it() {
        let mut g = GameState::new();
        g.cash = 1000;
        g.visit_cafe(Lang::Zh).unwrap();
        assert!((1001..=1010).contains(&g.cash));

        g.cash = CAFE_COST - 1;
        assert!(g.visit_cafe(Lang::Zh).is_err());
    }

    #[test]
    fn the_cafe_turns_you_away_after_three_visits() {
        let mut g = GameState::new();
        g.cash = 1000;
        for _ in 0..MAX_CAFE_VISITS {
            assert!(g.visit_cafe(Lang::Zh).is_ok());
        }
        assert!(g.visit_cafe(Lang::Zh).is_err());
    }

    #[test]
    fn a_full_bag_reports_space_rather_than_money() {
        let mut g = GameState::new();
        g.cash = 1_000_000;
        g.prices[0] = 100;
        g.inventory[0] = g.capacity;
        assert!(matches!(g.buy_block(0), BuyBlock::NoSpace));

        g.inventory[0] = 0;
        g.cash = 10;
        assert!(matches!(g.buy_block(0), BuyBlock::NoCash));
    }

    #[test]
    fn ranking_ignores_runs_that_did_not_turn_a_profit() {
        assert_eq!(rank_for(&[], 0), None);
        assert_eq!(rank_for(&[], -5), None);
        assert_eq!(rank_for(&[], 100), Some(1));
    }

    #[test]
    fn ranking_places_you_behind_equal_or_better_runs() {
        let existing = board(&[500, 300, 100]);
        assert_eq!(rank_for(&existing, 600), Some(1));
        assert_eq!(rank_for(&existing, 400), Some(2));
        assert_eq!(rank_for(&existing, 500), Some(2));
        assert_eq!(rank_for(&existing, 50), Some(4));
    }

    #[test]
    fn a_full_board_turns_away_a_low_score() {
        let existing = board(&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
        assert_eq!(rank_for(&existing, 1), None);
        assert_eq!(rank_for(&existing, 100), Some(1));
    }

    #[test]
    fn the_board_keeps_only_the_top_ten() {
        let mut existing = board(&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
        insert_high_score(
            &mut existing,
            HighScore {
                name: "new".into(),
                score: 100,
                health: 50,
                fame: 60,
            },
        );
        assert_eq!(existing.len(), 10);
        assert_eq!(existing[0].score, 100);
        assert_eq!(existing.last().unwrap().score, 2);
    }
}
