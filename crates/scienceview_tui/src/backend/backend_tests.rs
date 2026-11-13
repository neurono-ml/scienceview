//! The backend contract: frames, bytes, resize, update, save and show.

use super::SVTuiBackend;
use super::sv_tui_default_viewport;
use crate::cell::SVAnsiColourDepth;
use scienceview_core::sv_backend::SVBackendCapability;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_backend::SVSaveRequest;
#[cfg(not(feature = "terminal"))]
use scienceview_core::sv_backend::SVShowRequest;
use scienceview_core::sv_backend::SVUpdateRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use std::path::Path;

fn sv_request(values: &[f64]) -> SVRenderRequest<'_> {
    SVRenderRequest {
        backend_kind: SVBackendKind::Tui,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(values),
        title: Some("Terminal"),
    }
}

#[test]
fn the_default_viewport_is_eighty_by_twenty_four_cells() {
    let viewport = sv_tui_default_viewport();

    assert_eq!((viewport.cells_wide(), viewport.cells_high()), (80, 24));
    assert_eq!(viewport.width(), 640.0);
    assert_eq!(viewport.height(), 384.0);
    assert_eq!(viewport.cell_width(), 8.0);
    assert_eq!(viewport.cell_height(), 16.0);
}

#[test]
fn rendering_a_figure_produces_a_comparable_frame() {
    let values = [1.0, 3.0, 2.0, 5.0];
    let frame = SVTuiBackend::new()
        .render_frame(&sv_request(&values))
        .expect("the rasterizer renders");

    assert_eq!((frame.width(), frame.height()), (80, 24));
    assert!(!frame.is_empty());
    assert!(frame.to_text().contains("Terminal"));
}

#[test]
fn the_render_output_bytes_are_the_ansi_frame() {
    let values = [0.0, 1.0];
    let backend = SVTuiBackend::new();
    let output = backend
        .render(&sv_request(&values))
        .expect("the backend renders");
    let frame = backend
        .render_frame(&sv_request(&values))
        .expect("the frame is comparable");

    assert_eq!(output.backend_kind, SVBackendKind::Tui);
    assert_eq!(
        output.bytes,
        frame.to_ansi(SVAnsiColourDepth::Ansi256).into_bytes()
    );
}

#[test]
fn an_explicit_terminal_size_sizes_the_frame() {
    let values = [1.0, 2.0];
    let frame = SVTuiBackend::new()
        .render_frame_for_terminal(&sv_request(&values), 40, 12)
        .expect("the rasterizer renders");

    assert_eq!((frame.width(), frame.height()), (40, 12));
}

#[test]
fn degenerate_terminal_sizes_render_without_panicking() {
    let values = [1.0, 2.0];
    let backend = SVTuiBackend::new();

    for (columns, rows) in [(0, 0), (1, 1), (2, 1), (1, 2)] {
        let frame = backend
            .render_frame_for_terminal(&sv_request(&values), columns, rows)
            .expect("a degenerate terminal still renders");
        assert_eq!((frame.width(), frame.height()), (columns, rows));
    }
}

#[test]
fn updating_rerenders_the_same_frame() {
    let values = [1.0, 2.0, 3.0];
    let backend = SVTuiBackend::new();
    let update = SVUpdateRequest {
        backend_kind: SVBackendKind::Tui,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(&values),
        appended_points: 1,
    };

    let output = backend.update(&update).expect("the backend updates");
    let frame = backend
        .render_frame(&SVRenderRequest {
            title: None,
            ..sv_request(&values)
        })
        .expect("the backend renders");

    assert_eq!(output.backend_kind, SVBackendKind::Tui);
    assert_eq!(
        output.bytes,
        frame.to_ansi(SVAnsiColourDepth::Ansi256).into_bytes()
    );
}

#[test]
fn saving_reports_the_missing_export_capability() {
    let values = [1.0, 2.0];
    let error = SVTuiBackend::new()
        .save(&SVSaveRequest {
            backend_kind: SVBackendKind::Tui,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&values),
            destination: Path::new("frame.txt"),
        })
        .expect_err("a terminal declares no export format");

    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Tui,
            capability: SVBackendCapability::Export,
        }
    );
}

#[cfg(not(feature = "terminal"))]
#[test]
fn showing_without_the_terminal_feature_reports_a_backend_failure() {
    let values = [1.0, 2.0];
    let error = SVTuiBackend::new()
        .show(&SVShowRequest {
            backend_kind: SVBackendKind::Tui,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&values),
        })
        .expect_err("the interactive layer is not compiled");

    assert!(matches!(
        error,
        SVError::BackendFailure {
            backend_kind: SVBackendKind::Tui,
            ..
        }
    ));
    assert!(error.to_string().contains("terminal"));
}
