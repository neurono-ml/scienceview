//! The pure conversion layer: specification in, builder calls and bytes out.
//!
//! Every function here runs with no interpreter and no `pyo3` in the build. The
//! conversion is the single place the binding surface is translated into the
//! Rust surface, so a Python caller and a Rust caller cannot diverge on what an
//! option means.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_error::SVError;
use scienceview::sv_figure::SVFigureBuilder;
use scienceview::sv_layout::SVLength;
use scienceview::sv_recipes::sv_distribution::sv_histogram_plot::SVHistogramPlot;
use scienceview::sv_recipes::sv_relational::sv_area_plot::SVAreaPlotBuilder;
use scienceview::sv_recipes::sv_relational::sv_line_plot::SVLinePlotBuilder;
use scienceview::sv_recipes::sv_relational::sv_scatter_plot::SVScatterPlot;
use std::path::Path;

use super::backend::sv_require_backend_kind;
use super::figure_spec::SVPythonFigureSpec;
use super::options::SVPythonPlotOptions;
use super::plot_kind::SVPythonPlotKind;
use super::render_output::SVPythonRenderOutput;

/// Renders a specification through the configured backend.
///
/// # Errors
///
/// Returns [`SVError::MissingBackendSelection`] when the caller selected no
/// backend, [`SVError::InvalidConfiguration`] for a missing required option or
/// an impossible option, and every composition or backend error of the recipe.
pub fn sv_render_python_figure(
    specification: &SVPythonFigureSpec,
) -> Result<SVPythonRenderOutput, SVError> {
    let backend_kind = sv_require_backend_kind(specification.backend)?;
    specification.options.validate(specification.kind)?;
    match specification.kind {
        SVPythonPlotKind::Figure => sv_render_figure(specification, backend_kind),
        SVPythonPlotKind::ScatterPlot => sv_render_scatter(specification, backend_kind),
        SVPythonPlotKind::LinePlot => sv_render_line(specification, backend_kind),
        SVPythonPlotKind::AreaPlot => sv_render_area(specification, backend_kind),
        SVPythonPlotKind::HistogramPlot => sv_render_histogram(specification, backend_kind),
    }
}

/// Saves a specification's rendered bytes to `destination`.
///
/// The recipe path renders and then writes exactly the bytes that `render`
/// returned, so `save` can never produce a different artifact.
///
/// # Errors
///
/// Propagates every render error and returns [`SVError::BackendFailure`] when
/// the file cannot be written.
pub fn sv_save_python_figure(
    specification: &SVPythonFigureSpec,
    destination: &Path,
) -> Result<(), SVError> {
    let backend_kind = sv_require_backend_kind(specification.backend)?;
    if specification.kind == SVPythonPlotKind::Figure {
        return sv_with_figure(specification, backend_kind, |figure| {
            figure.save(destination)
        });
    }
    let output = sv_render_python_figure(specification)?;
    std::fs::write(destination, output.bytes()).map_err(|error| SVError::BackendFailure {
        backend_kind,
        operation: "save",
        message: error.to_string(),
    })
}

/// Applies `appended_points` to a specification and returns the new artifact.
///
/// The generic figure dispatches the backend's in-place update. A recipe
/// re-renders from the advanced window: the window snapshot is the operation's
/// input, so the rendered artifact reflects every sample pushed since the last
/// call, and the call never waits for a producer.
///
/// # Errors
///
/// Returns [`SVError::InvalidConfiguration`] when the data is not a live
/// stream, and every render error.
pub fn sv_update_python_figure(
    specification: &SVPythonFigureSpec,
    appended_points: usize,
) -> Result<SVPythonRenderOutput, SVError> {
    let backend_kind = sv_require_backend_kind(specification.backend)?;
    if !specification.is_streaming() {
        return Err(SVError::InvalidConfiguration {
            message:
                "update needs a live streaming source; push samples into a `StreamingColumn` first"
                    .to_string(),
        });
    }
    specification.options.validate(specification.kind)?;
    if specification.kind == SVPythonPlotKind::Figure {
        return sv_with_figure(specification, backend_kind, |figure| {
            let output = figure.update(appended_points)?;
            Ok(SVPythonRenderOutput::new(
                output.backend_kind,
                figure.representation(),
                output.bytes,
            ))
        });
    }
    sv_render_python_figure(specification)
}

