# Drifter

[中文](README.md)

A modern TUI remake of the classic Chinese trading game "Beijing Drifter" (北京浮生记), built with Rust.

Trade goods across Beijing's subway stations, dodge random events, manage your finances, and try to make a fortune in 40 days.

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

- **100% faithful game logic** - all 18 market events, 12 health events, 7 theft events, interest rates, fame system, and more, verified line-by-line against the original C++ source
- **Bilingual** - full Chinese/English support, switch with `L` at any time
- **Modern TUI** - day progress bar, color-coded prices (green = profitable, red = loss), unified status dashboard
- **Cross-platform** - macOS (Intel + Apple Silicon), Windows, Linux (build from source)
- **Single binary** - no dependencies, no runtime, no installation, just run it
- **Keyboard-driven** - WASD / arrow keys for navigation, natural panel flow

## Install

### macOS (Homebrew)

```bash
brew tap vraxel/tap
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
| Windows (64-bit) | `drifter-windows-x86_64.zip` |

### Build from source

```bash
git clone https://github.com/vraxel/drifter.git
cd drifter
cargo run --release
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
