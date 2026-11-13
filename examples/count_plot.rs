//! Count-plot example: builds `SVCountPlot` and writes
//! `docs/figures/count_plot.svg`.
//!
//! Run with `cargo run --example count_plot --features render-svg` from the
//! repository root. The figure is produced by the tool itself: the recipe
//! counts the category column through `SVStat::Count`, dodges the series
//! through `SVMove`, places the bars through the scales and the bar mark, and
//! the public `to_svg()` method serializes the scene it returns.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
use scienceview::sv_recipes::sv_distribution::sv_count_plot::{SVCountMultiple, SVCountPlot};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

/// Species codes: 0 = Adelie, 1 = Chinstrap, 2 = Gentoo.
const SPECIES: [f64; 30] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0,
    2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
];

/// Island codes: 0 = Biscoe, 1 = Dream, 2 = Torgersen.
const ISLAND: [f64; 30] = [
    0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
];

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    let directory = std::env::var("CARGO_MANIFEST_DIR")?;
    Ok(PathBuf::from(directory))
}

fn main() -> Result<(), Box<dyn Error>> {
    let table = SVDataTableBuilder::new()
        .with_column("species", SVColumnRole::Grouping, &SPECIES)
        .with_column("island", SVColumnRole::Grouping, &ISLAND)
        .build()?;

    let scene = SVCountPlot::builder()
        .data_source(&table)
        .category("species")
        .category_labels(&["Adelie", "Chinstrap", "Gentoo"])
        .series("island")
        .series_labels(&["Biscoe", "Dream", "Torgersen"])
        .multiple(SVCountMultiple::Dodge)
        .width(0.8)
        .title("Penguins observed per species and island")
        .x_label("species")
        .y_label("number of observations")
        .backend(SVBackendKind::Svg)
        .build()?
        .compose()?;

    let document = scene.to_svg()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("count_plot.svg");
    fs::write(&destination, document)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
