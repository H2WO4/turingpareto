use plotly::color::NamedColor;
use plotly::common::{Marker, Mode};
use plotly::layout::{Axis, AxisType, LayoutScene};
use plotly::{Layout, Plot, Scatter, Scatter3D};

use crate::{csv, pareto};

pub fn generate(level: &str) {
    let records = pareto::filter_3d(csv::from_level(level));

    let sums: Vec<_> = {
        let min_sum = records.iter().map(|r| r.sum3()).min().unwrap();
        records
            .iter()
            .filter(|r| r.sum3() == min_sum)
            .cloned()
            .collect()
    };
    let products: Vec<_> = {
        let min_product = records.iter().map(|r| r.product3()).min().unwrap();
        records
            .iter()
            .filter(|r| r.product3() == min_product)
            .cloned()
            .collect()
    };

    // Initialize the plot
    let mut plot = Plot::new();
    plot.set_layout(
        Layout::new().title(level).width(1280).height(960).scene(
            LayoutScene::new()
                .x_axis(Axis::new().title("Gates"))
                .y_axis(Axis::new().title("Ticks"))
                .z_axis(Axis::new().title("Delay")),
        ),
    );

    // Draw the frontier
    let frontier = {
        let xs = records.iter().map(|r| r.gates).collect();
        let ys = records.iter().map(|r| r.ticks).collect();
        let zs = records.iter().map(|r| r.delay).collect();

        Scatter3D::new(xs, ys, zs)
            .name("")
            .show_legend(false)
            .mode(Mode::Markers)
            .marker(Marker::new().size(10))
    };

    // Draw the sum record
    let sum = {
        let xs = sums.iter().map(|r| r.gates).collect();
        let ys = sums.iter().map(|r| r.ticks).collect();
        let zs = sums.iter().map(|r| r.delay).collect();

        Scatter3D::new(xs, ys, zs)
            .name("Sum")
            .mode(Mode::Markers)
            .marker(Marker::new().size(10).color(NamedColor::Red))
    };

    // Draw the product record
    let product = {
        let xs = products.iter().map(|r| r.gates).collect();
        let ys = products.iter().map(|r| r.ticks).collect();
        let zs = products.iter().map(|r| r.delay).collect();

        Scatter3D::new(xs, ys, zs)
            .name("Product")
            .mode(Mode::Markers)
            .marker(Marker::new().size(10).color(NamedColor::Purple))
    };

    plot.add_traces(vec![frontier, sum, product]);
    plot.write_html(format!("images/{level}.html"));
}
