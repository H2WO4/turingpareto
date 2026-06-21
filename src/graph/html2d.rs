use plotly::color::NamedColor;
use plotly::common::{Marker, Mode};
use plotly::layout::Axis;
use plotly::{Layout, Plot, Scatter};

use crate::{csv, pareto};

pub fn generate(level: &str) {
    let vals = pareto::filter_2d(csv::from_level(level));

    let sum: Vec<_> = {
        let min_sum = vals.iter().map(|r| r.sum2()).min().unwrap();
        vals.iter()
            .filter(|r| r.sum2() == min_sum)
            .cloned()
            .collect()
    };
    let product: Vec<_> = {
        let min_product = vals.iter().map(|r| r.product2()).min().unwrap();
        vals.iter()
            .filter(|r| r.product2() == min_product)
            .cloned()
            .collect()
    };

    let records: Vec<_> = vals
        .into_iter()
        .filter(|r| !sum.contains(r) && !product.contains(r))
        .collect();

    // Initialize the plot
    let mut plot = Plot::new();

    plot.set_layout(
        Layout::new()
            .title(level)
            .x_axis(Axis::new().title("Gates"))
            .y_axis(Axis::new().title("Delay")),
    );

    // Draw the frontier
    let frontier = {
        let xs = records.iter().map(|r| r.gates).collect();
        let ys = records.iter().map(|r| r.delay).collect();

        Scatter::new(xs, ys)
            .name("")
            .show_legend(false)
            .mode(Mode::Markers)
            .marker(Marker::new().size(10))
    };

    // Draw the sum record
    let sum = {
        let xs = sum.iter().map(|r| r.gates).collect();
        let ys = sum.iter().map(|r| r.delay).collect();

        Scatter::new(xs, ys)
            .name("Sum")
            .mode(Mode::Markers)
            .marker(Marker::new().size(10).color(NamedColor::Red))
    };

    // Draw the product record
    let product = {
        let xs = product.iter().map(|r| r.gates).collect();
        let ys = product.iter().map(|r| r.delay).collect();

        Scatter::new(xs, ys)
            .name("Product")
            .mode(Mode::Markers)
            .marker(Marker::new().size(10).color(NamedColor::Purple))
    };

    plot.add_traces(vec![frontier, sum, product]);
    plot.write_html(format!("images/{level}.html"));
}
