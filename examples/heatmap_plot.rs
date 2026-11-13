//! Heatmap example: builds `SVHeatmapPlot` and writes
//! `docs/figures/heatmap_plot.svg`.
//!
//! Run with `cargo run --example heatmap_plot --features render-svg` from the
//! repository root. The figure is produced by the tool itself: the recipe
//! pivots the long table, derives the continuous colour scale, draws the cells
//! and the colour bar, and the public `to_svg()` method serializes the scene.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
use scienceview::sv_mark::SVFormatSpec;
use scienceview::sv_recipes::sv_matrix::sv_heatmap_plot::SVHeatmapPlot;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const LABELS: [&str; 6] = [
    "sepal width",
    "petal length",
    "petal width",
    "body mass",
    "bill length",
    "flipper length",
];

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    Ok(PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?))
}

/// A symmetric, plausible correlation matrix.
fn sample_value(row: usize, column: usize) -> f64 {
    if row == column {
        return 1.0;
    }
    let distance = row.abs_diff(column) as f64;
    let wave = ((row * 7 + column * 3) % 5) as f64 / 10.0;
    (0.85 - distance * 0.13 + wave).clamp(-0.9, 0.9)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rows = Vec::new();
    let mut columns = Vec::new();
    let mut values = Vec::new();
    for row in 0..LABELS.len() {
        for column in 0..LABELS.len() {
            rows.push(row as f64);
            columns.push(column as f64);
            values.push(sample_value(row, column));
        }
    }

    let table = SVDataTableBuilder::new()
        .with_column("row", SVColumnRole::Grouping, &rows)
        .with_column("column", SVColumnRole::Grouping, &columns)
        .with_column("correlation", SVColumnRole::Weight, &values)
        .build()?;

    let scene = SVHeatmapPlot::builder()
        .data_source(&table)
        .row("row")
        .column("column")
        .value("correlation")
        .annot(true)
        .annot_format(SVFormatSpec::Fixed(2))
        .centre(0.0)
        .colour_bar(true)
        .line_width(1.0)
        .missing(scienceview::sv_recipes::sv_matrix::sv_heatmap_plot::SVMissingValue::Neutral)
        .title("Correlation matrix of body measurements")
        .x_label("measurement")
        .y_label("measurement")
        .backend(SVBackendKind::Svg)
        .build()?
        .compose()?;

    let document = scene.to_svg()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("heatmap_plot.svg");
    fs::write(&destination, document)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
