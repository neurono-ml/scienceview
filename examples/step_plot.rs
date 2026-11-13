//! Step-plot example: a sampled signal held constant between events.
//!
//! Run with `cargo run --example step_plot` from the repository root. The
//! figure is produced by the tool itself: the `SVStepPlot` recipe composes the
//! cores, the real SVG backend serializes the primitives, and the checked-in
//! `docs/figures/step_plot.svg` is the result.

use scienceview::SVStepPlot;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_svg::SVSvgDocument;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const TIME: [f64; 12] = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0];
const INDOOR: [f64; 12] = [
    18.0, 18.0, 19.5, 19.5, 21.0, 22.4, 22.4, 21.8, 20.5, 20.5, 19.6, 19.6,
];
const OUTDOOR: [f64; 12] = [
    12.0, 11.4, 11.4, 13.0, 13.0, 15.8, 17.1, 17.1, 16.2, 14.9, 14.9, 13.7,
];
const SENSOR: [f64; 24] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0,
];

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    let directory = std::env::var("CARGO_MANIFEST_DIR")?;
    Ok(PathBuf::from(directory))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut times = Vec::with_capacity(TIME.len() * 2);
    times.extend_from_slice(&TIME);
    times.extend_from_slice(&TIME);

    let mut values = Vec::with_capacity(INDOOR.len() + OUTDOOR.len());
    values.extend_from_slice(&INDOOR);
    values.extend_from_slice(&OUTDOOR);

    let table = SVDataTable::builder()
        .with_column("time", SVColumnRole::X, &times)
        .with_column("temperature", SVColumnRole::Y, &values)
        .with_column("sensor", SVColumnRole::Grouping, &SENSOR)
        .build()?;

    let plot = SVStepPlot::builder()
        .data_source(&table)
        .x("time")
        .y("temperature")
        .series("sensor")
        .title("Temperature held between samples")
        .backend(SVBackendKind::Svg)
        .build()?;
    let composition = plot.compose()?;

    let mut document = SVSvgDocument::new(composition.viewport())
        .with_description("Two sampled sensor traces drawn as step plots by scienceview")
        .with_background(composition.background());
    if let Some(title) = composition.title() {
        document = document.with_title(title);
    }
    document.push_decoration_primitives(composition.decorations_before_marks());
    for geometry in composition.mark_geometries()? {
        document.push_mark_geometry(&geometry);
    }
    document.push_decoration_primitives(composition.decorations_after_marks());

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("step_plot.svg");
    fs::write(&destination, document.to_bytes())?;
    println!("Wrote {}", destination.display());
    Ok(())
}
