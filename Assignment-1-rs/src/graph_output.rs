use csv::Reader;
use plotters::prelude::*;
use std::iter::zip;
use std::path::Path;

struct Data {
    lengths: Vec<i32>,
    selection: Vec<i32>,
    merge: Vec<i32>,
}
impl Data {
    fn new() -> Self {
        Data {
            lengths: vec![],
            selection: vec![],
            merge: vec![],
        }
    }
}

pub fn plot(path: &Path) {
    let data = read_data(path);
    let drawing_area = BitMapBackend::new("graph.png", (1000, 750)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&drawing_area)
        .margin(10)
        .caption("Instructions for Selection and Merge Sort", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0..100, 0..70_000)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            zip(data.lengths.clone(), data.selection.clone()),
            RED,
        ))
        .unwrap()
        .label("Selection Sort")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .draw_series(LineSeries::new(
            zip(data.lengths.clone(), data.merge.clone()),
            BLUE,
        ))
        .unwrap()
        .label("insertion Sort")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    chart
        .configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.8))
        .draw()
        .unwrap();
}

fn read_data(path: &Path) -> Data {
    let mut data = Data::new();
    let mut reader = Reader::from_path(path).unwrap();
    reader.records().map(|r| r.unwrap()).for_each(|r| {
        data.lengths.push(r[0].parse().unwrap());
        data.selection.push(r[1].parse().unwrap());
        data.merge.push(r[2].parse().unwrap());
    });
    data
}
