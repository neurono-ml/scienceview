//! Band-plot example: two confidence bands derived from observations, written
//! to `docs/figures/band_plot.svg`.
//!
//! Run with `cargo run --example band_plot` from the repository root. The
//! figure is produced by the tool itself: the real `SVBandPlotBuilder`
//! composes the bands and the public `to_svg()` serializer writes the very
//! geometry the recipe produced.

use scienceview::sv_recipes::sv_relational::sv_band_plot::SVBandPlotBuilder;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_layout::SVViewport;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const SAMPLE_COUNT: usize = 24;

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    Ok(PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?))
}

fn sample(index: usize) -> (f64, f64, f64) {
    let x = (index % 6) as f64;
    let series = if index < SAMPLE_COUNT / 2 { 1.0 } else { 2.0 };
    let base = if series == 1.0 { 2.0 } else { 4.0 };
    let wobble = ((index as f64) * 1.7).sin();
    let value = base + x * 0.6 + wobble * (0.4 + x * 0.05);
    (x, value, series)
}

fn main() -> Result<(), Box<dyn Error>> {
    let samples: Vec<(f64, f64, f64)> = (0..SAMPLE_COUNT).map(sample).collect();
    let x: Vec<f64> = samples.iter().map(|sample| sample.0).collect();
    let values: Vec<f64> = samples.iter().map(|sample| sample.1).collect();
    let series: Vec<f64> = samples.iter().map(|sample| sample.2).collect();

    let table = SVDataTable::builder()
        .with_column("time", SVColumnRole::X, &x)
        .with_column("load", SVColumnRole::Y, &values)
        .with_column("scenario", SVColumnRole::Grouping, &series)
        .build()?;

    let output = SVBandPlotBuilder::new()
        .data_source(&table)
        .x("time")
        .derive()
        .values("load")
        .series("scenario")
        .border(true)
        .centre_line(true)
        .title("Load interval by scenario")
        .x_title("time")
        .y_title("load")
        .viewport(SVViewport::new(640.0, 400.0))
        .backend(SVBackendKind::Svg)
        .build()?
        .render()?;

    let document = output.to_svg()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("band_plot.svg");
    fs::write(&destination, document)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
