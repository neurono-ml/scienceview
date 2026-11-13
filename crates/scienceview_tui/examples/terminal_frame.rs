//! Renders a bare figure onto the terminal cell grid and prints its plain text.
//!
//! This example needs no terminal: the rasterizer is headless, so running it
//! dumps the golden frame that the companion tests assert on.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_tui::SVTuiBackend;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let values: Vec<f64> = (0..32).map(|index| (index as f64 * 0.2).sin()).collect();
    let request = SVRenderRequest {
        backend_kind: SVBackendKind::Tui,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(&values),
        title: Some("Terminal frame"),
    };

    let frame = SVTuiBackend::new().render_frame(&request)?;
    println!("{}", frame.to_text());

    Ok(())
}