/// Renders a specification as one SVG document.
///
/// Used by the dashboard panel content, which embeds a fragment per panel.
///
/// # Errors
///
/// Returns [`SVError::UnsupportedBackendCapability`] when the specification did
/// not select `Svg`, and [`SVError::BackendFailure`] when the bytes are not
/// UTF-8.
pub fn sv_render_python_figure_svg(specification: &SVPythonFigureSpec) -> Result<String, SVError> {
    let backend_kind = sv_require_backend_kind(specification.backend)?;
    if backend_kind != SVBackendKind::Svg {
        return Err(SVError::UnsupportedBackendCapability {
            backend_kind,
            capability: scienceview::sv_backend::SVBackendCapability::Export,
        });
    }
    let output = sv_render_python_figure(specification)?;
    String::from_utf8(output.into_bytes()).map_err(|error| SVError::BackendFailure {
        backend_kind,
        operation: "render-svg",
        message: error.to_string(),
    })
}

/// The logical number of marks the specification's data holds.
pub fn sv_python_point_count(specification: &SVPythonFigureSpec) -> usize {
    specification.data.length()
}

fn sv_with_figure<Output>(
    specification: &SVPythonFigureSpec,
    backend_kind: SVBackendKind,
    operation: impl FnOnce(&scienceview::sv_figure::SVFigure<'_>) -> Result<Output, SVError>,
) -> Result<Output, SVError> {
    let mut snapshot = specification.data.primary_column()?;
    let view = snapshot.column_view();
    let requirements = specification.requirements;
    let mut builder = SVFigureBuilder::new()
        .data_source(view)
        .backend(backend_kind)
        .execution_mode(specification.execution_mode);
    if let Some(title) = specification.options.title.as_deref() {
        builder = builder.title(title);
    }
    if requirements.events {
        builder = builder.requires_events();
    }
    if requirements.scripting {
        builder = builder.requires_scripting();
    }
    if requirements.streaming || specification.is_streaming() {
        builder = builder.requires_streaming();
    }
    if requirements.in_place_update {
        builder = builder.requires_in_place_update();
    }
    let figure = builder.build()?;
    operation(&figure)
}

fn sv_render_figure(
    specification: &SVPythonFigureSpec,
    backend_kind: SVBackendKind,
) -> Result<SVPythonRenderOutput, SVError> {
    sv_with_figure(specification, backend_kind, |figure| {
        let output = figure.render()?;
        Ok(SVPythonRenderOutput::new(
            output.backend_kind,
            figure.representation(),
            output.bytes,
        ))
    })
}

fn sv_render_scatter(
    specification: &SVPythonFigureSpec,
    backend_kind: SVBackendKind,
) -> Result<SVPythonRenderOutput, SVError> {
    let options = &specification.options;
    let kind = specification.kind;
    let x = SVPythonPlotOptions::require(options.x.as_deref(), "x", kind)?;
    let y = SVPythonPlotOptions::require(options.y.as_deref(), "y", kind)?;
    let mut builder = SVScatterPlot::builder()
        .data_source(&specification.data)
        .x(x)
        .y(y)
        .backend(backend_kind)
        .execution_mode(specification.execution_mode);
    if let Some(series) = options.series.as_deref() {
        builder = builder.series(series);
    }
    if let Some(title) = options.title.as_deref() {
        builder = builder.title(title);
    }
    if let Some(label) = options.x_label.as_deref() {
        builder = builder.x_label(label);
    }
    if let Some(label) = options.y_label.as_deref() {
        builder = builder.y_label(label);
    }
    let plot = builder.build()?;
    let output = plot.render()?;
    Ok(SVPythonRenderOutput::new(
        output.backend_kind(),
        plot.representation(),
        output.bytes().to_vec(),
    ))
}

fn sv_render_line(
    specification: &SVPythonFigureSpec,
    backend_kind: SVBackendKind,
) -> Result<SVPythonRenderOutput, SVError> {
    let options = &specification.options;
    let kind = specification.kind;
    let x = SVPythonPlotOptions::require(options.x.as_deref(), "x", kind)?;
    let y = SVPythonPlotOptions::require(options.y.as_deref(), "y", kind)?;
    let mut builder = SVLinePlotBuilder::new()
        .data_source(&specification.data)
        .x(&x)
        .y(&y)
        .backend(backend_kind)
        .execution_mode(specification.execution_mode);
    if let Some(series) = options.series.as_deref() {
        builder = builder.series(series);
    }
    if let Some(title) = options.title.as_deref() {
        builder = builder.title(title);
    }
    if let Some(label) = options.x_label.as_deref() {
        builder = builder.x_axis_title(label);
    }
    if let Some(label) = options.y_label.as_deref() {
        builder = builder.y_axis_title(label);
    }
    if let Some(markers) = options.markers {
        builder = builder.markers(markers);
    }
    if let Some(width) = options.line_width {
        builder = builder.line_width(SVLength::absolute(width));
    }
    let plot = builder.build()?;
    let output = plot.render()?;
    Ok(SVPythonRenderOutput::new(
        output.backend_kind,
        specification.data.representation(),
        output.bytes,
    ))
}

fn sv_render_area(
    specification: &SVPythonFigureSpec,
    backend_kind: SVBackendKind,
) -> Result<SVPythonRenderOutput, SVError> {
    let options = &specification.options;
    let kind = specification.kind;
    let x = SVPythonPlotOptions::require(options.x.as_deref(), "x", kind)?;
    let y = SVPythonPlotOptions::require(options.y.as_deref(), "y", kind)?;
    let mut builder = SVAreaPlotBuilder::new()
        .data_source(&specification.data)
        .x(&x)
        .y(&y)
        .backend(backend_kind)
        .execution_mode(specification.execution_mode);
    if let Some(series) = options.series.as_deref() {
        builder = builder.series(series);
    }
    if let Some(title) = options.title.as_deref() {
        builder = builder.title(title);
    }
    if let Some(label) = options.x_label.as_deref() {
        builder = builder.x_title(label);
    }
    if let Some(label) = options.y_label.as_deref() {
        builder = builder.y_title(label);
    }
    if let Some(opacity) = options.opacity {
        builder = builder.opacity(opacity as f32);
    }
    let plot = builder.build()?;
    let output = plot.render()?;
    Ok(SVPythonRenderOutput::new(
        output.backend_kind,
        specification.data.representation(),
        output.bytes,
    ))
}

fn sv_render_histogram(
    specification: &SVPythonFigureSpec,
    backend_kind: SVBackendKind,
) -> Result<SVPythonRenderOutput, SVError> {
    let options = &specification.options;
    let kind = specification.kind;
    let value = SVPythonPlotOptions::require(options.value.as_deref(), "value", kind)?;
    let mut builder = SVHistogramPlot::builder()
        .data_source(&specification.data)
        .value(value)
        .backend(backend_kind)
        .execution_mode(specification.execution_mode);
    if let Some(series) = options.series.as_deref() {
        builder = builder.series(series);
    }
    if let Some(title) = options.title.as_deref() {
        builder = builder.title(title);
    }
    if let Some(label) = options.x_label.as_deref() {
        builder = builder.x_label(label);
    }
    if let Some(label) = options.y_label.as_deref() {
        builder = builder.y_label(label);
    }
    if let Some(bin_count) = options.bin_count {
        builder = builder.bin_count(bin_count);
    }
    let plot = builder.build()?;
    let output = plot.render()?;
    Ok(SVPythonRenderOutput::new(
        output.backend_kind(),
        plot.representation(),
        output.bytes().to_vec(),
    ))
}
