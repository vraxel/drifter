use std::io::{self, BufRead, Write};

use serde_json::{json, Value};

use crate::data::{self, GOODS, GOOD_COUNT, LOCATION_COUNT};
use crate::game::GameState;
use crate::i18n::Lang;

pub fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut server = Server { game: None, lang: Lang::En };

    for line in stdin.lock().lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        let Ok(req) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        if let Some(resp) = server.handle(&req) {
            let mut out = stdout.lock();
            serde_json::to_writer(&mut out, &resp)?;
            out.write_all(b"\n")?;
            out.flush()?;
        }
    }
    Ok(())
}

struct Server {
    game: Option<GameState>,
    lang: Lang,
}

impl Server {
    fn handle(&mut self, req: &Value) -> Option<Value> {
        let id = req.get("id");
        let method = req["method"].as_str().unwrap_or("");

        match method {
            "initialize" => Some(jsonrpc_ok(id?, json!({
                "protocolVersion": "2024-11-05",
                "capabilities": { "tools": {} },
                "serverInfo": { "name": "drifter", "version": "0.2.0" }
            }))),
            "notifications/initialized" | "notifications/cancelled" => None,
            "tools/list" => Some(jsonrpc_ok(id?, json!({ "tools": tool_defs() }))),
            "tools/call" => {
                let name = req["params"]["name"].as_str().unwrap_or("");
                let args = req["params"].get("arguments").cloned().unwrap_or(json!({}));
                Some(jsonrpc_ok(id?, self.call(name, &args)))
            }
            _ => id.map(|id| jsonrpc_err(id, -32601, &format!("unknown method: {method}"))),
        }
    }

    fn call(&mut self, name: &str, args: &Value) -> Value {
        let result = match name {
            "new_game" => self.tool_new_game(args),
            "get_state" => self.tool_get_state(),
            "travel" => self.tool_travel(args),
            "buy" => self.tool_buy(args),
            "sell" => self.tool_sell(args),
            "bank_deposit" => self.tool_bank_deposit(args),
            "bank_withdraw" => self.tool_bank_withdraw(args),
            "repay_debt" => self.tool_repay_debt(args),
            "visit_hospital" => self.tool_visit_hospital(args),
            "rent_house" => self.tool_rent_house(),
            "visit_cafe" => self.tool_visit_cafe(),
            "benchmark" => return self.tool_benchmark(args),
            _ => json!({"error": format!("unknown tool: {name}")}),
        };
        if result.get("error").is_some() {
            json!({"content": [{"type": "text", "text": result.to_string()}], "isError": true})
        } else {
            mcp_text(&result)
        }
    }

    fn active_game(&mut self) -> Result<&mut GameState, Value> {
        match self.game.as_mut() {
            Some(g) if !g.game_ended => Ok(g),
            Some(_) => Err(json!({"error": "game already ended, start a new_game"})),
            None => Err(json!({"error": "no active game, call new_game first"})),
        }
    }

    fn tool_new_game(&mut self, args: &Value) -> Value {
        let seed = args.get("seed").and_then(|v| v.as_u64());
        if let Some(s) = args.get("lang").and_then(|v| v.as_str()) {
            self.lang = if s == "zh" { Lang::Zh } else { Lang::En };
        }
        let mut game = GameState::with_seed(seed);
        game.start();
        let view = game_view(&game, self.lang);
        self.game = Some(game);
        view
    }

    fn tool_get_state(&self) -> Value {
        match &self.game {
            Some(g) => game_view(g, self.lang),
            None => json!({"error": "no active game, call new_game first"}),
        }
    }

    fn tool_travel(&mut self, args: &Value) -> Value {
        let lang = self.lang;
        let g = match self.active_game() {
            Ok(g) => g,
            Err(e) => return e,
        };
        let Some(loc) = args.get("location").and_then(|v| v.as_u64()) else {
            return json!({"error": "missing location (0-9)"});
        };
        let loc = loc as usize;
        if loc >= LOCATION_COUNT {
            return json!({"error": format!("location must be 0-{}", LOCATION_COUNT - 1)});
        }
        g.travel_to(loc, lang);
        let events = drain_events(g);
        let mut view = game_view(g, lang);
        view["events"] = json!(events);
        view
    }

