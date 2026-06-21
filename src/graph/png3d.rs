use std::f64;

use plotters::prelude::*;

use crate::{csv, pareto};

pub fn generate(level: &str) {
    let records: Vec<_> = {
        let vals = pareto::filter_3d(csv::from_level(level));
        let sum = vals
            .iter()
            .min_by_key(|r| r.gates + r.delay + r.ticks)
            .cloned();
        let product = vals
            .iter()
            .min_by_key(|r| r.gates * r.delay * r.ticks)
            .cloned();

        vals.into_iter()
            .map(|r| {
                (
                    r,
                    if sum.is_some_and(|s| s == r) {
                        RED
                    } else if product.is_some_and(|p| p == r) {
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
    let max_ticks = {
        let val = records.iter().map(|r| r.0.ticks).max().unwrap_or(0);
        val + (val / 10) + 1
    };


    // Initialize file
    let filename = format!("images/{level}.png");
    let drawing_area = BitMapBackend::new(&filename, (1920, 1080)).into_drawing_area();
    let drawing_area = drawing_area;

    // Setup graphing area
    let mut ctx = ChartBuilder::on(&drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .margin(40)
        .caption(level, ("sans-serif", 40))
        .build_cartesian_3d(0..max_width, 0..max_height, 0..max_ticks)
        .unwrap();

    drawing_area.fill(&WHITE).unwrap();

    // Setup perspective
    ctx.with_projection(|mut p| {
        p.pitch = 0.2;
        p.yaw = f64::consts::PI / 6.;
        p.scale = 1.0;
        p.into_matrix()
    });

    // Setup axes
    ctx.configure_axes().tick_size(16).draw().unwrap();

    ctx.draw_series([Circle::new((0, 0, 0), 0, WHITE)])
        .unwrap()
        .label("Sum")
        .legend(|pos| Circle::new(pos, 3, RED.filled()));

    // Draw points
    ctx.draw_series(
        records
            .iter()
            .map(|r| Circle::new((r.0.gates, r.0.delay, r.0.ticks), 10, r.1.filled())),
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
