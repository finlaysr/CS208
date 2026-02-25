use std::path::Path;

pub mod sorting;

mod parse_output;

fn main() {
    let path = Path::new("./target/gungraun/Assignment-1-rs/library_benchmark/bench_sorting/");
    parse_output::parse_output(path);
}
