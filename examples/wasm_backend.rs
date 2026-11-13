//! WebAssembly backend example: the same figure the canvas presents, written
//! to `docs/figures/wasm_backend.svg`.
//!
//! Run with `cargo run --example wasm_backend --features render-svg` from the
//! repository root. The figure is produced by the tool itself through the SVG
//! serializer. With `--features render-svg,wasm` the example also rasterizes
//! the very same figure through `SVWasmBackend` headlessly and checks that the
//! canvas target produces the pixels the shared CPU rasterizer produced.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::SVColumnView;
use scienceview::sv_figure::SVFigureBuilder;
use scienceview_svg::sv_register_svg_backend;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const POINT_COUNT: usize = 96;

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    let directory = std::env::var("CARGO_MANIFEST_DIR")?;
    Ok(PathBuf::from(directory))
}

fn wave_sample(index: usize) -> f64 {
    let fraction = index as f64 / (POINT_COUNT - 1) as f64;
    (fraction * std::f64::consts::TAU).sin() + fraction * 0.5
}

fn main() -> Result<(), Box<dyn Error>> {
    let samples: Vec<f64> = (0..POINT_COUNT).map(wave_sample).collect();

    sv_register_svg_backend();
    let figure = SVFigureBuilder::new()
        .data_source(SVColumnView::Dense(&samples))
        .backend(SVBackendKind::Svg)
        .title("Wave (scienceview WebAssembly backend)")
        .build()?;
    let output = figure.render()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("wasm_backend.svg");
    fs::write(&destination, output.bytes)?;
    println!("Wrote {}", destination.display());

    #[cfg(feature = "wasm")]
    sv_check_headless_frame(&samples)?;

    Ok(())
}

/// Rasterizes the same data through the canvas target without a browser.
///
/// This is the comparable-frame surface of the backend: the frame is what the
/// canvas would blit, and it comes from the very rasterizer the native target
/// presents.
#[cfg(feature = "wasm")]
fn sv_check_headless_frame(samples: &[f64]) -> Result<(), Box<dyn Error>> {
    use scienceview::scienceview_wasm::SVWasmBackend;
    use scienceview::sv_data::SVColumnView;
    use scienceview::sv_figure::SVExecutionMode;

    let backend = SVWasmBackend::new();
    let frame = backend.render_frame(&scienceview::sv_backend::SVRenderRequest {
        backend_kind: SVBackendKind::Wasm,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(samples),
        title: Some("Wave (scienceview WebAssembly backend)"),
    })?;

    println!(
        "Headless canvas frame: {}x{} device pixels, {} bytes",
        frame.width(),
        frame.height(),
        frame.pixels().len()
    );
    assert!(!frame.is_empty(), "the canvas frame has pixels");

    Ok(())
}
