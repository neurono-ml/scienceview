//! Point-plot example: the point estimate of each category with its
//! uncertainty, written to `docs/figures/point_plot.svg`.
//!
//! Run with `cargo run --example point_plot --features render-svg` from the
//! repository root. The figure is produced by the tool itself: the real
//! `SVPointPlotBuilder` aggregates the observations, dodges the series, joins
//! the points of one series with a line and the public `to_svg()` serializer
//! writes the very geometry the recipe produced.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::{SVColumnRole, SVDataTable};
use scienceview::sv_mark::SVDashPattern;
use scienceview::sv_recipes::sv_distribution::sv_point_plot::SVPointPlotBuilder;
use scienceview::sv_stat::{SVAggregateErrorKind, SVAggregateMethod};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const CATEGORY_COUNT: usize = 4;
const SERIES_COUNT: usize = 2;
const OBSERVATIONS_PER_GROUP: usize = 12;

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    Ok(PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?))
}

fn sample_value(category: usize, series: usize, observation: usize) -> f64 {
    let base = 2.0 + category as f64 * 1.4 - series as f64 * 1.2;
    let wobble = ((observation as f64) * 1.7 + category as f64).sin();
    base + wobble * 0.9
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut values = Vec::new();
    let mut categories = Vec::new();
    let mut series = Vec::new();
    for category in 0..CATEGORY_COUNT {
        for group in 0..SERIES_COUNT {
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

    let output = SVPointPlotBuilder::new()
        .data_source(&table)
        .value("measurement")
        .category("station")
        .series("scenario")
        .estimator(SVAggregateMethod::Mean)
        .error(SVAggregateErrorKind::StandardError)
        .confidence_interval(0.95)
        .join(true)
        .dash(SVDashPattern::Dashed)
        .caps(true)
        .cap_width(0.12)
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
    let destination = figures.join("point_plot.svg");
    fs::write(&destination, document)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
