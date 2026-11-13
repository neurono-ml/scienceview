//! Swarm example: builds `SVSwarmPlot` and writes `docs/figures/swarm_plot.svg`.
//!
//! Run with `cargo run --example swarm_plot --features render-svg` from the
//! repository root. The figure is produced by the tool itself: the recipe
//! resolves the columns, lays every category out with `SVMove::Swarm` and the
//! public `to_svg()` method serializes the scene it returns — this program
//! contains no serializer of its own.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
use scienceview::sv_recipes::sv_distribution::sv_swarm_plot::SVSwarmPlot;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const BILL_LENGTH: [f64; 60] = [
    37.8, 38.2, 38.2, 39.0, 39.0, 39.0, 40.1, 40.5, 41.3, 41.9, 39.2, 39.8, 40.2, 40.8, 41.1, 41.7,
    42.1, 42.6, 43.0, 43.6, 41.5, 42.0, 42.4, 42.9, 43.3, 43.8, 44.2, 44.7, 45.1, 45.6, 44.3, 44.9,
    45.4, 45.9, 46.3, 46.8, 47.2, 47.7, 48.1, 48.6, 39.0, 39.6, 40.1, 40.7, 41.2, 41.8, 42.3, 42.9,
    43.4, 44.0, 44.5, 45.2, 45.8, 46.4, 47.0, 47.6, 48.2, 48.8, 49.4, 50.0,
];
const SPECIES: [f64; 60] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
    2.0, 2.0, 2.0,
];
const SEX: [f64; 60] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0,
];

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    let directory = std::env::var("CARGO_MANIFEST_DIR")?;
    Ok(PathBuf::from(directory))
}

fn main() -> Result<(), Box<dyn Error>> {
    let table = SVDataTableBuilder::new()
        .with_column("bill_length", SVColumnRole::Y, &BILL_LENGTH)
        .with_column("species", SVColumnRole::Grouping, &SPECIES)
        .with_column("sex", SVColumnRole::Grouping, &SEX)
        .build()?;

    let scene = SVSwarmPlot::builder()
        .data_source(&table)
        .value("bill_length")
        .category("species")
        .series("sex")
        .series_labels(&["female", "male"])
        .alpha(0.85)
        .title("Bill length of penguins by species")
        .x_label("species")
        .y_label("bill length (mm)")
        .backend(SVBackendKind::Svg)
        .build()?
        .compose()?;

    let document = scene.to_svg()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("swarm_plot.svg");
    fs::write(&destination, document)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
