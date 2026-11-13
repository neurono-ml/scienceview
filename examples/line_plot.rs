//! Line-plot recipe example.
//!
//! Builds `SVLinePlot` from a real multi-series dataset through the named
//! in-memory source adapter and renders it with the configured backend. The
//! figure is written to `docs/figures/line_plot_recipe.svg`; the name is
//! distinct from the `line_plot_svg` example's figure so the two never drift
//! against each other.

use scienceview::sv_recipes::sv_relational::sv_line_plot::SVLinePlotBuilder;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_layout::SVLength;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const SAMPLE_COUNT: usize = 81;

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    Ok(PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?))
}

fn main() -> Result<(), Box<dyn Error>> {
    let phases: Vec<f64> = (0..SAMPLE_COUNT)
        .map(|index| index as f64 / (SAMPLE_COUNT - 1) as f64 * std::f64::consts::TAU)
        .collect();
    let mut x = Vec::with_capacity(SAMPLE_COUNT * 2);
    let mut y = Vec::with_capacity(SAMPLE_COUNT * 2);
    let mut series = Vec::with_capacity(SAMPLE_COUNT * 2);
    for (index, value) in phases.iter().enumerate() {
        let damping = (-0.15 * index as f64).exp();
        x.push(*value);
        y.push(damping * value.sin());
        series.push(0.0);
        x.push(*value);
        y.push(damping * value.cos());
        series.push(1.0);
    }

    let table = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &x)
        .with_column("y", SVColumnRole::Y, &y)
        .with_column("series", SVColumnRole::Grouping, &series)
        .build()?;

    let plot = SVLinePlotBuilder::new()
        .data_source(&table)
        .x("x")
        .y("y")
        .series("series")
        .markers(true)
        .line_width(SVLength::absolute(2.0))
        .title("Damped oscillation by series")
        .x_axis_title("phase")
        .y_axis_title("amplitude")
        .backend(SVBackendKind::Svg)
        .build()?;

    let output = plot.render()?;

    let figures = manifest_directory()?.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("line_plot_recipe.svg");
    fs::write(&destination, output.bytes)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
