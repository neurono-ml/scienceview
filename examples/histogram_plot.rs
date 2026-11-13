//! Histogram example: builds `SVHistogramPlot` and writes
//! `docs/figures/histogram_plot.svg`.
//!
//! Run with `cargo run --example histogram_plot --features render-svg` from the
//! repository root. The figure is produced by the tool itself: the recipe bins
//! the column through `SVStat::Bin`, stacks the series through `SVMove`, places
//! the bars through the scales and the bar mark, and the public `to_svg()`
//! method serializes the scene it returns.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
use scienceview::sv_recipes::sv_distribution::sv_histogram_plot::{
    SVHistogramMultiple, SVHistogramPlot,
};
use scienceview::sv_stat::SVNormalisation;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const SEPAL_WIDTH: [f64; 36] = [
    2.9, 3.0, 3.1, 3.1, 3.2, 3.2, 3.3, 3.4, 3.4, 3.5, 3.6, 3.7, 2.5, 2.6, 2.7, 2.8, 2.8, 2.9, 3.0,
    3.0, 3.1, 3.2, 3.3, 3.4, 2.6, 2.8, 2.9, 3.0, 3.0, 3.1, 3.1, 3.2, 3.2, 3.3, 3.4, 3.5,
];

const SPECIES: [f64; 36] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
];

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    let directory = std::env::var("CARGO_MANIFEST_DIR")?;
    Ok(PathBuf::from(directory))
}

fn main() -> Result<(), Box<dyn Error>> {
    let table = SVDataTableBuilder::new()
        .with_column("sepal_width", SVColumnRole::X, &SEPAL_WIDTH)
        .with_column("species", SVColumnRole::Grouping, &SPECIES)
        .build()?;

    let scene = SVHistogramPlot::builder()
        .data_source(&table)
        .value("sepal_width")
        .series("species")
        .series_labels(&["setosa", "versicolor", "virginica"])
        .bin_count(10)
        .normalise(SVNormalisation::Probability)
        .cumulative(false)
        .multiple(SVHistogramMultiple::Stack)
        .shrink(0.95)
        .title("Sepal width by species")
        .x_label("sepal width (cm)")
        .y_label("probability")
        .backend(SVBackendKind::Svg)
        .build()?
        .compose()?;

    let document = scene.to_svg()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("histogram_plot.svg");
    fs::write(&destination, document)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
