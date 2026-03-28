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

# Valid Sudoku
cargo test valid_sudoku

# Valid Anagram
cargo test valid_anagram

# Valid Palindrome
cargo test valid_palindrome

# First Unique Character in a String
cargo test first_unique_character_in_a_string

# String to Integer (atoi)
cargo test string_to_integer_atoi

# Implement strStr()
cargo test implement_str_str

# Longest Common Prefix
cargo test longest_common_prefix

# Reverse String
cargo test reverse_string

# Reverse Integer
cargo test reverse_integer

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
| **Valid Sudoku** | `valid_sudoku` | Check a 9×9 board: no duplicate digits in any row, column, or 3×3 box. |
| **Valid Anagram** | `valid_anagram` | Check whether two strings are anagrams by comparing their sorted character sequences. |
| **Valid Palindrome** | `valid_palindrome` | Check whether a string reads the same forward and backward after lowercasing and stripping non-alphanumeric characters. |
| **Reverse String** | `reverse_string` | Reverse a `Vec<char>` in place (`Vec::reverse`) and an iterator-based variant. |
| **Reverse Integer** | `reverse_integer` | Reverse decimal digits of a signed 32-bit integer; return `0` on overflow. |
| **First Unique Character in a String** | `first_unique_character_in_a_string` | Return the zero-based index of the first non-repeating character, or `-1` if none exists. |
| **String to Integer (atoi)** | `string_to_integer_atoi` | Parse a string to `i32`: trim, optional sign, digit run, clamp on overflow. |
| **Implement strStr()** | `implement_str_str` | Return the first index of `needle` in `haystack`, or `-1` if not found. |
| **Longest Common Prefix** | `longest_common_prefix` | Longest prefix shared by all strings (column-wise compare up to the shortest length). |

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
│       ├── two_sum.rs
│       ├── valid_sudoku.rs
│       ├── valid_anagram.rs
│       ├── valid_palindrome.rs
│       ├── first_unique_character_in_a_string.rs
│       ├── string_to_integer_atoi.rs
│       ├── implement_str_str.rs
│       ├── longest_common_prefix.rs
│       ├── reverse_string.rs
│       └── reverse_integer.rs
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
