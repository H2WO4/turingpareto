use plotters::prelude::*;

use crate::{csv, pareto};

pub fn generate(level: &str) {
    let records = pareto::filter(csv::from_level(level));

    let max_width = records.iter().map(|r| r.gates).max().unwrap_or(0);
    let max_height = records.iter().map(|r| r.delay).max().unwrap_or(0);

    let max_width = max_width + (max_width / 10);
    let max_height = max_height + (max_height / 10);

    let filename = format!("images/{level}.png");
    let drawing_area = BitMapBackend::new(&filename, (1024, 768)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(level, ("sans-serif", 40))
        .build_cartesian_2d(0..max_width, 0..max_height)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        records
            .into_iter()
            .map(|val| Circle::new((val.gates, val.delay), 3, BLUE)),
    )
    .unwrap();
}
