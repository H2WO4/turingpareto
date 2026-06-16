use plotters::prelude::*;

use crate::{csv, list, pareto};

pub fn handle(level: &str) {
    if list::LEVELS_2D.contains(&level) {
        generate_2d(level);
    } else if list::LEVELS_3D.contains(&level) {
        todo!()
    } else {
        eprintln!("Unknown level ID!")
    }
}

fn generate_2d(level: &str) {
    let records: Vec<_> = {
        let vals = pareto::filter(csv::from_level(level));
        let sum = *vals.iter().min_by_key(|r| r.gates + r.delay).unwrap();
        let product = *vals.iter().min_by_key(|r| r.gates * r.delay).unwrap();

        vals.into_iter()
            .map(|r| {
                (
                    r,
                    if r == sum {
                        RED
                    } else if r == product {
                        MAGENTA
                    } else {
                        BLUE
                    },
                )
            })
            .collect()
    };

    let max_width = {
        let val = records.iter().map(|r| r.0.gates).max().unwrap_or(0);
        val + (val / 10) + 1
    };
    let max_height = {
        let val = records.iter().map(|r| r.0.delay).max().unwrap_or(0);
        val + (val / 10) + 1
    };


    // Initialize file
    let filename = format!("images/{level}.png");
    let drawing_area = BitMapBackend::new(&filename, (1920, 1080)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    // Setup graphing area
    let mut ctx = ChartBuilder::on(&drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(level, ("sans-serif", 40))
        .build_cartesian_2d(0..max_width, 0..max_height)
        .unwrap();

    // Setup axises
    ctx.configure_mesh()
        .x_desc("Gates")
        .y_desc("Delay")
        .axis_desc_style(("sans-serif", 20))
        .draw()
        .unwrap();

    // Draw line
    ctx.draw_series(LineSeries::new(records.iter().map(|r| (r.0.gates, r.0.delay)), BLUE))
        .unwrap()
        .label("Sum")
        .legend(|pos| Circle::new(pos, 3, RED.filled()));

    // Draw points
    ctx.draw_series(
        records
            .iter()
            .map(|r| Circle::new((r.0.gates, r.0.delay), 5, r.1.filled())),
    )
    .unwrap()
    .label("Product")
    .legend(|pos| Circle::new(pos, 3, MAGENTA.filled()));


    // Draw legend
    ctx.configure_series_labels()
        .border_style(BLACK)
        .position(SeriesLabelPosition::UpperRight)
        .background_style(WHITE)
        .draw()
        .unwrap();
}
