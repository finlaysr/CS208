use std::fs::{self};
use std::path::Path;

pub fn parse_output(path: &Path, output_location: &Path) {
    let lengths = read_lenghts(Path::new("./benches/lengths.txt"));
    println!("lens: {:?}", lengths);
    let mut merge_data = vec![];
    let mut selection_data = vec![];

    fs::read_dir(path)
        .expect("No data logged. Make sure you have run `cargo bench` first!")
        .for_each(|folder| {
            fs::read_dir(folder.unwrap().path())
                .unwrap()
                .map(|file| file.unwrap())
                .for_each(|file| {
                    if file.path().extension().unwrap() == "log" {
                        let name = file.file_name().into_string().unwrap();
                        let algo: &str = name
                            .split(".")
                            .nth(2)
                            .unwrap()
                            .split("_")
                            .collect::<Vec<&str>>()
                            .first()
                            .unwrap();
                        let iterations = get_iterations(fs::read_to_string(file.path()).unwrap());
                        if algo == "merge" {
                            merge_data.push(iterations);
                        } else {
                            selection_data.push(iterations);
                        }
                    }
                });
        });
    // Remove data after its read to reduce risk of data contamination from caching
    fs::remove_dir_all(path).unwrap();

    //write to CSV
    let mut writer = csv::Writer::from_path(output_location).unwrap();
    writer
        .write_record(["Array Length", "Selection", "Merge"])
        .unwrap();
    for i in 0..lengths.len() {
        writer
            .write_record(&[
                lengths[i].to_string(),
                selection_data[i].to_string(),
                merge_data[i].to_string(),
            ])
            .unwrap();
        writer.flush().expect("Couldn't flush the writer");
    }
}

fn read_lenghts(path: &Path) -> Vec<u32> {
    println!("path: {:?}", path);
    fs::read_to_string(path)
        .expect("Couldn't find the lengths file!")
        .lines()
        .map(|c| c.parse().unwrap())
        .collect()
}

fn get_iterations(text: String) -> u32 {
    let line = text
        .split("\n")
        .find(|line| line.contains("I   refs:"))
        .expect("Couln't find iterations line!")
        .split(":")
        .collect::<Vec<&str>>();
    line[1].trim().replace(",", "").parse().unwrap()
}
