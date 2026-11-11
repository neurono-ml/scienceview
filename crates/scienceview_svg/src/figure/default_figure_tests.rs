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
