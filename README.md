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

# Rotate Image (matrix 90° clockwise)
cargo test rotate_image

# Remove Duplicates from Sorted Array
cargo test remove_duplicates_from_sorted_array

# Best Time to Buy and Sell Stock
cargo test best_time_to_buy_and_sell_stock

# Best Time to Buy and Sell Stock II
cargo test best_time_to_buy_and_sell_stock_2

# Contains Duplicate
cargo test contains_duplicate

# Climbing Stairs
cargo test climbing_stairs

# Single Number
cargo test single_number

# Intersection of Two Arrays II
cargo test intersection_of_two_arrays_2

# Plus One
cargo test plus_one

# Power of Three
cargo test power_of_three

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

# First Bad Version
cargo test first_bad_version

# String to Integer (atoi)
cargo test string_to_integer_atoi

# Implement strStr()
cargo test implement_str_str

# Longest Common Prefix
cargo test longest_common_prefix

# Remove Nth Node From End of List
cargo test remove_nth_node_from_end_of_the_list

# Reverse Linked List
cargo test reverse_linked_list

# Merge Two Sorted Lists
cargo test merge_two_sorted_lists

# Merge Sorted Array
cargo test merge_sorted_array

# Palindrome Linked List
cargo test palindrome_linked_list

# House Robber
cargo test house_robber

# Maximum Subarray
cargo test maximum_subarray

# Maximum Depth of Binary Tree
cargo test maximum_depth_of_binary_tree

# Validate Binary Search Tree
cargo test validate_binary_search_tree

# Symmetric Tree
cargo test symmetric_tree

# Reverse String
cargo test reverse_string

# Reverse Integer
cargo test reverse_integer

# Shuffle an Array
cargo test shuffle_an_array

# Fizz Buzz
cargo test fizz_buzz

