use Assignment_1_rs::other::BenchType;
use std::fs;
use std::path::Path;
use std::process::Command;

mod graph_output;
mod parse_output;

const BENCH_ITERATIONS: i32 = 10;

fn main() {
    let bench_type = BenchType::from_file(Path::new("./benches/bench_type.txt"));

    // Ensure all required directories exist and are clean
    for dir in [
        "graphs",
        "test_data/before",
        "test_data/after",
        "output_data",
    ] {
        fs::remove_dir_all(Path::new(&format!("./{}/{}", dir, bench_type))).ok();
        fs::create_dir_all(Path::new(&format!("./{}/{}", dir, bench_type))).unwrap();
    }
    fs::create_dir_all(Path::new("./py_plotting/graphs")).unwrap();

    // Run the benchmark a specified amount of times
    for i in 1..=BENCH_ITERATIONS {
        println!("Running benchmark iteration {i}");
        let status = Command::new("cargo")
            .arg("bench")
            .status()
            .expect("Failed to execute cargo bench");
        if !status.success() {
            eprintln!("cargo bench failed with exit code: {:?}", status.code());
            std::process::exit(1);
        }

        let raw_data = "./target/gungraun/Assignment-1-rs/library_benchmark/bench_sorting/";
        let csv_location = format!(
            "./output_data/{}/output-{}{}.csv",
            bench_type, bench_type, i
        );
        let graph_location = format!("./graphs/{}/graph-{}{}.png", bench_type, bench_type, i);

        parse_output::parse_output(Path::new(raw_data), Path::new(csv_location.as_str()));
        graph_output::plot(
            Path::new(csv_location.as_str()),
            Path::new(graph_location.as_str()),
        );
    }

    // Plot the results using a python script
    Command::new("uv")
        .current_dir("./py_plotting")
        .args(["run", "main.py"])
        .status()
        .expect("Failed to create graph");
}
