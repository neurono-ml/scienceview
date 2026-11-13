//! ECDF example: two empirical cumulative distributions compared on one axes,
//! written to `docs/figures/ecdf_plot.svg`.
//!
//! Run with `cargo run --example ecdf_plot --features render-svg` from the
//! repository root. The figure is produced by the tool itself: the real
//! `SVECdfPlotBuilder` composes the curves through `SVStat::EmpiricalCdf` and
//! the public `to_svg()` serializer writes the very geometry the recipe
//! produced.

use scienceview::sv_recipes::sv_distribution::sv_ecdf_plot::SVECdfPlotBuilder;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_layout::SVViewport;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const SAMPLE_COUNT: usize = 240;

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    Ok(PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?))
}

/// A deterministic pseudo-sample, so the checked-in figure never drifts.
fn sample(index: usize) -> (f64, f64) {
    let phase = index as f64;
    let scenario = if index % 2 == 0 { 1.0 } else { 2.0 };
    let centre = if scenario == 1.0 { 0.0 } else { 1.4 };
    let spread = if scenario == 1.0 { 1.0 } else { 0.7 };
    // The sum of incommensurable sines behaves like a smooth bell.
    let value = centre
        + spread
            * 1.6
            * ((phase * 0.71).sin() + 0.6 * (phase * 1.37).sin() + 0.3 * (phase * 2.11).sin());
    (value, scenario)
}

fn main() -> Result<(), Box<dyn Error>> {
    let samples: Vec<(f64, f64)> = (0..SAMPLE_COUNT).map(sample).collect();
    let values: Vec<f64> = samples.iter().map(|sample| sample.0).collect();
    let scenario: Vec<f64> = samples.iter().map(|sample| sample.1).collect();

    let table = SVDataTable::builder()
        .with_column("error", SVColumnRole::Y, &values)
        .with_column("model", SVColumnRole::Grouping, &scenario)
        .build()?;

    let document = SVECdfPlotBuilder::new()
        .data_source(&table)
        .value("error")
        .series("model")
        .title("Residual distribution by model")
        .viewport(SVViewport::new(640.0, 400.0))
        .backend(SVBackendKind::Svg)
        .build()?
        .render()?
        .to_svg()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("ecdf_plot.svg");
    fs::write(&destination, document)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