# Count Primes
cargo test count_primes

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
| **Rotate Image** | `rotate_image` | Rotate an `n × n` matrix 90° clockwise; builds a temp grid then copies back (`O(n²)` extra space). |
| **Remove Duplicates from Sorted Array** | `remove_duplicates_from_sorted_array` | In-place dedup of a sorted vector; returns length of unique prefix. |
| **Best Time to Buy and Sell Stock** | `best_time_to_buy_and_sell_stock` | One transaction; track min-so-far buy price and best profit in one pass (`O(n)` time, `O(1)` space). |
| **Best Time to Buy and Sell Stock II** | `best_time_to_buy_and_sell_stock_2` | Unlimited trades, one share at a time; greedy max profit from daily price series. |
| **Contains Duplicate** | `contains_duplicate` | Detect duplicates (brute-force, `HashSet` length, single-pass inserts, `contains` + `insert`, …). |
| **Climbing Stairs** | `climbing_stairs` | Count ways to climb `n` steps with 1- or 2-step moves; Fibonacci recurrence with two variables (`O(n)` time, `O(1)` space). |
| **Single Number** | `single_number` | Find the unique element when every other value appears exactly twice. |
| **Intersection of Two Arrays II** | `intersection_of_two_arrays_2` | Return common values with multiplicity using sort + two pointers. |
| **Plus One** | `plus_one` | Increment a big-endian digit vector by one (carry from the right). |
| **Power of Three** | `power_of_three` | Check if `n` is `3^k` by scaling from `3` with `checked_mul` until `pow >= n`. |
| **Move Zeroes** | `move_zeroes` | Move all zeroes to the end while preserving non-zero order. |
| **Two Sum** | `two_sum` | Return indices of two values whose sum equals the target. |
| **Valid Sudoku** | `valid_sudoku` | Check a 9×9 board: no duplicate digits in any row, column, or 3×3 box. |
| **Valid Anagram** | `valid_anagram` | Check whether two strings are anagrams by comparing their sorted character sequences. |
| **Valid Palindrome** | `valid_palindrome` | Check whether a string reads the same forward and backward after lowercasing and stripping non-alphanumeric characters. |
| **Reverse String** | `reverse_string` | Reverse a `Vec<char>` in place (`Vec::reverse`) and an iterator-based variant. |
| **Reverse Integer** | `reverse_integer` | Reverse decimal digits of a signed 32-bit integer; return `0` on overflow. |
| **First Unique Character in a String** | `first_unique_character_in_a_string` | Return the zero-based index of the first non-repeating character, or `-1` if none exists. |
| **First Bad Version** | `first_bad_version` | Binary search on `1..=n` with `isBadVersion`; midpoint via `u32` to avoid `i32` overflow. |
| **String to Integer (atoi)** | `string_to_integer_atoi` | Parse a string to `i32`: trim, optional sign, digit run, clamp on overflow. |
| **Implement strStr()** | `implement_str_str` | Return the first index of `needle` in `haystack`, or `-1` if not found. |
| **Longest Common Prefix** | `longest_common_prefix` | Longest prefix shared by all strings (column-wise compare up to the shortest length). |
| **Remove Nth Node From End of List** | `remove_nth_node_from_end_of_the_list` | Remove the n-th node from the end of a singly linked list (vector rebuild and two-pass pointer rewiring). |
| **Reverse Linked List** | `reverse_linked_list` | Reverse a singly linked list by collecting values and rebuilding with prepended nodes (`fold`). |
| **Merge Two Sorted Lists** | `merge_two_sorted_lists` | Merge two sorted linked lists into one sorted list (collect values, sort, rebuild new nodes). |
| **Merge Sorted Array** | `merge_sorted_array` | Merge two sorted vectors into `nums1` (length `m+n`): drain tail, insert each `nums2` head into sorted position (`insert` / `remove(0)`). |
| **Palindrome Linked List** | `palindrome_linked_list` | Check if a linked list is a palindrome (collect values and compare mirrored halves). |
| **House Robber** | `house_robber` | DP: at each house take `dp[i-2] + nums[i]` vs skip `dp[i-1]`; rolling two-state implementation (`O(n)` time, `O(1)` space). |
| **Maximum Subarray** | `maximum_subarray` | Max sum over a non-empty contiguous subarray (Kadane-style one-pass DP with running best). |
| **Maximum Depth of Binary Tree** | `maximum_depth_of_binary_tree` | Compute tree height via DFS recursion; return the longest root-to-leaf path length. |
| **Validate Binary Search Tree** | `validate_binary_search_tree` | Validate BST ordering (strict inequalities) across the whole tree. |
| **Symmetric Tree** | `symmetric_tree` | Check whether a binary tree is a mirror of itself (left and right subtrees mirror-matched recursively). |
| **Shuffle an Array** | `shuffle_an_array` | Implement `reset()` and `shuffle()`; includes Fisher–Yates (`rand`) and a simple custom RNG variant. |
| **Fizz Buzz** | `fizz_buzz` | Generate strings from `1..=n` with “Fizz”/“Buzz”/“FizzBuzz” divisibility rules. |
| **Count Primes** | `count_primes` | Count primes `< n` with a sieve (`O(n log log n)` time, `O(n)` space). |

---

## Project layout

```
algotics/
├── src/
│   ├── lib.rs
│   └── top_interview/
│       ├── rotate_array.rs
│       ├── rotate_image.rs
│       ├── remove_duplicates_from_sorted_array.rs
│       ├── best_time_to_buy_and_sell_stock.rs
│       ├── best_time_to_buy_and_sell_stock_2.rs
│       ├── contains_duplicate.rs
│       ├── climbing_stairs.rs
│       ├── single_number.rs
│       ├── intersection_of_two_arrays_2.rs
│       ├── plus_one.rs
│       ├── power_of_three.rs
│       ├── move_zeroes.rs
│       ├── two_sum.rs
│       ├── valid_sudoku.rs
│       ├── valid_anagram.rs
│       ├── valid_palindrome.rs
│       ├── first_bad_version.rs
│       ├── first_unique_character_in_a_string.rs
│       ├── string_to_integer_atoi.rs
│       ├── implement_str_str.rs
│       ├── longest_common_prefix.rs
│       ├── remove_nth_node_from_end_of_the_list.rs
│       ├── reverse_linked_list.rs
│       ├── merge_two_sorted_lists.rs
│       ├── merge_sorted_array.rs
│       ├── palindrome_linked_list.rs
│       ├── house_robber.rs
│       ├── maximum_subarray.rs
│       ├── maximum_depth_of_binary_tree.rs
│       ├── validate_binary_search_tree.rs
│       ├── symmetric_tree.rs
│       ├── reverse_string.rs
│       └── reverse_integer.rs
│       ├── shuffle_an_array.rs
│       ├── fizz_buzz.rs
│       ├── count_primes.rs
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
