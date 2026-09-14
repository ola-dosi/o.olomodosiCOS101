# Week 4 — Decision Making and Loops

CSC 101 Practical, 2026/2027 Session.

Each practice is its own Cargo project. To run one:

```bash
cd practice-1
cargo run
```

| Practice | Topic | What it does |
|---|---|---|
| 1 | `std::io`, `read_line`, `parse` | Reads a name and an age and prints them back |
| 2 | Reading several numbers | Area of a triangle from three sides (Heron's formula) |
| 3 | `if` | Area from base and height, but only when the base is positive |
| 4 | `if else` | Party gate: 18 and over get in, everyone else does not |
| 5 | `else if` ladder | Sorts a height in cm into average / tall / dwarf / abnormal |
| 6 | `for` | Counts between a lower and upper bound typed by the user |
| 7 | `while` | Counts up to 10, then prints the value that ended the loop |
| 8 | `loop` + `break` | Runs forever until `x` reaches 15 |
| 9 | `continue` | Counts 1 to 20 but only tallies the values up to 10 |

## Notes

- `read_line` gives back **text**, always. `trim()` removes the newline the Enter
  key leaves behind, and `parse()` converts the text into a number.
- `..` in a range excludes the upper bound: `1..5` is 1, 2, 3, 4.
- `break` leaves the loop entirely; `continue` only skips the rest of the
  current pass and goes round again.
