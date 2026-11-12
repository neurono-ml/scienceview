//! The rasterizer is deterministic and representation-parity holds.

use crate::backend::SVNativeBackend;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_data::SVSparseView;
use scienceview_core::sv_data::SVStreamingView;
use scienceview_core::sv_figure::SVExecutionMode;

fn sv_request(view: SVColumnView<'_>) -> SVRenderRequest<'_> {
    SVRenderRequest {
        backend_kind: SVBackendKind::Native,
        execution_mode: SVExecutionMode::Automatic,
        view,
        title: Some("Parity"),
    }
}

#[test]
fn the_same_request_renders_byte_identical_frames() {
    let values = [0.5, 4.0, 1.5, 3.0, 2.0];
    let backend = SVNativeBackend::new();

    let first = backend
        .render(&sv_request(SVColumnView::Dense(&values)))
        .expect("the backend renders");
    let second = backend
        .render(&sv_request(SVColumnView::Dense(&values)))
        .expect("the backend renders");

    assert_eq!(first, second);
}

#[test]
fn dense_sparse_and_streaming_views_render_the_same_frame() {
    let values = [0.5, 4.0, 1.5, 3.0, 2.0];
    let indices = [0, 1, 2, 3, 4];
    let sparse =
        SVSparseView::new(&indices, &values, values.len()).expect("the sparse view is valid");
    let streaming = SVStreamingView::new(&values, 1, 0);
    let backend = SVNativeBackend::new();

    let dense_frame = backend
        .render(&sv_request(SVColumnView::Dense(&values)))
        .expect("the dense view renders");
    let sparse_frame = backend
        .render(&sv_request(SVColumnView::Sparse(sparse)))
        .expect("the sparse view renders");
    let streaming_frame = backend
        .render(&sv_request(SVColumnView::Streaming(streaming)))
        .expect("the streaming view renders");

    assert_eq!(dense_frame, sparse_frame);
    assert_eq!(dense_frame, streaming_frame);
}

#[test]
fn a_sparse_gap_stays_a_gap() {
    let values = [1.0, 5.0];
    let indices = [0, 4];
    let sparse = SVSparseView::new(&indices, &values, 5).expect("the sparse view is valid");
    let backend = SVNativeBackend::new();

    let sparse_frame = backend
        .render(&sv_request(SVColumnView::Sparse(sparse)))
        .expect("the sparse view renders");
    let dense_frame = backend
        .render(&sv_request(SVColumnView::Dense(&values)))
        .expect("the dense view renders");

    assert_ne!(
        sparse_frame, dense_frame,
        "the logical extent of the sparse column is preserved in the scale"
    );
}
