//! Tests for the backend contract: the capability row, the document and
//! in-place update semantics and the registry dispatch (spec
//! `render-html-backend`, `backend-capabilities`, `backend-selection`).

use super::SVHtmlBackend;
use super::SVHtmlBackendProvider;
use super::sv_register_html_backend;
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
use std::sync::Arc;

const VALUES: [f64; 5] = [0.0, 1.0, 0.0, -1.0, 0.0];

fn request(values: &[f64]) -> SVRenderRequest<'_> {
    SVRenderRequest {
        backend_kind: SVBackendKind::Html,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(values),
        title: None,
    }
}

fn rendered(backend: &SVHtmlBackend, title: Option<&str>) -> String {
    let mut request = request(&VALUES);
    request.title = title;
    let output = backend.render(&request).expect("the backend renders");
    assert_eq!(output.backend_kind, SVBackendKind::Html);
    String::from_utf8(output.bytes).expect("the document is UTF-8")
}

#[test]
fn the_backend_reports_the_html_row() {
    let backend = SVHtmlBackend::new();
    let capabilities = backend.capabilities();

    assert_eq!(backend.kind(), SVBackendKind::Html);
    assert_eq!(capabilities, sv_backend_capabilities(SVBackendKind::Html));
    assert!(capabilities.supports(SVBackendCapability::Export));
    assert!(capabilities.supports(SVBackendCapability::InPlaceUpdate));
    assert!(capabilities.supports(SVBackendCapability::Streaming));
    assert!(capabilities.supports(SVBackendCapability::Events));
    assert!(!capabilities.supports(SVBackendCapability::Scripting));
    assert!(!capabilities.supports(SVBackendCapability::Presentation));
}

#[test]
fn a_saved_document_renders_without_external_resources() {
    let backend = SVHtmlBackend::new();
    let html = rendered(&backend, Some("Offline"));

    assert!(html.starts_with("<!DOCTYPE html>"), "{html}");
    assert!(
        html.contains("<svg xmlns=\"http://www.w3.org/2000/svg\""),
        "{html}"
    );
    assert!(html.contains("viewBox=\"0 0 640 400\""), "{html}");
    assert!(html.contains("Offline"), "{html}");
    assert!(html.contains("<style>"), "{html}");
    assert!(!html.contains("<link "), "{html}");
    assert!(!html.contains("@import"), "{html}");
    assert!(!html.contains(" src=\""), "{html}");
}

#[test]
fn the_title_reaches_the_page_and_its_caption() {
    let html = rendered(&SVHtmlBackend::new(), Some("Signal & noise"));

    assert!(html.contains("<title>Signal &amp; noise</title>"), "{html}");
    assert!(html.contains(">Signal &amp; noise</figcaption>"), "{html}");
}

#[test]
fn the_document_embeds_a_sparse_column_without_densifying_it() {
    let indices = [0_usize, 4];
    let values = [1.0, -1.0];
    let sparse = scienceview_core::sv_data::SVSparseView::new(&indices, &values, 5)
        .expect("valid sparse view");
    let backend = SVHtmlBackend::new();
    let output = backend
        .render(&SVRenderRequest {
            backend_kind: SVBackendKind::Html,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Sparse(sparse),
            title: None,
        })
        .expect("the backend renders sparse data");
    let html = String::from_utf8(output.bytes).expect("the document is UTF-8");

    assert!(html.contains("\"representation\":\"Sparse\""), "{html}");
    assert!(html.contains("\"indices\":[0,4]"), "{html}");
    assert!(html.contains("\"logical_length\":5"), "{html}");
}

#[test]
fn streaming_data_is_embedded_from_the_snapshot_it_already_carries() {
    let snapshot = [1.0, 2.0, 3.0];
    let streaming = scienceview_core::sv_data::SVStreamingView::new(&snapshot, 12, 1);
    let backend = SVHtmlBackend::new();
    let output = backend
        .render(&SVRenderRequest {
            backend_kind: SVBackendKind::Html,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Streaming(streaming),
            title: None,
        })
        .expect("the backend renders a streaming snapshot");
    let html = String::from_utf8(output.bytes).expect("the document is UTF-8");

    assert!(html.contains("\"representation\":\"Streaming\""), "{html}");
    assert!(html.contains("\"values\":[1,2,3]"), "{html}");
    assert!(html.contains("\"sequence\":12"), "{html}");
    assert!(html.contains("\"dropped_samples\":1"), "{html}");
}

#[test]
fn an_update_returns_only_the_data_block() {
    let backend = SVHtmlBackend::new();
    let html = rendered(&backend, None);
    let output = backend
        .update(&SVUpdateRequest {
            backend_kind: SVBackendKind::Html,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&VALUES),
            appended_points: 2,
        })
        .expect("the html backend updates in place");
    let update = String::from_utf8(output.bytes).expect("the payload is UTF-8");

    assert!(
        update.starts_with("<script type=\"application/json\""),
        "{update}"
    );
    assert!(update.trim_end().ends_with("</script>"), "{update}");
    assert!(!update.contains("<html"), "{update}");
    assert!(!update.contains("<style"), "{update}");
    assert!(!update.contains("<figure"), "{update}");
    assert!(update.contains("\"revision\":2"), "{update}");
    assert!(update.contains("\"appended_points\":2"), "{update}");
    assert!(
        html.contains("\"revision\":0"),
        "the document starts at the zero revision"
    );
    assert!(
        html.contains("\"points\":5"),
        "the document carries the same data"
    );
}

