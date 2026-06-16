use plotters::prelude::*;

use crate::csv;

pub fn generate(level: &str) {
    let records = csv::from_level(level).filter(|&record| record.gates < 1024 && record.delay < 768);

    let filename = format!("images/{level}.png");
    let drawing_area = BitMapBackend::new(&filename, (1024, 768)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(level, ("sans-serif", 40))
        .build_cartesian_2d(0..1024u64, 0..768u64)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(records.map(|val| Circle::new((val.gates, val.delay), 3, BLUE)))
        .unwrap();
}
