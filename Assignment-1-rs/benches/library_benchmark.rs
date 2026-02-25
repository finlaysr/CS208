use Assignment_1_rs::other;
use Assignment_1_rs::sorting;
use gungraun::{library_benchmark, library_benchmark_group, main};
use std::hint::black_box;

// Get a random array of the given size for testing
fn get_test_array(size: String) -> Vec<i32> {
    other::random_array(size.parse().unwrap())
}

// After benchmarking, check that the array is actually sorted
fn check_sorted(array: Vec<i32>) {
    assert!(array.is_sorted(), "Array was not sorted!!");
}

// Benchmark the merge sort
#[library_benchmark]
#[benches::merge(file = "benches/lengths", setup = get_test_array, teardown = check_sorted)]
fn bench_merge(mut array: Vec<i32>) -> Vec<i32> {
    black_box(sorting::merge_sort(&mut array));
    black_box(array)
}

// Benchmark the selection sort
#[library_benchmark]
#[benches::selection(file = "benches/lengths", setup = get_test_array, teardown = check_sorted)]
fn bench_selection(mut array: Vec<i32>) -> Vec<i32> {
    black_box(sorting::selection_sort(&mut array));
    black_box(array)
}

// Outputs
library_benchmark_group!(
    name = bench_sorting,
    benchmarks = [bench_merge, bench_selection]
);
main!(library_benchmark_groups = bench_sorting);
