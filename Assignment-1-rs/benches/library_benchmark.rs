use Assignment_1_rs::other;
use Assignment_1_rs::other::BenchType;
use Assignment_1_rs::sorting;
use gungraun::{library_benchmark, library_benchmark_group, main};
use std::env;
use std::fs::OpenOptions;
use std::hint::black_box;
use std::path::Path;

// Get an array of the given size for testing
fn get_test_array(size: String) -> Vec<i32> {
    let bench_type = BenchType::from_file(Path::new("./benches/bench_type.txt"));
    let array = match bench_type {
        BenchType::Linear => other::linear_array(size.parse().unwrap()),
        BenchType::Reversed => other::reversed_array(size.parse().unwrap()),
        BenchType::Random => other::random_array(size.parse().unwrap()),
    };

    // Save the test data used for verification
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("./test_data/before/{}/{}.csv", bench_type, size))
        .expect("Couldn't open csv file");
    let mut writer = csv::Writer::from_writer(file);
    writer
        .write_record(array.iter().map(|x| x.to_string()))
        .expect("Couldn't write to csv");
    writer.flush().expect("Couldn't flush the writer");

    array
}

fn check_sorted(array: Vec<i32>) {
    // Save the data after sorting for verification
    // Merge on odd lines, selection on even lines
    let bench_type = BenchType::from_file(Path::new("./benches/bench_type.txt"));
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!(
            "./test_data/after/{}/{}.csv",
            bench_type,
            array.len()
        ))
        .expect("Couldn't open csv file");
    let mut writer = csv::Writer::from_writer(file);
    writer
        .write_record(array.iter().map(|x| x.to_string()))
        .expect("Couldn't write to csv");
    writer.flush().expect("Couldn't flush the writer");

    // Check the array was sorted, exit if not
    assert!(array.is_sorted(), "Array was not sorted!!");
}

// Benchmark the merge sort
#[library_benchmark]
#[benches::merge(file = "benches/lengths.txt", setup = get_test_array, teardown = check_sorted)]
fn bench_merge(mut array: Vec<i32>) -> Vec<i32> {
    black_box(sorting::merge_sort(&mut array));
    black_box(array)
}

// Benchmark the selection sort
#[library_benchmark]
#[benches::selection(file = "benches/lengths.txt", setup = get_test_array, teardown = check_sorted)]
fn bench_selection(mut array: Vec<i32>) -> Vec<i32> {
    black_box(sorting::selection_sort(&mut array));
    black_box(array)
}

// Run benchmarks
library_benchmark_group!(
    name = bench_sorting,
    benchmarks = [bench_merge, bench_selection]
);
main!(library_benchmark_groups = bench_sorting);
