# Binary Calculator (8-Bit)

An interactive 8-bit binary calculator built with **egui** (Rust). Learn how binary numbers work by clicking cards, toggling bits, and watching the decimal value update in real time.

## Features

- 🎮 Click cards or use keyboard keys to toggle individual bits
- 🖥️ Visual representation of 8-bit values (cards + LED indicators)
- 🔢 Real-time conversion: Binary ↔ Decimal ↔ Hexadecimal
- 🔤 ASCII character preview for printable characters
- ⚙️ Bit operations: Left/Right shift, Invert

## How to Run

```bash
cargo run
```

## Keyboard Controls

| Key | Action |
|-----|--------|
| `1`–`8` | Toggle bit at that position |
| `↑` / `↓` | Increment / Decrement value |
| `←` / `→` | Left/Right shift (×2 ÷2) |
| `Toggles button` | Switch between Simple/Full mode |

## Modes

- **Simple Mode**: Clean UI, shows only essential info (card number + decimal value + 1/0 state).
- **Full Mode**: Shows all details including power-of-two labels (`2ⁿ`) and ASCII descriptions.

Toggle with the green button in the top-left corner.

---

Built with Rust & egui — no dependencies required beyond `cargo run`.
