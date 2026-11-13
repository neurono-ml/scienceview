//! The `SVRenderBackend` implementation for a self-contained HTML document.
//!
//! The backend is the document target of the capability matrix: it exports a
//! page, updates the page in place through the embedded payload, and reports
//! the missing presentation capability for `show` instead of pretending to own
//! a window. The SVG markup is produced by the SVG backend, so both targets
//! draw exactly the same figure.

use crate::document::SVHtmlDocument;
use crate::payload::SVHtmlDataPayload;
use scienceview_core::sv_backend::SVBackendCapabilities;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderOutput;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_backend::SVSaveRequest;
use scienceview_core::sv_backend::SVUpdateRequest;
use scienceview_core::sv_backend::sv_backend_capabilities;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_svg::SVSvgBackend;
use std::fs;

/// Renders figures as self-contained, interactive HTML documents.
///
/// The value carries no state: every operation derives its output from the
/// request, so one instance serves concurrent renders.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SVHtmlBackend;

impl SVHtmlBackend {
    /// Creates the backend.
    pub const fn new() -> Self {
        Self
    }
}

impl SVRenderBackend for SVHtmlBackend {
    fn kind(&self) -> SVBackendKind {
        SVBackendKind::Html
    }

    fn capabilities(&self) -> SVBackendCapabilities {
        sv_backend_capabilities(SVBackendKind::Html)
    }

    fn render(&self, request: &SVRenderRequest<'_>) -> Result<SVRenderOutput, SVError> {
        let document = sv_build_document(request.view, request.title, request.execution_mode, 0)?;

        Ok(SVRenderOutput {
            backend_kind: request.backend_kind,
            bytes: document.to_bytes(),
        })
    }

    fn update(&self, request: &SVUpdateRequest<'_>) -> Result<SVRenderOutput, SVError> {
        let payload = sv_build_payload(request.view, request.appended_points)?;

        Ok(SVRenderOutput {
            backend_kind: request.backend_kind,
            bytes: SVHtmlDocument::new(payload).data_block().into_bytes(),
        })
    }

    fn save(&self, request: &SVSaveRequest<'_>) -> Result<(), SVError> {
        let document = sv_build_document(request.view, None, request.execution_mode, 0)?;

        fs::write(request.destination, document.to_bytes()).map_err(|error| {
            SVError::BackendFailure {
                backend_kind: SVBackendKind::Html,
                operation: "save",
                message: format!("cannot write {}: {error}", request.destination.display()),
            }
        })
    }
}

fn sv_build_document(
    view: SVColumnView<'_>,
    title: Option<&str>,
    execution_mode: SVExecutionMode,
    appended_points: usize,
) -> Result<SVHtmlDocument, SVError> {
    let payload = sv_build_payload_with_title(view, title, execution_mode, appended_points)?;
    let document = SVHtmlDocument::new(payload);

    Ok(match title {
        Some(title) => document.with_title(title),
        None => document,
    })
}

fn sv_build_payload(
    view: SVColumnView<'_>,
    appended_points: usize,
) -> Result<SVHtmlDataPayload, SVError> {
    sv_build_payload_with_title(view, None, SVExecutionMode::Automatic, appended_points)
}

fn sv_build_payload_with_title(
    view: SVColumnView<'_>,
    title: Option<&str>,
    execution_mode: SVExecutionMode,
    appended_points: usize,
) -> Result<SVHtmlDataPayload, SVError> {
    let svg = SVSvgBackend::new().render(&SVRenderRequest {
        backend_kind: SVBackendKind::Svg,
        execution_mode,
        view,
        title,
    })?;
    let markup = String::from_utf8(svg.bytes).map_err(|error| SVError::BackendFailure {
        backend_kind: SVBackendKind::Html,
        operation: "render",
        message: format!("the SVG markup is not UTF-8: {error}"),
    })?;

    Ok(SVHtmlDataPayload::new(view, markup).with_appended_points(appended_points))
}
