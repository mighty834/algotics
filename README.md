# Algotics

**Algorithm solutions in Rust** — a pet repository of clean, tested implementations for classic algorithms and interview-style problems.

---

## Quick start

Run tests for a specific algorithm by **module name** (substring match):

```bash
cargo test <module_name>
```

**Examples:**

```bash
# Rotate Array — all unit tests in that module
cargo test rotate_array

# Remove Duplicates from Sorted Array
cargo test remove_duplicates_from_sorted_array

# Best Time to Buy and Sell Stock II
cargo test best_time_to_buy_and_sell_stock_2

# Contains Duplicate
cargo test contains_duplicate

# Single Number
cargo test single_number

# Intersection of Two Arrays II
cargo test intersection_of_two_arrays_2

# Plus One
cargo test plus_one

# Move Zeroes
cargo test move_zeroes

# Two Sum
cargo test two_sum

# Everything under `top_interview`
cargo test top_interview

# Full suite
cargo test
```

---

## Algorithms

### Top Interview

| Problem | Module | Notes |
|--------|--------|--------|
| **Rotate Array** | `rotate_array` | Rotate to the right by `k`; several approaches (stdlib, slices, iterators, …). |
| **Remove Duplicates from Sorted Array** | `remove_duplicates_from_sorted_array` | In-place dedup of a sorted vector; returns length of unique prefix. |
| **Best Time to Buy and Sell Stock II** | `best_time_to_buy_and_sell_stock_2` | Unlimited trades, one share at a time; greedy max profit from daily price series. |
| **Contains Duplicate** | `contains_duplicate` | Detect duplicates (brute-force, `HashSet` length, single-pass inserts, `contains` + `insert`, …). |
| **Single Number** | `single_number` | Find the unique element when every other value appears exactly twice. |
| **Intersection of Two Arrays II** | `intersection_of_two_arrays_2` | Return common values with multiplicity using sort + two pointers. |
| **Plus One** | `plus_one` | Increment a big-endian digit vector by one (carry from the right). |
| **Move Zeroes** | `move_zeroes` | Move all zeroes to the end while preserving non-zero order. |
| **Two Sum** | `two_sum` | Return indices of two values whose sum equals the target. |

---

## Project layout

```
algotics/
├── src/
│   ├── lib.rs
│   └── top_interview/
│       ├── rotate_array.rs
│       ├── remove_duplicates_from_sorted_array.rs
│       ├── best_time_to_buy_and_sell_stock_2.rs
│       ├── contains_duplicate.rs
│       ├── single_number.rs
│       ├── intersection_of_two_arrays_2.rs
│       ├── plus_one.rs
│       ├── move_zeroes.rs
│       └── two_sum.rs
├── Cargo.toml
├── rustfmt.toml
├── clippy.toml
└── README.md
```

Each solution file has **Rustdoc** on the problem and helpers, plus `#[cfg(test)]` tests. Filter `cargo test` by the submodule name (see above).

---

## Development

```bash
cargo fmt              # format (see rustfmt.toml)
cargo clippy           # lints (see clippy.toml)
cargo doc --no-deps    # open API docs locally
cargo test --doc       # run Rustdoc examples (doctests)
```

---

## Requirements

- [Rust](https://www.rust-lang.org/) (latest stable)

```bash
rustc --version
cargo --version
```

---

## License

Use and adapt the code as you like for learning and interviews.
