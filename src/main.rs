use std::ops::RangeInclusive;

use plotters::prelude::*;
// A and B must satisfy discriminant != 0 to avoid self-intersections
// (https://en.wikipedia.org/wiki/Elliptic_curve)
const A: f32 = -2.0;
const B: f32 = 2.0;
const RANGE: std::ops::Range<f32> = -2500f32..2500f32; //lazy

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plot/0.png", (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "y^2=x^3 + ax + b, a = 3, b = 1",
            ("sans-serif", 32).into_font(),
        )
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

    let result = chart.draw_series(LineSeries::new(
        RangeInclusive::new(RANGE.start as i32, RANGE.end as i32)
            .map(|x| x as f32 / RANGE.end)
            .map(|x| (x, (x * x * x + A * x + B).sqrt())),
        &RED,
    ));
    if result.is_ok() {
        chart.configure_mesh().draw()?;
        root.present()?;
        Ok(())
    } else {
        eprint!("couldn't plot function");
        Ok(())
    }
}
