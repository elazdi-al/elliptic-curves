use plotters::prelude::*;
use std::ops::RangeInclusive;

// condition equation must satisfy discriminant != 0 to avoid self-intersections
// (https://en.wikipedia.org/wiki/Elliptic_curve)
const A: f32 = -1.0;
const B: f32 = 1.0;

const RANGE: f32 = 100000.0;
const REVERSE_ZOOM: f32 = 2.0;

fn elliptic(x: f32) -> Option<f32> {
    let result = x.powi(3) + A * x + B;
    if result >= 0.0 {
        Some(result.sqrt())
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plot/0.png", (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("y^2=x^3 + ax + b, a = {}, b = {}", A, B),
            ("sans-serif", 32).into_font(),
        )
        .margin(7)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-REVERSE_ZOOM..REVERSE_ZOOM, -REVERSE_ZOOM..REVERSE_ZOOM)?;

    let points: Vec<f32> = RangeInclusive::new(-RANGE as i32, RANGE as i32)
        .map(|x| x as f32 * REVERSE_ZOOM / RANGE)
        .collect();

    chart.draw_series(LineSeries::new(
        points.iter().filter_map(|&x| elliptic(x).map(|y| (x, y))),
        &RED,
    ))?;

    chart.draw_series(LineSeries::new(
        points.iter().filter_map(|&x| elliptic(x).map(|y| (x, -y))),
        &RED,
    ))?;
    chart.configure_mesh().draw()?;
    root.present()?;
    Ok(())
}