    fn tool_buy(&mut self, args: &Value) -> Value {
        let lang = self.lang;
        let g = match self.active_game() {
            Ok(g) => g,
            Err(e) => return e,
        };
        let good = args.get("good").and_then(|v| v.as_u64()).unwrap_or(99) as usize;
        let qty = args.get("quantity").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        if good >= GOOD_COUNT {
            return json!({"error": "good must be 0-7"});
        }
        if !g.buy(good, qty) {
            return json!({"error": "cannot buy: not enough cash or space", "state": game_view(g, lang)});
        }
        game_view(g, lang)
    }

    fn tool_sell(&mut self, args: &Value) -> Value {
        let lang = self.lang;
        let g = match self.active_game() {
            Ok(g) => g,
            Err(e) => return e,
        };
        let good = args.get("good").and_then(|v| v.as_u64()).unwrap_or(99) as usize;
        let qty = args.get("quantity").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        if good >= GOOD_COUNT {
            return json!({"error": "good must be 0-7"});
        }
        if !g.sell(good, qty, lang) {
            return json!({"error": "cannot sell: insufficient quantity or not available"});
        }
        let events = drain_events(g);
        let mut view = game_view(g, lang);
        if !events.is_empty() {
            view["events"] = json!(events);
        }
        view
    }

    fn tool_bank_deposit(&mut self, args: &Value) -> Value {
        let lang = self.lang;
        let g = match self.active_game() {
            Ok(g) => g,
            Err(e) => return e,
        };
        let amount = args.get("amount").and_then(|v| v.as_i64()).unwrap_or(0);
        if !g.bank_deposit(amount) {
            return json!({"error": "cannot deposit: invalid amount"});
        }
        game_view(g, lang)
    }

    fn tool_bank_withdraw(&mut self, args: &Value) -> Value {
        let lang = self.lang;
        let g = match self.active_game() {
            Ok(g) => g,
            Err(e) => return e,
        };
        let amount = args.get("amount").and_then(|v| v.as_i64()).unwrap_or(0);
        if !g.bank_withdraw(amount) {
            return json!({"error": "cannot withdraw: invalid amount"});
        }
        game_view(g, lang)
    }

    fn tool_repay_debt(&mut self, args: &Value) -> Value {
        let lang = self.lang;
        let g = match self.active_game() {
            Ok(g) => g,
            Err(e) => return e,
        };
        let amount = args.get("amount").and_then(|v| v.as_i64()).unwrap_or(0);
        if !g.repay_debt(amount) {
            return json!({"error": "cannot repay: invalid amount or no debt"});
        }
        game_view(g, lang)
    }

    fn tool_visit_hospital(&mut self, args: &Value) -> Value {
        let lang = self.lang;
        let g = match self.active_game() {
            Ok(g) => g,
            Err(e) => return e,
        };
        let points = args.get("points").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        if !g.visit_hospital(points) {
            return json!({"error": "cannot heal: full health or not enough cash"});
        }
        game_view(g, lang)
    }

    fn tool_rent_house(&mut self) -> Value {
        let lang = self.lang;
        let g = match self.active_game() {
            Ok(g) => g,
            Err(e) => return e,
        };
        match g.rent_house(lang) {
            Ok(_) => game_view(g, lang),
            Err(msg) => json!({"error": msg}),
        }
    }

    fn tool_visit_cafe(&mut self) -> Value {
        let lang = self.lang;
        let g = match self.active_game() {
            Ok(g) => g,
            Err(e) => return e,
        };
        match g.visit_cafe(lang) {
            Ok(msg) => {
                let mut view = game_view(g, lang);
                view["message"] = json!(msg);
                view
            }
            Err(msg) => json!({"error": msg}),
        }
    }

