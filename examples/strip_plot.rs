//! Strip-plot example: every observation of a variable, jittered along its
//! category, written to `docs/figures/strip_plot.svg`.
//!
//! Run with `cargo run --example strip_plot --features render-svg` from the
//! repository root. The figure is produced by the tool itself: the real
//! `SVStripPlot` builder composes the categories, the keyed jitter and the
//! scales, and the public `to_svg()` method serializes the scene it returns —
//! this program contains no serializer of its own.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
use scienceview::sv_recipes::sv_distribution::sv_strip_plot::SVStripPlot;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const OBSERVATIONS_PER_GROUP: usize = 36;
const CATEGORY_COUNT: usize = 3;
const SERIES_COUNT: usize = 2;

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    Ok(PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?))
}

fn sample_value(category: usize, group: usize, observation: usize) -> f64 {
    let base = 38.0 + category as f64 * 5.5 + group as f64 * 1.8;
    let wobble = ((observation as f64) * 2.3 + category as f64 * 0.7 + group as f64).sin();
    let trend = ((observation as f64) * 0.37).cos() * 1.4;
    base + wobble * 2.6 + trend
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut values = Vec::new();
    let mut categories = Vec::new();
    let mut series = Vec::new();
    for category in 1..=CATEGORY_COUNT {
        for group in 1..=SERIES_COUNT {
            for observation in 0..OBSERVATIONS_PER_GROUP {
                let mut value = sample_value(category, group, observation);
                if observation == 4 {
                    // A missing observation: the default policy omits it.
                    value = f64::NAN;
                }
                values.push(value);
                categories.push(category as f64);
                series.push(group as f64);
            }
        }
    }

    let table = SVDataTableBuilder::new()
        .with_column("body_mass", SVColumnRole::Y, &values)
        .with_column("island", SVColumnRole::Grouping, &categories)
        .with_column("sex", SVColumnRole::Grouping, &series)
        .build()?;

    let scene = SVStripPlot::builder()
        .data_source(&table)
        .value("body_mass")
        .category("island")
        .series("sex")
        .series_labels(&["female", "male"])
        .jitter(0.18)
        .jitter_mode(scienceview::sv_move::SVJitterMode::Uniform)
        .seed(42)
        .marker(scienceview::sv_mark::SVDotShape::Circle)
        .alpha(0.75)
        .size(scienceview::sv_layout::SVLength::absolute(6.0))
        .title("Body mass by island and sex")
        .value_label("body mass (g)")
        .category_label("island")
        .backend(SVBackendKind::Svg)
        .build()?
        .compose()?;

    let document = scene.to_svg()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("strip_plot.svg");
    fs::write(&destination, document)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
