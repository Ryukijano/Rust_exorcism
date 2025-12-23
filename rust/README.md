# Rust exercises

This folder contains a collection of small, independent Rust exercises.
Each exercise is its own Cargo crate (with its own `Cargo.toml`).

## Running an exercise

From an exercise directory:

```bash
cargo test
```

Most exercises start with only the first test enabled. To progress, open the
exercise’s test file in `tests/` and remove one `#[ignore]` at a time.

## Running all exercise tests (parallel)

From `rust/`:

```bash
./run-all-tests.sh
```

## New exercises added

- `word-count`
- `anagram`
- `grains`
- `raindrops`
- `parallel-sum`

