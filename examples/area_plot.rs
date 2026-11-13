//! Area-plot example: a stacked three-series area chart rendered to
//! `docs/figures/area_plot.svg`.
//!
//! Run with `cargo run --example area_plot` from the repository root. The
//! figure is produced by the tool itself: the recipe composes the data, the
//! stack, the scales and the marks, and the SVG backend serializes the
//! resulting primitives, exactly as a user program would do.

use scienceview::recipes::relational::SVAreaPlot;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_svg::SVSvgDocument;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const SAMPLE_COUNT: usize = 13;

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    let directory = std::env::var("CARGO_MANIFEST_DIR")?;
    Ok(PathBuf::from(directory))
}

fn main() -> Result<(), Box<dyn Error>> {
    let x: Vec<f64> = (0..SAMPLE_COUNT).map(|index| index as f64).collect();
    let baseline: Vec<f64> = (0..SAMPLE_COUNT)
        .map(|index| 2.0 + (index as f64 * 0.35).sin())
        .collect();
    let middle: Vec<f64> = (0..SAMPLE_COUNT)
        .map(|index| 1.4 + (index as f64 * 0.5).cos())
        .collect();
    let top: Vec<f64> = (0..SAMPLE_COUNT)
        .map(|index| 0.9 + (index as f64 * 0.8).sin() * 0.6)
        .collect();
    let mut all_x = Vec::new();
    let mut all_y = Vec::new();
    let mut all_series = Vec::new();
    for (code, values) in [&baseline, &middle, &top].into_iter().enumerate() {
        all_x.extend_from_slice(&x);
        all_y.extend_from_slice(values);
        all_series.extend(std::iter::repeat_n(code as f64, SAMPLE_COUNT));
    }

    let table = SVDataTable::builder()
        .with_column("month", SVColumnRole::X, &all_x)
        .with_column("revenue", SVColumnRole::Y, &all_y)
        .with_column("channel", SVColumnRole::Grouping, &all_series)
        .build()?;

    let plot = SVAreaPlot::new()
        .data_source(&table)
        .x("month")
        .y("revenue")
        .series("channel")
        .stack(true)
        .opacity(0.55)
        .line(true)
        .title("Stacked revenue by channel")
        .x_title("Month")
        .y_title("Revenue (thousands)")
        .backend(SVBackendKind::Svg)
        .build()?;
    let rendering = plot.rendering()?;

    let mut document =
        SVSvgDocument::new(rendering.viewport()).with_background(rendering.background());
    if let Some(title) = rendering.title() {
        document = document.with_title(title);
    }
    document.push_decoration_primitives(rendering.grid_and_frame());
    for primitive in rendering.primitives() {
        document.push_mark_primitive(primitive);
    }
    document.push_decoration_primitives(rendering.axes_and_title());

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("area_plot.svg");
    fs::write(&destination, document.to_svg())?;
    println!("Wrote {}", destination.display());
    Ok(())
}