    fn tool_benchmark(&mut self, args: &Value) -> Value {
        let n = args.get("games").and_then(|v| v.as_u64()).unwrap_or(100) as usize;
        let base_seed = args.get("seed").and_then(|v| v.as_u64()).unwrap_or(42);
        let n = n.clamp(1, 10000);

        let mut scores: Vec<i64> = Vec::with_capacity(n);
        let mut deaths = 0usize;
        let mut best = (i64::MIN, 0u64);
        let mut worst = (i64::MAX, 0u64);

        for i in 0..n {
            let seed = base_seed.wrapping_mul(6364136223846793005).wrapping_add(i as u64);
            let r = benchmark_one(seed);
            scores.push(r.score);
            if r.dead {
                deaths += 1;
            }
            if r.score > best.0 {
                best = (r.score, seed);
            }
            if r.score < worst.0 {
                worst = (r.score, seed);
            }
        }

        scores.sort();
        let total: i64 = scores.iter().sum();
        let avg = total / n as i64;
        let median = scores[n / 2];
        let positive = scores.iter().filter(|&&s| s > 0).count();

        let best_detail = benchmark_one(best.1);
        let worst_detail = benchmark_one(worst.1);

        let brackets: &[(i64, &str)] = &[
            (-100_000, "< -100k"),
            (0, "-100k ~ 0"),
            (10_000, "0 ~ 10k"),
            (50_000, "10k ~ 50k"),
            (200_000, "50k ~ 200k"),
            (1_000_000, "200k ~ 1M"),
            (10_000_000, "1M ~ 10M"),
            (i64::MAX, "> 10M"),
        ];
        let mut dist = Vec::new();
        let mut prev = i64::MIN;
        for &(upper, label) in brackets {
            let count = scores.iter().filter(|&&s| s >= prev && s < upper).count();
            dist.push(json!({"range": label, "count": count}));
            prev = upper;
        }

        let result = json!({
            "summary": {
                "games": n,
                "best_score": best.0,
                "worst_score": worst.0,
                "average": avg,
                "median": median,
                "positive_rate": format!("{:.1}%", positive as f64 / n as f64 * 100.0),
                "deaths": deaths
            },
            "distribution": dist,
            "best_game": {
                "seed": best.1,
                "score": best_detail.score,
                "cash": best_detail.cash,
                "bank": best_detail.bank,
                "debt": best_detail.debt,
                "health": best_detail.health,
                "fame": best_detail.fame
            },
            "worst_game": {
                "seed": worst.1,
                "score": worst_detail.score,
                "cash": worst_detail.cash,
                "bank": worst_detail.bank,
                "debt": worst_detail.debt,
                "health": worst_detail.health,
                "fame": worst_detail.fame
            }
        });
        mcp_text(&result)
    }
}

// ---- helpers ----

fn mcp_text(v: &Value) -> Value {
    json!({"content": [{"type": "text", "text": v.to_string()}]})
}

fn jsonrpc_ok(id: &Value, result: Value) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "result": result})
}

fn jsonrpc_err(id: &Value, code: i32, msg: &str) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": msg}})
}

fn game_view(game: &GameState, lang: Lang) -> Value {
    let names = data::location_names(game.subway_mode, lang);
    let goods: Vec<Value> = (0..GOOD_COUNT)
        .map(|i| {
            json!({
                "index": i,
                "name": data::good_name(i, lang),
                "price": game.prices[i],
                "inventory": game.inventory[i],
                "avg_cost": game.avg_cost(i)
            })
        })
        .collect();

    json!({
        "day": game.days_elapsed(),
        "days_left": game.days_left,
        "cash": game.cash,
        "debt": game.debt,
        "bank": game.bank,
        "health": game.health,
        "fame": game.fame,
        "score": game.score(),
        "capacity": game.capacity,
        "free_space": game.free_space(),
        "current_location": game.current_location,
        "game_ended": game.game_ended,
        "dead": game.dead,
        "can_rent_house": game.can_rent_house(),
        "can_visit_cafe": game.can_visit_cafe(),
        "subway_mode": game.subway_mode,
        "goods": goods,
        "locations": names.iter().enumerate().map(|(i, &name)| {
            json!({"index": i, "name": name})
        }).collect::<Vec<Value>>()
    })
}

