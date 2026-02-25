use std::path::Path;

mod graph_output;
mod parse_output;

fn main() {
    let path = Path::new("./target/gungraun/Assignment-1-rs/library_benchmark/bench_sorting/");
    parse_output::parse_output(path);
    graph_output::plot(Path::new("./output.csv"));
}
