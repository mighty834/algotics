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

---

## Project layout

```
algotics/
├── src/
│   ├── lib.rs
│   └── top_interview/
│       ├── rotate_array.rs
│       └── remove_duplicates_from_sorted_array.rs
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
