//! Tests for the default figure a bare `SVFigure` renders.

use super::sv_render_default_figure;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;

fn request<'values>(
    values: &'values [f64],
    title: Option<&'values str>,
) -> SVRenderRequest<'values> {
    SVRenderRequest {
        backend_kind: SVBackendKind::Svg,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(values),
        title,
    }
}

#[test]
fn an_empty_view_still_produces_a_document() {
    let document = sv_render_default_figure(&request(&[], None)).expect("the figure renders");

    assert!(document.starts_with("<svg "));
    assert!(document.contains("<title>scienceview figure</title>"));
}

#[test]
fn the_configured_title_is_the_document_title() {
    let document =
        sv_render_default_figure(&request(&[1.0], Some("A title"))).expect("the figure renders");

    assert!(document.contains("<title>A title</title>"));
    assert!(document.contains("aria-label=\"A title\""));
}

#[test]
fn the_background_comes_from_the_decoration_style() {
    let document = sv_render_default_figure(&request(&[1.0], None)).expect("the figure renders");

    assert!(document.contains("<rect x=\"0\" y=\"0\" width=\"640\" height=\"400\""));
}

#[test]
fn dense_values_become_a_line_and_two_axes() {
    let values = [0.0, 1.0, 0.0, -1.0, 0.0];
    let document = sv_render_default_figure(&request(&values, Some("Wave"))).expect("renders");

    assert!(document.contains("<path "), "{document}");
    assert!(
        document.contains("<line "),
        "the axes draw lines: {document}"
    );
    assert!(
        document.contains("<text "),
        "the ticks carry labels: {document}"
    );
    assert!(document.contains("Wave"));
}

#[test]
fn sparse_values_are_read_from_their_stored_entries_without_densifying() {
    let indices = [0usize, 2, 4];
    let values = [1.0, 2.0, 3.0];
    let sparse = scienceview_core::sv_data::SVSparseView::new(&indices, &values, 5)
        .expect("the sparse view is valid");
    let document = sv_render_default_figure(&SVRenderRequest {
        backend_kind: SVBackendKind::Svg,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Sparse(sparse),
        title: None,
    })
    .expect("the figure renders");

    assert!(document.contains("<path "), "{document}");
}

#[test]
fn streaming_values_render_the_bounded_snapshot() {
    let values = [1.0, 2.0, 1.5];
    let streaming = scienceview_core::sv_data::SVStreamingView::new(&values, 7, 0);
    let document = sv_render_default_figure(&SVRenderRequest {
        backend_kind: SVBackendKind::Svg,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Streaming(streaming),
        title: None,
    })
    .expect("the figure renders");

    assert!(document.contains("<path "), "{document}");
}
