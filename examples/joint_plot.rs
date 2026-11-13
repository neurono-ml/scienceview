//! Joint example: builds `SVJointPlot` and writes `docs/figures/joint_plot.svg`.
//!
//! Run with `cargo run --example joint_plot --features render-svg` from the
//! repository root. The figure is produced by the tool itself: the recipe
//! composes the central scatter, the two marginal histograms and the shared
//! scales, and the public `to_svg()` method serializes the scene it returns.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
use scienceview::sv_recipes::sv_multivariate::sv_joint_plot::SVJointPlot;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const NUMBER_OF_POINTS: usize = 140;

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    let directory = std::env::var("CARGO_MANIFEST_DIR")?;
    Ok(PathBuf::from(directory))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut horizontal = Vec::with_capacity(NUMBER_OF_POINTS);
    let mut vertical = Vec::with_capacity(NUMBER_OF_POINTS);
    let mut group = Vec::with_capacity(NUMBER_OF_POINTS);

    for index in 0..NUMBER_OF_POINTS {
        let position = index as f64;
        let series = if index % 2 == 0 { 0.0 } else { 1.0 };
        let offset = if series == 0.0 { 0.0 } else { 1.4 };
        let wave = (position * 0.37).sin() * 0.9;
        let ripple = (position * 0.11).cos() * 0.4;
        horizontal.push(position * 0.06 + wave + offset);
        vertical.push(position * 0.045 + wave * 1.8 + ripple + offset * 0.6);
        group.push(series);
    }

    let table = SVDataTableBuilder::new()
        .with_column("height", SVColumnRole::X, &horizontal)
        .with_column("weight", SVColumnRole::Y, &vertical)
        .with_column("cohort", SVColumnRole::Grouping, &group)
        .build()?;

    let scene = SVJointPlot::builder()
        .data_source(&table)
        .x("height")
        .y("weight")
        .series("cohort")
        .alpha(0.7)
        .title("Joint distribution of height and weight")
        .x_label("height")
        .y_label("weight")
        .backend(SVBackendKind::Svg)
        .build()?
        .compose()?;

    let document = scene.to_svg()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("joint_plot.svg");
    fs::write(&destination, document)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
