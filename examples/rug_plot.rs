//! Rug example: builds `SVRugPlot` and writes `docs/figures/rug_plot.svg`.
//!
//! Run with `cargo run --example rug_plot --features render-svg` from the
//! repository root. The figure is produced by the tool itself: the recipe
//! composes the real scale, line marks and decorations, and the public
//! `to_svg()` method serializes the scene it returns — this program contains no
//! serializer of its own.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
use scienceview::sv_recipes::sv_distribution::sv_rug_plot::SVRugPlot;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const BODY_MASS: [f64; 40] = [
    36.2, 37.1, 38.4, 39.0, 39.6, 40.1, 40.4, 40.9, 41.2, 41.8, 42.0, 42.3, 42.9, 43.1, 43.6, 44.0,
    44.2, 44.7, 45.0, 45.4, 45.9, 46.2, 46.5, 47.0, 47.4, 47.9, 48.2, 48.7, 49.1, 49.5, 50.0, 50.6,
    51.1, 51.8, 52.4, 53.0, 53.7, 54.4, 55.2, 56.1,
];
const SPECIES: [f64; 40] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0,
];

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    let directory = std::env::var("CARGO_MANIFEST_DIR")?;
    Ok(PathBuf::from(directory))
}

fn main() -> Result<(), Box<dyn Error>> {
    let table = SVDataTableBuilder::new()
        .with_column("body_mass", SVColumnRole::X, &BODY_MASS)
        .with_column("species", SVColumnRole::Grouping, &SPECIES)
        .build()?;

    let scene = SVRugPlot::builder()
        .data_source(&table)
        .value("body_mass")
        .series("species")
        .series_labels(&["Adelie", "Gentoo"])
        .height(0.08)
        .alpha(0.85)
        .title("Body mass of penguins")
        .value_label("body mass (g)")
        .backend(SVBackendKind::Svg)
        .build()?
        .compose()?;

    let document = scene.to_svg()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("rug_plot.svg");
    fs::write(&destination, document)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
