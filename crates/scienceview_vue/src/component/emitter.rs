//! Rendering a figure request as a self-contained Vue component source.
//!
//! The figure itself is rendered by the SVG backend, so a component and a
//! standalone SVG document can never disagree about the geometry. This module
//! only wraps that markup, the data snapshot and the theme in the Vue
//! template.

use super::data_serialization::sv_component_data_json;
use super::escaping::sv_quote_javascript_string;
use super::escaping::sv_quote_markup_string;
use super::templates::VUE_TEMPLATE;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_error::SVError;
use scienceview_svg::SVSvgBackend;

/// The title used when the figure does not carry one.
const DEFAULT_TITLE: &str = "scienceview figure";

/// The theme object embedded in the component until the host overrides it.
const DEFAULT_THEME_JSON: &str =
    "{\"background\":\"#ffffff\",\"foreground\":\"#1f2933\",\"accent\":\"#2f6feb\"}";

/// Renders a Vue component source string for `request`.
pub fn sv_render_vue_component(request: &SVRenderRequest<'_>) -> Result<String, SVError> {
    sv_render_component_source(request, 0)
}

/// Renders the component source, recording how many points an update appended.
pub(crate) fn sv_render_component_source(
    request: &SVRenderRequest<'_>,
    appended_points: usize,
) -> Result<String, SVError> {
    let title = request.title.unwrap_or(DEFAULT_TITLE);
    let svg_output = SVSvgBackend::new().render(&SVRenderRequest {
        backend_kind: SVBackendKind::Svg,
        execution_mode: request.execution_mode,
        view: request.view,
        title: Some(title),
    })?;
    let markup = String::from_utf8(svg_output.bytes).map_err(|error| SVError::BackendFailure {
        backend_kind: request.backend_kind,
        operation: "render",
        message: format!("the SVG backend produced non-UTF-8 markup: {error}"),
    })?;

    let data_json = sv_component_data_json(&request.view);

    Ok(VUE_TEMPLATE
        .replace("__MARKUP_LITERAL__", &sv_quote_markup_string(&markup))
        .replace("__DATA_JSON__", &data_json)
        .replace("__TITLE_LITERAL__", &sv_quote_javascript_string(title))
        .replace("__THEME_JSON__", DEFAULT_THEME_JSON)
        .replace("__APPENDED_POINTS__", &appended_points.to_string()))
}