#[test]
fn an_update_changes_only_the_data_block_and_keeps_the_host_state() {
    let backend = SVHtmlBackend::new();
    let html = rendered(&backend, Some("Stable"));
    let block_start = html
        .find("<script type=\"application/json\"")
        .expect("the document carries a data block");
    let block_end = html[block_start..]
        .find("</script>")
        .map(|offset| block_start + offset + "</script>".len())
        .expect("the data block is closed");
    let original_block = &html[block_start..block_end];

    let update = backend
        .update(&SVUpdateRequest {
            backend_kind: SVBackendKind::Html,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&VALUES),
            appended_points: 1,
        })
        .expect("the html backend updates in place");
    let update = String::from_utf8(update.bytes).expect("the payload is UTF-8");

    assert!(!update.contains("<style"), "the stylesheet stays: {update}");
    assert!(
        !update.contains("<figcaption"),
        "the caption stays: {update}"
    );
    assert!(
        !update.contains("window.scienceview"),
        "the runtime stays: {update}"
    );
    assert!(html[..block_start].contains("Stable"));
    assert!(html[block_end..].contains("window.scienceview"));

    let rebuilt = format!("{}{}{}", &html[..block_start], update, &html[block_end..]);
    assert_eq!(&rebuilt[..block_start], &html[..block_start]);
    assert_eq!(
        &rebuilt[block_start + update.len()..],
        &html[block_end..],
        "everything after the data block is untouched"
    );
    assert!(rebuilt.contains("Stable"));
    assert_ne!(original_block, update.as_str());
}

#[test]
fn show_reports_the_missing_presentation_capability() {
    let error = SVHtmlBackend::new()
        .show(&SVShowRequest {
            backend_kind: SVBackendKind::Html,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&VALUES),
        })
        .expect_err("a document has no window to present");

    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Html,
            capability: SVBackendCapability::Presentation,
        }
    );
}

#[test]
fn scripting_is_rejected_at_build_time_for_html() {
    let error = SVFigureBuilder::new()
        .data_source(SVColumnView::Dense(&VALUES))
        .backend(SVBackendKind::Html)
        .requires_scripting()
        .build()
        .err()
        .expect("the document inlines a runtime, it does not host a script engine");

    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Html,
            capability: SVBackendCapability::Scripting,
        }
    );
}

#[test]
fn save_writes_the_rendered_document_to_disk() {
    let backend = SVHtmlBackend::new();
    let destination = std::env::temp_dir().join(format!(
        "scienceview-html-save-{}-{:?}.html",
        std::process::id(),
        std::thread::current().id()
    ));

    backend
        .save(&SVSaveRequest {
            backend_kind: SVBackendKind::Html,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&VALUES),
            destination: &destination,
        })
        .expect("the backend can export a file");
    let written = std::fs::read_to_string(&destination).expect("the file was written");
    assert_eq!(written.as_bytes(), rendered(&backend, None).as_bytes());

    std::fs::remove_file(&destination).expect("the temporary file is removed");
}

#[test]
fn save_reports_a_typed_failure_when_the_destination_cannot_be_written() {
    let error = SVHtmlBackend::new()
        .save(&SVSaveRequest {
            backend_kind: SVBackendKind::Html,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&VALUES),
            destination: &std::env::temp_dir(),
        })
        .expect_err("a directory is not a writable file");

    assert!(matches!(
        error,
        SVError::BackendFailure {
            backend_kind: SVBackendKind::Html,
            operation: "save",
            ..
        }
    ));
}

#[test]
fn the_provider_answers_only_for_html() {
    let provider = SVHtmlBackendProvider::new();

    let backend = provider
        .provide(SVBackendKind::Html)
        .expect("the provider serves html");
    assert_eq!(backend.kind(), SVBackendKind::Html);
    assert!(provider.provide(SVBackendKind::Svg).is_none());
    assert!(provider.provide(SVBackendKind::Native).is_none());
}

#[test]
fn a_registered_backend_is_dispatched_through_a_registry() {
    let registry = SVBackendRegistry::new();
    registry.register(Arc::new(SVHtmlBackendProvider::new()));

    let backend = sv_resolve_render_backend(&registry, SVBackendKind::Html)
        .expect("the registry resolves the html backend");
    assert_eq!(backend.kind(), SVBackendKind::Html);
}

#[test]
fn a_built_figure_renders_end_to_end_through_the_process_registry() {
    sv_register_html_backend();

    let figure = SVFigureBuilder::new()
        .data_source(SVColumnView::Dense(&VALUES))
        .backend(SVBackendKind::Html)
        .title("End to end")
        .build()
        .expect("the configuration is complete");

    let output = figure.render().expect("the registered backend renders");
    let html = String::from_utf8(output.bytes).expect("the document is UTF-8");

    assert_eq!(output.backend_kind, SVBackendKind::Html);
    assert!(html.contains("End to end"), "{html}");
    assert!(html.contains("data-scienceview-figure"), "{html}");
}
