//! Regression example: builds the real chart and writes
//! `docs/figures/regression_plot.svg` through the public render path.
//!
//! Run with `cargo run --example regression_plot --features render-svg` from the
//! repository root. The figure is checked in and embedded in `README.md` and
//! the mdBook, so the picture shown in the documentation is produced by the
//! tool itself: data source → `SVStat::Fit` → scales → ribbon, line and dot
//! marks → decorations → SVG serialization. The example never re-implements the
//! serializer: it calls `SVRegressionPlot::save`, the same operation a user
//! program calls.

use scienceview::sv_recipes::sv_relational::SVRegressionPlotBuilder;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const FIRST_X: [f64; 12] = [1.2, 1.6, 1.9, 2.3, 2.7, 3.1, 3.4, 3.8, 4.2, 4.6, 5.0, 5.4];
const SECOND_X: [f64; 12] = [1.4, 1.8, 2.2, 2.6, 3.0, 3.4, 3.8, 4.2, 4.6, 5.0, 5.4, 5.8];
const FIRST_Y: [f64; 12] = [2.1, 2.6, 2.9, 3.6, 4.0, 4.8, 5.1, 5.8, 6.3, 6.9, 7.4, 7.9];
const SECOND_Y: [f64; 12] = [1.4, 1.9, 2.5, 2.8, 3.5, 3.9, 4.6, 5.0, 5.4, 6.1, 6.5, 7.2];

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    let directory = std::env::var("CARGO_MANIFEST_DIR")?;
    Ok(PathBuf::from(directory))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut x = FIRST_X.to_vec();
    x.extend_from_slice(&SECOND_X);
    let mut y = FIRST_Y.to_vec();
    y.extend_from_slice(&SECOND_Y);
    let mut series = vec![0.0; FIRST_X.len()];
    series.extend(std::iter::repeat_n(1.0, SECOND_X.len()));

    let table = SVDataTable::builder()
        .with_column("petal length", SVColumnRole::X, &x)
        .with_column("sepal width", SVColumnRole::Y, &y)
        .with_column("species", SVColumnRole::Grouping, &series)
        .build()?;

    let plot = SVRegressionPlotBuilder::new()
        .data_source(&table)
        .x("petal length")
        .y("sepal width")
        .series("species")
        .confidence(0.95)
        .title("Sepal width against petal length")
        .backend(SVBackendKind::Svg)
        .build()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("regression_plot.svg");
    plot.save(&destination)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
