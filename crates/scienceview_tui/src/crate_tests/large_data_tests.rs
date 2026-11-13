//! Ten thousand points still fit the grid and render deterministically.

use crate::backend::SVTuiBackend;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;

#[test]
fn ten_thousand_points_render_onto_one_frame() {
    let values: Vec<f64> = (0..10_000)
        .map(|index| (index as f64 * 0.01).sin())
        .collect();
    let request = SVRenderRequest {
        backend_kind: SVBackendKind::Tui,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(&values),
        title: Some("Large"),
    };

    let frame = SVTuiBackend::new()
        .render_frame(&request)
        .expect("the large figure renders");

    assert_eq!((frame.width(), frame.height()), (80, 24));
    assert!(
        frame
            .cells()
            .iter()
            .any(|cell| ('\u{2800}'..='\u{28FF}').contains(&cell.symbol())),
        "the polyline lands in braille dots"
    );
}
