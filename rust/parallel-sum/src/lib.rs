/// Sum the numbers in `input`, splitting work across up to `worker_count` threads.
///
/// - If `worker_count` is 0, treat it as 1.
/// - It should work correctly for any `worker_count`, including values larger than `input.len()`.
pub fn parallel_sum(_input: &[u64], _worker_count: usize) -> u64 {
    unimplemented!("implement a parallel sum");
}
