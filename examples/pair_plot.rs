//! Pair-grid example: builds `SVPairPlot` and writes
//! `docs/figures/pair_plot.svg`.
//!
//! Run with `cargo run --example pair_plot --features render-svg` from the
//! repository root. The figure is produced by the tool itself: the recipe
//! composes every cell, the shared scales and the hue legend, and the public
//! `to_svg()` method serializes the scene — this program contains no serializer
//! of its own.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
use scienceview::sv_recipes::sv_multivariate::sv_pair_plot::{
    SVPairDiagonalKind, SVPairOffDiagonalKind, SVPairPlot,
};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const SEPAL_LENGTH: [f64; 24] = [
    5.1, 4.9, 4.7, 5.0, 5.4, 4.6, 5.0, 5.2, 7.0, 6.4, 6.9, 5.5, 6.5, 5.7, 6.3, 5.8, 6.4, 6.5, 6.8,
    5.7, 5.8, 6.4, 6.5, 6.0,
];
const SEPAL_WIDTH: [f64; 24] = [
    3.5, 3.0, 3.2, 3.6, 3.9, 3.4, 3.4, 3.5, 3.2, 3.2, 3.1, 2.3, 2.8, 2.8, 3.3, 2.7, 3.2, 3.0, 3.0,
    2.5, 2.7, 2.9, 3.0, 2.2,
];
const PETAL_LENGTH: [f64; 24] = [
    1.4, 1.4, 1.3, 1.5, 1.7, 1.4, 1.5, 1.5, 4.7, 4.5, 4.9, 4.0, 4.6, 4.5, 4.7, 4.1, 4.5, 4.6, 4.8,
    3.9, 4.0, 4.2, 4.4, 4.0,
];
const SPECIES: [f64; 24] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0,
    2.0, 2.0, 2.0, 2.0, 2.0,
];

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    let directory = std::env::var("CARGO_MANIFEST_DIR")?;
    Ok(PathBuf::from(directory))
}

fn main() -> Result<(), Box<dyn Error>> {
    let table = SVDataTableBuilder::new()
        .with_column("sepal_length", SVColumnRole::X, &SEPAL_LENGTH)
        .with_column("sepal_width", SVColumnRole::Y, &SEPAL_WIDTH)
        .with_column("petal_length", SVColumnRole::Y, &PETAL_LENGTH)
        .with_column("species", SVColumnRole::Grouping, &SPECIES)
        .build()?;

    let scene = SVPairPlot::builder()
        .data_source(&table)
        .variables(&["sepal_length", "sepal_width", "petal_length"])
        .hue("species")
        .diagonal_kind(SVPairDiagonalKind::Density)
        .off_diagonal_kind(SVPairOffDiagonalKind::Scatter)
        .title("Iris measurements by species")
        .backend(SVBackendKind::Svg)
        .build()?
        .compose()?;

    let document = scene.to_svg()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("pair_plot.svg");
    fs::write(&destination, document)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
