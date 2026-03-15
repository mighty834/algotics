# Algotics

**Algorithm solutions in Rust** — a pet repository of clean, tested implementations for classic algorithms and interview-style problems.

---

## Quick start

Run tests for a specific algorithm by module name:

```bash
cargo test <module_name>
```

**Examples:**

```bash
# Run all tests for the rotate_array algorithm
cargo test rotate_array

# Run all tests in the top_interview category
cargo test top_interview

# Run the full test suite
cargo test
```

---

## Algorithms

### Top Interview

| Algorithm      | Module         | Description                                      |
|----------------|----------------|--------------------------------------------------|
| **Rotate Array** | `rotate_array` | Rotate an array to the right by `k` steps (in-place). |

---

## Project layout

```
algotics/
├── src/
│   ├── lib.rs
│   └── top_interview/
│       └── rotate_array.rs   # Rotate Array (LeetCode-style)
├── Cargo.toml
└── README.md
```

Each algorithm lives in its own module with doc comments and `#[cfg(test)]` unit tests. Use the module path as the filter for `cargo test`.

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
