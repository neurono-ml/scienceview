//! Bar-plot example: the aggregate value of each category and series, written
//! to `docs/figures/bar_plot.svg`.
//!
//! Run with `cargo run --example bar_plot` from the repository root. The figure
//! is produced by the tool itself: the real `SVBarPlotBuilder` aggregates the
//! observations, arranges the series and the public `to_svg()` serializer
//! writes the very geometry the recipe produced.
//!
//! A caller that already has one aggregate value per category can pass exactly
//! one row per category; the mean of one observation is that observation, so
//! the same builder draws a pre-aggregated column unchanged.

use scienceview::sv_recipes::sv_distribution::sv_bar_plot::SVBarPlotBuilder;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_stat::{SVAggregateErrorKind, SVAggregateMethod};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const CATEGORY_COUNT: usize = 3;
const SERIES_COUNT: usize = 2;
const OBSERVATIONS_PER_GROUP: usize = 12;

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    Ok(PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?))
}

fn sample_value(category: usize, series: usize, observation: usize) -> f64 {
    let base = 3.0 + category as f64 * 1.6 + series as f64 * 2.4;
    let wobble = ((observation as f64) * 1.3 + category as f64).sin();
    base + wobble * 0.8
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut values = Vec::new();
    let mut categories = Vec::new();
    let mut series = Vec::new();
    for category in 1..=CATEGORY_COUNT {
        for group in 1..=SERIES_COUNT {
            for observation in 0..OBSERVATIONS_PER_GROUP {
                values.push(sample_value(category, group, observation));
                categories.push(category as f64);
                series.push(group as f64);
            }
        }
    }

    let table = SVDataTable::builder()
        .with_column("measurement", SVColumnRole::Y, &values)
        .with_column("station", SVColumnRole::Grouping, &categories)
        .with_column("scenario", SVColumnRole::Grouping, &series)
        .build()?;

    let output = SVBarPlotBuilder::new()
        .data_source(&table)
        .value("measurement")
        .category("station")
        .series("scenario")
        .estimator(SVAggregateMethod::Mean)
        .error(SVAggregateErrorKind::StandardError)
        .width(0.7)
        .label(true)
        .title("Mean measurement by station and scenario")
        .x_title("station")
        .y_title("mean measurement")
        .backend(SVBackendKind::Svg)
        .build()?
        .render()?;

    let document = output.to_svg()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("bar_plot.svg");
    fs::write(&destination, document)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
