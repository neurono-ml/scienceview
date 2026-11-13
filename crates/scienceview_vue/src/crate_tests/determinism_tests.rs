//! The component source is deterministic and representation metadata is the
//! only difference between representations of the same values.

use crate::SVVueBackend;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_data::SVSparseView;
use scienceview_core::sv_data::SVStreamingView;
use scienceview_core::sv_figure::SVExecutionMode;

fn sv_request(view: SVColumnView<'_>) -> SVRenderRequest<'_> {
    SVRenderRequest {
        backend_kind: SVBackendKind::Vue,
        execution_mode: SVExecutionMode::Automatic,
        view,
        title: Some("Deterministic"),
    }
}

#[test]
fn the_same_request_renders_byte_identical_source() {
    let values = [0.5, 4.0, 1.5, 3.0, 2.0];
    let backend = SVVueBackend::new();

    let first = backend
        .render(&sv_request(SVColumnView::Dense(&values)))
        .expect("renders");
    let second = backend
        .render(&sv_request(SVColumnView::Dense(&values)))
        .expect("renders");

    assert_eq!(first, second);
}

#[test]
fn every_representation_embeds_the_same_inline_svg_markup() {
    let values = [0.5, 4.0, 1.5, 3.0, 2.0];
    let indices = [0, 1, 2, 3, 4];
    let sparse =
        SVSparseView::new(&indices, &values, values.len()).expect("the sparse view is valid");
    let streaming = SVStreamingView::new(&values, 1, 0);
    let backend = SVVueBackend::new();

    let dense_source = backend
        .render(&sv_request(SVColumnView::Dense(&values)))
        .expect("renders");
    let sparse_source = backend
        .render(&sv_request(SVColumnView::Sparse(sparse)))
        .expect("renders");
    let streaming_source = backend
        .render(&sv_request(SVColumnView::Streaming(streaming)))
        .expect("renders");

    let dense_markup = sv_markup(&dense_source.bytes);
    assert_eq!(dense_markup, sv_markup(&sparse_source.bytes));
    assert_eq!(dense_markup, sv_markup(&streaming_source.bytes));
}

fn sv_markup(source: &[u8]) -> String {
    let text = String::from_utf8(source.to_vec()).expect("the source is UTF-8");
    let start = text
        .find("<svg")
        .expect("the markup starts with an SVG element");
    let end = text
        .find("</svg>")
        .expect("the markup ends with an SVG element")
        + "</svg>".len();
    text[start..end].to_string()
}
