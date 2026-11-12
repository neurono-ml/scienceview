//! Line-plot example: renders a sine wave through the real SVG backend to
//! `docs/figures/line_plot.svg`.
//!
//! Run with `cargo run --example line_plot_svg` from the repository root.
//! The figure is produced by the tool itself: a built `SVFigure` is rendered
//! by the registered `SVSvgBackend`, exactly as a user program would do.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVFigureBuilder;
use scienceview_svg::sv_register_svg_backend;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const POINT_COUNT: usize = 81;

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    let directory = std::env::var("CARGO_MANIFEST_DIR")?;
    Ok(PathBuf::from(directory))
}

fn sine_sample(index: usize) -> f64 {
    let fraction = index as f64 / (POINT_COUNT - 1) as f64;
    (fraction * std::f64::consts::TAU).sin()
}

fn main() -> Result<(), Box<dyn Error>> {
    let samples: Vec<f64> = (0..POINT_COUNT).map(sine_sample).collect();

    sv_register_svg_backend();
    let figure = SVFigureBuilder::new()
        .data_source(SVColumnView::Dense(&samples))
        .backend(SVBackendKind::Svg)
        .title("Sine wave (scienceview SVG backend)")
        .build()?;
    let output = figure.render()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("line_plot.svg");
    fs::write(&destination, output.bytes)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
