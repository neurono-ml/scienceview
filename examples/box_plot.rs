//! Box-plot example: five-number summaries by category and series, written to
//! `docs/figures/box_plot.svg`.
//!
//! Run with `cargo run --example box_plot` from the repository root. The figure
//! is produced by the tool itself: the real `SVBoxPlotBuilder` composes the
//! boxes and the public `to_svg()` serializer writes the very geometry the
//! recipe produced.

use scienceview::sv_recipes::sv_distribution::sv_box_plot::SVBoxPlotBuilder;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_layout::SVViewport;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const CATEGORY_COUNT: usize = 3;
const SERIES_COUNT: usize = 2;
const OBSERVATIONS_PER_GROUP: usize = 16;

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    Ok(PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?))
}

fn sample_value(category: usize, series: usize, observation: usize) -> f64 {
    let base = 4.0 + category as f64 * 1.5 + series as f64 * 2.2;
    let wobble = ((observation as f64) * 1.7 + category as f64 + series as f64 * 0.5).sin();
    let trend = observation as f64 * 0.12;
    base + wobble * 0.9 + trend
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut values = Vec::new();
    let mut categories = Vec::new();
    let mut series = Vec::new();
    for category in 1..=CATEGORY_COUNT {
        for group in 1..=SERIES_COUNT {
            for observation in 0..OBSERVATIONS_PER_GROUP {
                let mut value = sample_value(category, group, observation);
                if observation == 2 {
                    value += 6.0;
                }
                if observation == 5 {
                    value -= 5.5;
                }
                values.push(value);
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

    let output = SVBoxPlotBuilder::new()
        .data_source(&table)
        .value("measurement")
        .category("station")
        .series("scenario")
        .show_mean(true)
        .show_outliers(true)
        .width(0.7)
        .title("Measurement distribution by station and scenario")
        .x_title("station")
        .y_title("measurement")
        .viewport(SVViewport::new(640.0, 400.0))
        .backend(SVBackendKind::Svg)
        .build()?
        .render()?;

    let document = output.to_svg()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("box_plot.svg");
    fs::write(&destination, document)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
