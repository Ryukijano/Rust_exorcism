# Parallel Sum

Welcome to Parallel Sum.

## Instructions

Compute the sum of a slice of integers, using multiple worker threads.

The goal is to practice:

- Splitting work into chunks
- Spawning and joining threads
- Avoiding data races without cloning the whole input

Work through the tests in `tests/parallel-sum.rs` by removing `#[ignore]` one-by-one.

