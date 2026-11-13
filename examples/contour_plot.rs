//! Contour example: builds `SVContourPlot` and writes
//! `docs/figures/contour_plot.svg`.
//!
//! Run with `cargo run --example contour_plot --features render-svg` from the
//! repository root. The recipe reads a row-major matrix as the scalar field,
//! isolates the requested levels by marching squares, paints the bands through
//! the continuous colour scale and serializes the composed scene itself.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
use scienceview::sv_recipes::sv_matrix::sv_contour_plot::{SVContourPlot, SVContourStyle};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const ROWS: usize = 41;
const COLUMNS: usize = 41;

/// A response surface over `[-3, 3]` on both axes: a central bump plus a
/// gentle periodic ripple, so the contours are neither a single circle nor a
/// straight gradient.
fn response_surface() -> Vec<f64> {
    let mut values = Vec::with_capacity(ROWS * COLUMNS);
    for row in 0..ROWS {
        let y = -3.0 + 6.0 * row as f64 / (ROWS - 1) as f64;
        for column in 0..COLUMNS {
            let x = -3.0 + 6.0 * column as f64 / (COLUMNS - 1) as f64;
            let bump = 3.0 * (-(x * x + y * y) / 2.0).exp();
            values.push(bump + x.sin() * y.cos());
        }
    }
    values
}

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    Ok(PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?))
}

fn main() -> Result<(), Box<dyn Error>> {
    let field = response_surface();
    let table = SVDataTableBuilder::new()
        .with_column("value", SVColumnRole::Y, &field)
        .build()?;

    let scene = SVContourPlot::builder()
        .data_source(&table)
        .value("value")
        .matrix(ROWS, COLUMNS)
        .x_range(-3.0, 3.0)
        .y_range(-3.0, 3.0)
        .levels(6)
        .style(SVContourStyle::Filled)
        .labels(true)
        .title("Response surface")
        .x_label("x")
        .y_label("y")
        .backend(SVBackendKind::Svg)
        .build()?
        .compose()?;

    let document = scene.to_svg()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("contour_plot.svg");
    fs::write(&destination, document)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
