//! Parallel-categories example: builds `SVParallelCategoriesPlot` and writes
//! `docs/figures/parallel_categories_plot.svg`.
//!
//! Run with `cargo run --example parallel_categories_plot --features render-svg`
//! from the repository root. The figure is produced by the tool itself: the
//! recipe counts every category through `SVStat::Count`, aggregates the
//! observations into ribbons between adjacent axes, places the bands through
//! nominal scales and draws the ribbons with the ribbon mark, and the public
//! `to_svg()` method serializes the scene it returns.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
use scienceview::sv_recipes::sv_multivariate::sv_parallel_categories_plot::SVParallelCategoriesPlot;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

/// Outcome codes: 0 = completed, 1 = withdrew, 2 = failed.
const OUTCOME: [f64; 24] = [
    0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 2.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 0.0, 0.0, 1.0,
    0.0, 0.0, 1.0, 0.0, 2.0,
];

/// Delivery codes: 0 = online, 1 = campus, 2 = hybrid.
const DELIVERY: [f64; 24] = [
    0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 2.0, 2.0, 2.0, 1.0, 1.0, 0.0, 0.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0,
    0.0, 0.0, 2.0, 2.0, 1.0,
];

/// Region codes: 0 = north, 1 = south, 2 = east.
const REGION: [f64; 24] = [
    0.0, 1.0, 0.0, 1.0, 2.0, 0.0, 1.0, 2.0, 1.0, 2.0, 0.0, 1.0, 2.0, 0.0, 1.0, 2.0, 0.0, 1.0, 2.0,
    0.0, 1.0, 2.0, 0.0, 1.0,
];

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    let directory = std::env::var("CARGO_MANIFEST_DIR")?;
    Ok(PathBuf::from(directory))
}

fn main() -> Result<(), Box<dyn Error>> {
    let table = SVDataTableBuilder::new()
        .with_column("outcome", SVColumnRole::Grouping, &OUTCOME)
        .with_column("delivery", SVColumnRole::Grouping, &DELIVERY)
        .with_column("region", SVColumnRole::Grouping, &REGION)
        .build()?;

    let scene = SVParallelCategoriesPlot::builder()
        .data_source(&table)
        .variables(&["delivery", "region", "outcome"])
        .variable_labels(&["Delivery", "Region", "Outcome"])
        .category_labels("delivery", &["online", "campus", "hybrid"])
        .category_labels("region", &["north", "south", "east"])
        .category_labels("outcome", &["completed", "withdrew", "failed"])
        .band_gap(0.24)
        .opacity(0.62)
        .title("Course outcomes across delivery, region and result")
        .backend(SVBackendKind::Svg)
        .build()?
        .compose()?;

    let document = scene.to_svg()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("parallel_categories_plot.svg");
    fs::write(&destination, document)?;
    println!(
        "Wrote {} ({} axes, {} ribbons)",
        destination.display(),
        scene.number_of_axes(),
        scene.number_of_ribbons()
    );
    Ok(())
}