fn drain_events(game: &mut GameState) -> Vec<String> {
    let mut events = Vec::new();
    while let Some(evt) = game.pop_event() {
        events.push(format!("[{}] {}", evt.title, evt.body));
    }
    events
}

// ---- tool definitions ----

fn tool_defs() -> Value {
    json!([
        tool_def("new_game",
            "Start a new game of Beijing Drifter. You begin with 2000 cash, 5000 debt (10% daily interest), 100 health, 100 fame, 100 bag capacity, and 40 days. Returns initial state with day-0 prices. Strategy tip: repay debt ASAP (10%/day is devastating), buy low sell high, and bank surplus cash after debt is cleared (1%/day interest).",
            json!({
                "type": "object",
                "properties": {
                    "seed": {"type": "integer", "description": "Random seed for deterministic replay"},
                    "lang": {"type": "string", "enum": ["zh", "en"], "description": "Language for event messages (default: en)"}
                }
            })),
        tool_def("get_state",
            "Get current game state: cash, debt, bank, health, fame, prices, inventory, locations.",
            json!({"type": "object"})),
        tool_def("travel",
            "Travel to a location (0-9). This is the main turn action: triggers a new day with price refresh, 10% debt interest, 1% bank interest, and random events (market booms/crashes, health incidents, theft). You MUST travel to advance the game. Buy/sell BEFORE traveling to lock in current prices.",
            json!({
                "type": "object",
                "properties": {
                    "location": {"type": "integer", "description": "Location index 0-9. Must differ from current location.", "minimum": 0, "maximum": 9}
                },
                "required": ["location"]
            })),
        tool_def("buy",
            "Buy goods at current market price. Price 0 means unavailable. Limited by cash and free bag space.",
            json!({
                "type": "object",
                "properties": {
                    "good": {"type": "integer", "description": "0=cigarettes 1=cars 2=VCDs 3=fake_liquor 4=banned_book 5=toys 6=phones 7=cosmetics", "minimum": 0, "maximum": 7},
                    "quantity": {"type": "integer", "minimum": 1}
                },
                "required": ["good", "quantity"]
            })),
        tool_def("sell",
            "Sell goods from inventory. Selling fake_liquor(3) costs 10 fame, banned_book(4) costs 7 fame.",
            json!({
                "type": "object",
                "properties": {
                    "good": {"type": "integer", "minimum": 0, "maximum": 7},
                    "quantity": {"type": "integer", "minimum": 1}
                },
                "required": ["good", "quantity"]
            })),
        tool_def("bank_deposit",
            "Deposit cash to bank. Bank pays 1% daily compound interest.",
            json!({
                "type": "object",
                "properties": {
                    "amount": {"type": "integer", "minimum": 1}
                },
                "required": ["amount"]
            })),
        tool_def("bank_withdraw",
            "Withdraw cash from bank.",
            json!({
                "type": "object",
                "properties": {
                    "amount": {"type": "integer", "minimum": 1}
                },
                "required": ["amount"]
            })),
        tool_def("repay_debt",
            "Repay debt. Debt charges 10% DAILY compound interest -- pay it off early! 5000 debt becomes 226k in 40 days if unpaid.",
            json!({
                "type": "object",
                "properties": {
                    "amount": {"type": "integer", "minimum": 1}
                },
                "required": ["amount"]
            })),
        tool_def("visit_hospital",
            "Heal at hospital, 3500 yuan per health point. If health drops below 0 you die.",
            json!({
                "type": "object",
                "properties": {
                    "points": {"type": "integer", "minimum": 1}
                },
                "required": ["points"]
            })),
        tool_def("rent_house",
            "Expand bag capacity by 10 (max 140). Requires >= 30000 cash. Cost: 25000 if cash <= 30000, otherwise cash becomes cash/2 - 2000.",
            json!({"type": "object"})),
        tool_def("visit_cafe",
            "Visit internet cafe for a small cash reward (1-10 yuan). Max 3 visits per game. Requires >= 15 cash (not deducted).",
            json!({"type": "object"})),
        tool_def("benchmark",
            "Run N games with a built-in greedy AI and return aggregate statistics: best/worst/average/median scores, win rate, death count, score distribution, and details of the best and worst games.",
            json!({
                "type": "object",
                "properties": {
                    "games": {"type": "integer", "description": "Number of games (default 100, max 10000)", "minimum": 1, "maximum": 10000},
                    "seed": {"type": "integer", "description": "Base random seed (default 42)"}
                }
            })),
    ])
}

