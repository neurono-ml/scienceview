//! Image example: builds `SVImagePlot` and writes
//! `docs/figures/image_plot.svg`.
//!
//! Run with `cargo run --example image_plot --features render-svg` from the
//! repository root. The raster is computed here from a deterministic analytic
//! field — no external asset is read — so the figure is reproducible. The
//! recipe maps the values through the default colour map, places them on their
//! data extent and the public `to_svg()` method serializes the scene; this
//! program contains no serializer of its own.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
use scienceview::sv_recipes::sv_matrix::sv_image_plot::{
    SVImageExtent, SVImagePlot, SVImageSource, SVRasterDimensions,
};
use scienceview::sv_scale::sv_default_colour_map;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const ROWS: usize = 36;
const COLUMNS: usize = 48;

/// A two-dimensional field: one Gaussian bump plus a damped ripple.
fn field_value(row: usize, column: usize) -> f64 {
    let x = column as f64 / (COLUMNS - 1) as f64;
    let y = row as f64 / (ROWS - 1) as f64;
    let bump_x = x - 0.38;
    let bump_y = y - 0.42;
    let bump = (-(bump_x * bump_x + bump_y * bump_y) / 0.035).exp();
    let ripple = 0.35 * (12.0 * x * std::f64::consts::PI).sin() * (1.0 - y);
    bump + ripple
}

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    let directory = std::env::var("CARGO_MANIFEST_DIR")?;
    Ok(PathBuf::from(directory))
}

fn main() -> Result<(), Box<dyn Error>> {
    let values: Vec<f64> = (0..ROWS)
        .flat_map(|row| (0..COLUMNS).map(move |column| field_value(row, column)))
        .collect();
    let data = SVDataTableBuilder::new()
        .with_column("intensity", SVColumnRole::Weight, &values)
        .build()?;
    let dimensions = SVRasterDimensions::new(ROWS, COLUMNS)?;

    let scene = SVImagePlot::builder()
        .data_source(&data)
        .raster(SVImageSource::intensity("intensity", dimensions))
        .colour_map(sv_default_colour_map())
        .extent(SVImageExtent::new(0.0, COLUMNS as f64, 0.0, ROWS as f64)?)
        .colour_bar(true)
        .title("A computed field shown as an image")
        .x_label("column")
        .y_label("row")
        .backend(SVBackendKind::Svg)
        .build()?
        .compose()?;

    let document = scene.to_svg()?;
    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("image_plot.svg");
    fs::write(&destination, document)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
