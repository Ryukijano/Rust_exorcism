use parallel_sum::parallel_sum;

#[test]
fn sums_an_empty_slice() {
    assert_eq!(parallel_sum(&[], 4), 0);
}

#[test]
#[ignore]
fn sums_with_one_worker() {
    let input: Vec<u64> = (1..=1_000).collect();
    assert_eq!(parallel_sum(&input, 1), 500_500);
}

#[test]
#[ignore]
fn sums_with_many_workers() {
    let input: Vec<u64> = (1..=1_000_000).collect();
    assert_eq!(parallel_sum(&input, 8), 500_000_500_000);
}

#[test]
#[ignore]
fn worker_count_larger_than_input_is_ok() {
    let input = [10, 20, 30];
    assert_eq!(parallel_sum(&input, 64), 60);
}

#[test]
#[ignore]
fn worker_count_zero_treated_as_one() {
    let input: Vec<u64> = (1..=10).collect();
    assert_eq!(parallel_sum(&input, 0), 55);
}