fn tool_def(name: &str, desc: &str, schema: Value) -> Value {
    json!({"name": name, "description": desc, "inputSchema": schema})
}

// ---- benchmark AI ----

struct BenchmarkResult {
    score: i64,
    cash: i64,
    bank: i64,
    debt: i64,
    health: i32,
    fame: i32,
    dead: bool,
}

fn benchmark_one(seed: u64) -> BenchmarkResult {
    let mut game = GameState::with_seed(Some(seed));
    game.start();
    ai_trade(&mut game);

    for turn in 0..50 {
        if game.game_ended {
            break;
        }
        let loc = (turn + 1) % LOCATION_COUNT;
        game.travel_to(loc, Lang::En);
        while game.pop_event().is_some() {}
        if game.game_ended {
            break;
        }
        ai_facilities(&mut game);
        ai_trade(&mut game);
    }

    BenchmarkResult {
        score: game.score(),
        cash: game.cash,
        bank: game.bank,
        debt: game.debt,
        health: game.health,
        fame: game.fame,
        dead: game.dead,
    }
}

fn ai_facilities(game: &mut GameState) {
    while game.can_visit_cafe() {
        let _ = game.visit_cafe(Lang::En);
    }
    while game.pop_event().is_some() {}

    if game.debt > 0 {
        let keep = if game.days_left > 10 { 5000 } else { 2000 };
        let repay = (game.cash - keep).max(0).min(game.debt);
        if repay > 0 {
            game.repay_debt(repay);
        }
    }

    if game.days_left > 15 && game.can_rent_house() {
        let _ = game.rent_house(Lang::En);
    }

    if game.health < 50 && game.cash > 50000 {
        let pts = (80 - game.health).max(0).min((game.cash / 3500) as i32);
        if pts > 0 {
            game.visit_hospital(pts);
        }
    }

    if game.debt == 0 && game.days_left > 2 {
        let deposit = (game.cash - 20000).max(0);
        if deposit > 0 {
            game.bank_deposit(deposit);
        }
    }

    if game.days_left <= 2 && game.bank > 0 {
        game.bank_withdraw(game.bank);
    }
}

fn ai_trade(game: &mut GameState) {
    for i in 0..GOOD_COUNT {
        if game.inventory[i] <= 0 || game.prices[i] == 0 {
            continue;
        }
        let avg = game.avg_cost(i).unwrap_or(0);
        if game.prices[i] > avg || game.days_left <= 2 {
            game.sell(i, game.inventory[i], Lang::En);
        }
    }
    while game.pop_event().is_some() {}

    if game.cash <= 0 || game.free_space() <= 0 {
        return;
    }

    let mut opps: Vec<(usize, f64)> = (0..GOOD_COUNT)
        .filter(|&i| game.prices[i] > 0)
        .map(|i| {
            let avg = GOODS[i].base_price + GOODS[i].price_range / 2;
            let ratio = avg as f64 / game.prices[i] as f64;
            let bonus = if i == 1 || i == 4 { 1.2 } else { 1.0 };
            (i, ratio * bonus)
        })
        .collect();
    opps.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (idx, score) in &opps {
        if *score < 0.8 {
            break;
        }
        let max_qty = ((game.cash / game.prices[*idx]) as i32).min(game.free_space());
        if max_qty <= 0 {
            continue;
        }
        game.buy(*idx, max_qty);
        if game.free_space() <= 0 {
            break;
        }
    }
}
