# Drifter

[中文](README.md)

A modern TUI remake of the classic Chinese trading game "Beijing Drifter" (北京浮生记) -- play it yourself, or **let AI play for you**.

Built-in [MCP](https://modelcontextprotocol.io) server lets Claude, ChatGPT, Copilot and other AI models play the game through tool calls: making buy/sell/travel decisions turn by turn for a full 40-day game, or batch-running 1000 games to benchmark decision-making ability across models. Same seed, different models -- who profits more?

![Drifter TUI - Trading](assets/screenshot.png)

![Drifter TUI - Main Menu](assets/menu.png)

## About

This project is a terminal-based reimplementation of [Beijing Fushengji (北京浮生记)](https://github.com/chrisguo/beijing_fushengji), a classic Windows game originally written in VC++ 6.0 by Guo Xianghao (2002). The original game is a Chinese adaptation of the "Drug Wars" genre.

<details>
<summary>Original Windows version</summary>

![Original](assets/original.png)

</details>

We faithfully reproduced all game mechanics from the original source code, while modernizing the interface for the terminal.

## Features

- **Let AI play for you** - built-in [MCP](https://modelcontextprotocol.io) server + [Claude Code Skill](https://docs.anthropic.com/en/docs/claude-code); tell an AI to play the whole game in one sentence; works with Claude Code / Codex / Claude Desktop / Cursor / Windsurf / VS Code Copilot and more
- **AI Benchmark** - batch-run N games with fixed seeds to compare decision-making across models; outputs win rate, score distribution, and best game details
- **100% faithful game logic** - all 18 market events, 12 health events, 7 theft events, interest rates, fame system, and more, verified line-by-line against the original C++ source
- **Bilingual** - full Chinese/English support, switch with `L` at any time
- **Modern TUI** - day progress bar, color-coded prices (green = profitable, red = loss), unified status dashboard
- **Cross-platform** - macOS (Intel + Apple Silicon), Windows, Linux (x86_64 + aarch64)
- **Single binary** - no dependencies, no runtime, no installation, just run it
- **Keyboard-driven** - WASD / arrow keys for navigation, natural panel flow

## Install

### macOS (Homebrew)

```bash
brew tap vraxel/tap
brew trust vraxel/tap
brew install drifter
drifter
```

### Windows (Scoop)

```powershell
scoop bucket add vraxel https://github.com/vraxel/scoop-bucket
scoop install drifter
drifter
```

### Download binary

Or grab the latest release from [Releases](../../releases):

| Platform | File |
|---|---|
| macOS (Universal) | `drifter-macos-universal.tar.gz` |
| macOS (Intel) | `drifter-macos-x86_64.tar.gz` |
| macOS (Apple Silicon) | `drifter-macos-aarch64.tar.gz` |
| Linux (x86_64) | `drifter-linux-x86_64.tar.gz` |
| Linux (aarch64) | `drifter-linux-aarch64.tar.gz` |
| Windows (64-bit) | `drifter-windows-x86_64.zip` |

> **Windows note**: On first run, Windows Defender SmartScreen may show a "Windows protected your PC" warning because the exe is not code-signed (open-source projects typically don't purchase signing certificates). Click "More info" -> "Run anyway". If Defender quarantines the file, go to "Windows Security" -> "Virus & threat protection" -> "Protection history", find the entry, and choose "Allow" to restore it.

### Build from source

```bash
git clone https://github.com/vraxel/drifter.git
cd drifter
cargo run --release
```

## CLI

```
drifter              Start the TUI game
drifter mcp          Start MCP server (stdio, JSON-RPC 2.0)
drifter -v|--version Print version
drifter -h|--help    Print help
```

## Controls

| Key | Action |
|---|---|
| WASD / Arrow keys | Navigate |
| Enter / J | Confirm / Buy / Sell / Travel |
| Esc / K | Cancel / Back |
| Q | Buy goods |
| E | Sell goods |
| B / H / P / R / N | Bank / Hospital / Post Office / House Agency / Internet Cafe |
| G | Switch subway/surface map |
| L | Toggle Chinese/English |
| O | Settings |
| Tab | Cycle panels |

## Game Rules

You are a villager who comes to Beijing with **2,000 yuan** and **5,000 yuan of debt** (10% daily interest). You have **40 days** to trade 8 types of goods across 10 locations, dealing with random market swings, street thugs, and corrupt officials.

**Goal**: Maximize `Cash + Savings - Debt` by day 40.

**Facilities**: Bank (1% daily interest on savings), Hospital (3,500/point), House Agency (expand bag capacity), Post Office (repay debt), Internet Cafe (earn small rewards).

## AI Play (MCP)

Drifter includes a built-in MCP (Model Context Protocol) server. Any MCP-compatible AI client can play the game through tool calls. Two modes:

- **Interactive** -- the AI makes every decision (buy/sell/travel) turn by turn, playing a full 40-day game
- **Benchmark** -- built-in greedy strategy runs N games and returns aggregate statistics (best/worst/average scores, win rate, score distribution)

Use the same seed across different models to compare their decision-making ability.

### Available Tools

| Tool | Description |
|---|---|
| `new_game` | Start a new game (optional seed and language) |
| `get_state` | Get current game state |
| `travel` | Travel to a location (main turn action, triggers new day) |
| `buy` / `sell` | Buy / sell goods |
| `bank_deposit` / `bank_withdraw` | Bank operations |
| `repay_debt` | Repay debt |
| `visit_hospital` | Heal at hospital |
| `rent_house` | Expand bag capacity |
| `visit_cafe` | Visit internet cafe |
| `benchmark` | Run N games and return statistics |

### Configuration

Make sure `drifter` is in your PATH (via Homebrew/Scoop or manual install), then configure your client:

<details>
<summary><b>Claude Code (CLI / IDE extensions)</b></summary>

```bash
claude mcp add drifter -- drifter mcp
```

Or manually edit `~/.claude.json`:

```json
{
  "mcpServers": {
    "drifter": {
      "command": "drifter",
      "args": ["mcp"]
    }
  }
}
```

Then just ask: "Play a game of Beijing Drifter" or "Benchmark 1000 games".

The project includes a Claude Code Skill (`.claude/skills/play-drifter.md`). When working in the drifter directory, use `/play-drifter` or `/play-drifter 1000`.

</details>

<details>
<summary><b>OpenAI Codex CLI</b></summary>

Edit `~/.codex/config.json`:

```json
{
  "mcpServers": {
    "drifter": {
      "command": "drifter",
      "args": ["mcp"]
    }
  }
}
```

Or create `.codex/config.json` in your project root (project-level config takes precedence).

</details>

<details>
<summary><b>Claude Desktop</b></summary>

Edit the config file:

- macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
- Windows: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "drifter": {
      "command": "drifter",
      "args": ["mcp"]
    }
  }
}
```

Restart Claude Desktop. The drifter tools will be available in chat.

</details>

<details>
<summary><b>Cursor</b></summary>

Open Cursor Settings -> MCP, click "Add new MCP server":

```json
{
  "mcpServers": {
    "drifter": {
      "command": "drifter",
      "args": ["mcp"]
    }
  }
}
```

Or create `.cursor/mcp.json` in your project root:

```json
{
  "mcpServers": {
    "drifter": {
      "command": "drifter",
      "args": ["mcp"]
    }
  }
}
```

</details>

<details>
<summary><b>Windsurf</b></summary>

Edit `~/.codeium/windsurf/mcp_config.json`:

```json
{
  "mcpServers": {
    "drifter": {
      "command": "drifter",
      "args": ["mcp"]
    }
  }
}
```

</details>

<details>
<summary><b>VS Code (GitHub Copilot)</b></summary>

Create `.vscode/mcp.json` in your project root:

```json
{
  "servers": {
    "drifter": {
      "type": "stdio",
      "command": "drifter",
      "args": ["mcp"]
    }
  }
}
```

Use Agent mode in Copilot Chat to access the tools.

</details>

<details>
<summary><b>Other MCP clients</b></summary>

Drifter MCP uses standard stdio transport. Any [MCP spec](https://modelcontextprotocol.io)-compatible client can connect:

```
Command:   drifter mcp
Transport: stdio
Protocol:  JSON-RPC 2.0
```

Send an `initialize` handshake, then use `tools/list` to discover tools and `tools/call` to invoke them.

</details>

## Tech Stack

- [Rust](https://www.rust-lang.org/)
- [ratatui](https://github.com/ratatui/ratatui) - TUI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - terminal backend
- [unicode-width](https://github.com/unicode-rs/unicode-width) - CJK character width handling

## Acknowledgements

- Original game: [chrisguo/beijing_fushengji](https://github.com/chrisguo/beijing_fushengji) by Guo Xianghao (GPL-2.0)
- Game concept inspired by "Drug Wars" / "Dope Wars"

## License

GPL-2.0 (same as the original game)
