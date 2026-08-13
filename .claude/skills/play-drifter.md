---
name: play-drifter
description: "Play Beijing Drifter (北京浮生记) via MCP -- AI plays a full game or runs batch benchmarks"
user_invocable: true
---

# Play Drifter

You are playing Beijing Drifter (北京浮生记), a classic Chinese trading game, via MCP tools.

## Determine mode from user input

- No args or `1`: play ONE game interactively (you make every decision)
- A number N > 1: call `benchmark` with `games: N` and format the results
- `benchmark` or `bench`: same as passing 1000

## Interactive mode (single game)

### Setup
1. Call `new_game` (with `lang: "zh"` if user speaks Chinese, else `"en"`)
2. Read the initial state: prices, cash (2000), debt (5500 after day-0 interest)

### Each turn
1. **Sell** anything profitable: if current price > avg_cost for a held good, sell all of it
2. **Buy** the best opportunity: pick the good whose price is lowest relative to its typical range. Spend most of your cash. Prefer cars (index 1) and banned books (index 4) -- they have the highest multiplier events
3. **Facilities** (in this priority):
   - `visit_cafe` if available (free money)
   - `repay_debt` aggressively: keep only 5000 cash for trading, put the rest toward debt. 10%/day interest is the #1 enemy
   - `rent_house` if days_left > 15 and cash >= 30000
   - `bank_deposit` surplus after debt is cleared (keep 20000 for trading)
   - `bank_withdraw` everything on the last 2 days
   - `visit_hospital` if health < 50 and cash > 50000
4. **Travel** to the next location (cycle 0-9). This ends the turn and triggers the new day.
5. Read events from the travel response. Note any price-changing market events.

### Key rules
- You MUST travel to advance the game. Buy/sell happen BEFORE travel.
- Debt grows 10%/day. Unpaid 5000 becomes 226k by day 40. Pay it off ASAP.
- Bank pays 1%/day compound interest. Deposit surplus after debt is zero.
- If health drops below 0 you die. Below 85 triggers forced hospitalization (costs days + money added to debt).
- Selling fake liquor (good 3) costs 10 fame. Selling banned book (good 4) costs 7 fame.
- Last 2 days: all goods appear (no leaveout). Withdraw bank, sell everything, go all-in on high-value goods hoping for a multiplier event.

### After game ends
Show a summary table:

```
Day 40 -- Game Over
Score:  cash + bank - debt = XXX
Cash:   XXX
Bank:   XXX
Debt:   XXX
Health: XX
Fame:   XX
```

## Benchmark mode

1. Call `benchmark` with `{games: N}` (use the number the user gave, default 1000)
2. Format the response as a table:

```
=== Beijing Drifter Benchmark (N games) ===

| Metric       | Value         |
|-------------|---------------|
| Best score   | X,XXX,XXX     |
| Worst score  | -XXX,XXX      |
| Average      | XXX,XXX       |
| Median       | XXX,XXX       |
| Win rate     | XX.X%         |
| Deaths       | XX            |

Score distribution:
  < -100k:  XX  ####
  -100k~0:  XX  ##
      ...

Best game:  score=X,XXX,XXX  health=XX  fame=XX  (seed=XXXX)
Worst game: score=-XXX,XXX   health=XX  fame=XX  (seed=XXXX)
```

## Important

- All tool calls go through the `drifter` MCP server (must be configured in Claude Code settings)
- Use thousand separators for all money values
- Keep commentary minimal during gameplay -- show state changes, not analysis
- When playing interactively, aim to maximize score. Play to win.
