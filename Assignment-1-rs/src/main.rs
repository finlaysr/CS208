use std::path::Path;
use std::process::Command;

mod graph_output;
mod parse_output;

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
        let csv_location = format!("./output_data/output-random{}.csv", i);
        let graph_location = format!("./graphs/graph-random{}.png", i);

        parse_output::parse_output(Path::new(raw_data), Path::new(csv_location.as_str()));
        graph_output::plot(
            Path::new(csv_location.as_str()),
            Path::new(graph_location.as_str()),
        );
    }
    // Run cargo bench
}
