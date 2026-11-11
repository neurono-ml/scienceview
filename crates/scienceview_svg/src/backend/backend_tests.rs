//! Tests for the backend contract: the capability row, the export-only
//! semantics and the registry dispatch (spec `render-svg-backend`,
//! `backend-capabilities`, `backend-selection`).

use super::SVSvgBackend;
use super::SVSvgBackendProvider;
use super::sv_register_svg_backend;
use crate::document::SVSvgDocument;
use scienceview_core::sv_backend::SVBackendCapability;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVBackendRegistry;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderBackendProvider;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_backend::SVSaveRequest;
use scienceview_core::sv_backend::SVShowRequest;
use scienceview_core::sv_backend::SVUpdateRequest;
use scienceview_core::sv_backend::sv_backend_capabilities;
use scienceview_core::sv_backend::sv_resolve_render_backend;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::{SVExecutionMode, SVFigureBuilder};
use scienceview_core::sv_layout::SVViewport;
use std::sync::Arc;

fn request(values: &[f64]) -> SVRenderRequest<'_> {
    SVRenderRequest {
        backend_kind: SVBackendKind::Svg,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(values),
        title: None,
    }
}

#[test]
fn the_backend_reports_the_svg_kind_and_the_export_only_row() {
    let backend = SVSvgBackend::new();

    assert_eq!(backend.kind(), SVBackendKind::Svg);
    assert_eq!(
        backend.capabilities(),
        sv_backend_capabilities(SVBackendKind::Svg)
    );
    assert!(backend.capabilities().supports(SVBackendCapability::Export));
    assert!(
        !backend
            .capabilities()
            .supports(SVBackendCapability::InPlaceUpdate)
    );
}

#[test]
fn an_empty_figure_renders_a_well_formed_document() {
    let backend = SVSvgBackend::new();
    let output = backend
        .render(&request(&[]))
        .expect("the backend always renders a document");

    let document = String::from_utf8(output.bytes).expect("the document is UTF-8");
    assert_eq!(output.backend_kind, SVBackendKind::Svg);
    assert!(document.starts_with("<svg xmlns=\"http://www.w3.org/2000/svg\""));
    assert!(document.contains("viewBox=\"0 0 640 400\""));
    assert!(document.contains("width=\"640\""));
    assert!(document.contains("height=\"400\""));
    assert!(document.contains("role=\"img\""));
    assert!(document.contains("<title>"));
    assert!(document.contains("<desc>"));
    assert!(document.trim_end().ends_with("</svg>"));
}

#[test]
fn a_title_reaches_the_document_and_its_accessibility_label() {
    let backend = SVSvgBackend::new();
    let values = [1.0, 2.0, 3.0];
    let mut figure_request = request(&values);
    figure_request.title = Some("Signal & noise");

    let output = backend
        .render(&figure_request)
        .expect("the backend renders the title");
    let document = String::from_utf8(output.bytes).expect("the document is UTF-8");

    assert!(document.contains("Signal &amp; noise"), "{document}");
    assert!(
        document.contains("<title>Signal &amp; noise</title>"),
        "{document}"
    );
}

#[test]
fn update_and_show_report_the_missing_capabilities() {
    let backend = SVSvgBackend::new();
    let values = [1.0, 2.0];

    let update_error = backend
        .update(&SVUpdateRequest {
            backend_kind: SVBackendKind::Svg,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&values),
            appended_points: 1,
        })
        .expect_err("a static document has no in-place update");
    let show_error = backend
        .show(&SVShowRequest {
            backend_kind: SVBackendKind::Svg,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&values),
        })
        .expect_err("a static document has no presentation surface");

    assert_eq!(
        update_error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Svg,
            capability: SVBackendCapability::InPlaceUpdate,
        }
    );
    assert_eq!(
        show_error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Svg,
            capability: SVBackendCapability::Presentation,
        }
    );
}

#[test]
fn save_writes_the_rendered_document_to_disk() {
    let backend = SVSvgBackend::new();
    let values = [1.0, 2.0, 3.0];
    let destination = std::env::temp_dir().join(format!(
        "scienceview-svg-save-{}-{:?}.svg",
        std::process::id(),
        std::thread::current().id()
    ));

    backend
        .save(&SVSaveRequest {
            backend_kind: SVBackendKind::Svg,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&values),
            destination: &destination,
        })
        .expect("the backend can export a file");
    let written = std::fs::read_to_string(&destination).expect("the file was written");
    let rendered = backend.render(&request(&values)).expect("render succeeds");
    assert_eq!(written.as_bytes(), rendered.bytes.as_slice());

    std::fs::remove_file(&destination).expect("the temporary file is removed");
}

#[test]
fn the_provider_answers_only_for_svg() {
    let provider = SVSvgBackendProvider::new();

    let backend = provider
        .provide(SVBackendKind::Svg)
        .expect("the provider serves svg");
    assert_eq!(backend.kind(), SVBackendKind::Svg);
    assert!(provider.provide(SVBackendKind::Html).is_none());
    assert!(provider.provide(SVBackendKind::Native).is_none());
}

#[test]
fn a_registered_backend_is_dispatched_through_a_registry() {
    let registry = SVBackendRegistry::new();
    registry.register(Arc::new(SVSvgBackendProvider::new()));

    let backend = sv_resolve_render_backend(&registry, SVBackendKind::Svg)
        .expect("the registry resolves the svg backend");
    assert_eq!(backend.kind(), SVBackendKind::Svg);
}

#[test]
fn a_built_figure_renders_end_to_end_through_the_process_registry() {
    sv_register_svg_backend();

    let values = [0.0, 1.0, 0.0, -1.0, 0.0];
    let figure = SVFigureBuilder::new()
        .data_source(SVColumnView::Dense(&values))
        .backend(SVBackendKind::Svg)
        .title("End to end")
        .build()
        .expect("the configuration is complete");

    let output = figure.render().expect("the registered backend renders");
    let document = String::from_utf8(output.bytes).expect("the document is UTF-8");

    assert_eq!(output.backend_kind, SVBackendKind::Svg);
    assert!(document.contains("<svg xmlns=\"http://www.w3.org/2000/svg\""));
    assert!(document.contains("End to end"));
}

#[test]
fn a_document_composes_the_public_surface_without_the_backend() {
    let viewport = SVViewport::new(320.0, 200.0);
    let document = SVSvgDocument::new(viewport).with_title("Composed").to_svg();

    assert!(document.contains("viewBox=\"0 0 320 200\""));
    assert!(document.contains("<title>Composed</title>"));
}
