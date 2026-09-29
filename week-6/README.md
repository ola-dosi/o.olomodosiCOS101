# Week 6 — Strings and Operators

CSC 101 Practical, 2026/2027 Session.

Each practice is its own Cargo project. To run one:

```bash
cd practice-1
cargo run
```

## Part 1: Strings (Practices 1–7)

| Practice | Topic | What it does |
|---|---|---|
| 1 | String literals (`&str`) | Five hardcoded strings with type `&str` and `&'static str` |
| 2 | String objects | Creates empty and pre-filled Strings, measures their length |
| 3 | `replace()` | Finds and swaps text in a string |
| 4 | `push_str()` | Grows a String by appending text to it |
| 5 | `trim()` | Removes whitespace from the edges of a string |
| 6 | Concatenation | Joins Strings with the `+` operator |
| 7 | `format!` macro | Builds a String in a variable instead of printing it |

## Part 2: Operators (Practices 8–11)

| Practice | Topic | What it does |
|---|---|---|
| 8 | Arithmetic | All five: `+`, `-`, `*`, `/`, `%` on 10 and 2 |
| 9 | Relational | All six: `>`, `<`, `>=`, `<=`, `==`, `!=` on 10 and 20 |
| 10 | Logical | `&&` (both), `\|\|` (either), `!` (not) |
| 11 | Bitwise | `&`, `\|`, `^`, `!`, `<<`, `>>` on 2 and 3 |

## Project: Restaurant Menu

A real order-taking program. Displays a menu of 5 Nigerian dishes, reads which one and how many the customer wants, calculates the total, and applies a 5% discount if the total is over N10,000.

**Run it:**

```bash
cd project
cargo run
```

**Input:**
- Food choice: a single letter (P, F, A, E, or W)
- Quantity: a whole number

**Output:**
- Food name, price per unit, quantity, total
- If total exceeds N10,000, shows discount and final total

## Pushes

All practices (1–11) and the project are ready to commit:

```bash
git add week-6
git commit -m "Week 6: strings and operators (practices 1-11 and restaurant menu project)"
git push
```
