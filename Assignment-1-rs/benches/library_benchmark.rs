use Assignment_1_rs::other;
use Assignment_1_rs::sorting;
use gungraun::{library_benchmark, library_benchmark_group, main};
use std::fs;
use std::fs::File;
use std::hint::black_box;
use std::io::Write;

const BENCH_TYPE: &str = "reversed";

// Get an array of the given size for testing
fn get_test_array(size: String) -> Vec<i32> {
    let array = match BENCH_TYPE {
        "linear" => other::linear_array(size.parse().unwrap()),
        "reversed" => other::reversed_array(size.parse().unwrap()),
        "random" => other::reversed_array(size.parse().unwrap()),
        _ => panic!("invalid benchmark type!"),
    };

    let p = format!("./test_data/{}/{}.txt", BENCH_TYPE, size);
    //fs::create_dir_all(&p).expect("couln't make test data directory");
    let mut file = File::create(p.as_str()).unwrap();
    array
        .iter()
        .for_each(|n| file.write_all((n.to_string() + "\n").as_bytes()).unwrap());
    array
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
