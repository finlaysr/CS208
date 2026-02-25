use std::path::Path;
use std::process::Command;

mod graph_output;
mod parse_output;

const BENCH_TYPE: &str = "reversed";
fn main() {
    for i in 1..=10 {
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
            BENCH_TYPE, BENCH_TYPE, i
        );
        let graph_location = format!("./graphs/{}/graph-{}{}.png", BENCH_TYPE, BENCH_TYPE, i);
        //fs::create_dir_all(&csv_location).unwrap();
        //fs::create_dir_all(&graph_location).unwrap();

        parse_output::parse_output(Path::new(raw_data), Path::new(csv_location.as_str()));
        graph_output::plot(
            Path::new(csv_location.as_str()),
            Path::new(graph_location.as_str()),
        );
    }

    Command::new("uv")
        .current_dir("./py_plotting")
        .args(["run", "main.py"])
        .status()
        .expect("Failed to create graph");
}
