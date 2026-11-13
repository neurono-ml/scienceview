//! Dashboard composition at the Python boundary.
//!
//! A Python caller declares a grid and a list of panels; each panel embeds one
//! bound figure, rendered through the very same conversion layer a single
//! figure uses. The result is one self-contained SVG document, exactly as the
//! Rust dashboard produces.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::SVRepresentation;
use scienceview::sv_error::SVError;
use scienceview_dashboard::sv_dashboard::SVDashboardBuilder;
use scienceview_dashboard::sv_grid::{SVDashboardGrid, SVGridPlacement};
use scienceview_dashboard::sv_panel::{
    SVDashboardPanelBuilder, SVDashboardPanelContent, SVPanelRenderRequest,
};
use scienceview_dashboard::sv_source::SVSharedSource;

use super::backend::sv_require_backend_kind;
use super::data_source::SVPythonData;
use super::figure_spec::SVPythonFigureSpec;
use super::operation::sv_render_python_figure_svg;
use super::render_output::SVPythonRenderOutput;

/// One panel of a Python dashboard: a grid placement and one bound figure.
#[derive(Clone, Debug)]
pub struct SVPythonDashboardPanel {
    /// The stable panel identifier.
    pub identifier: String,
    /// The accessible panel title; defaults to the identifier.
    pub title: Option<String>,
    /// The zero-based grid row.
    pub row: usize,
    /// The zero-based grid column.
    pub column: usize,
    /// How many rows the panel spans.
    pub row_span: usize,
    /// How many columns the panel spans.
    pub column_span: usize,
    /// The figure the panel draws.
    pub figure: SVPythonFigureSpec,
}

impl SVPythonDashboardPanel {
    /// Builds a one-cell panel at `(row, column)`.
    pub fn new(
        identifier: impl Into<String>,
        row: usize,
        column: usize,
        figure: SVPythonFigureSpec,
    ) -> Self {
        Self {
            identifier: identifier.into(),
            title: None,
            row,
            column,
            row_span: 1,
            column_span: 1,
            figure,
        }
    }

    /// Spans the panel over several cells.
    pub fn with_span(mut self, row_span: usize, column_span: usize) -> Self {
        self.row_span = row_span;
        self.column_span = column_span;
        self
    }

    /// Sets the accessible title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

/// A composition of panels over a declared grid.
#[derive(Clone, Debug)]
pub struct SVPythonDashboardSpec {
    /// The number of grid rows.
    pub rows: usize,
    /// The number of grid columns.
    pub columns: usize,
    /// The explicit backend selection.
    pub backend: Option<SVBackendKind>,
    /// The optional document title.
    pub title: Option<String>,
    /// The panels, in declaration order.
    pub panels: Vec<SVPythonDashboardPanel>,
}

impl SVPythonDashboardSpec {
    /// Starts an empty composition.
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows,
            columns,
            backend: None,
            title: None,
            panels: Vec::new(),
        }
    }

    /// Selects the backend kind explicitly.
    pub fn with_backend(mut self, backend_kind: SVBackendKind) -> Self {
        self.backend = Some(backend_kind);
        self
    }

    /// Sets the document title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Appends a panel.
    pub fn with_panel(mut self, panel: SVPythonDashboardPanel) -> Self {
        self.panels.push(panel);
        self
    }
}

/// Renders the composition to one self-contained document.
///
/// # Errors
///
/// Returns [`SVError::MissingBackendSelection`] without an explicit backend,
/// [`SVError::InvalidConfiguration`] for an empty or malformed composition and
/// every panel, layout or backend error of the dashboard.
pub fn sv_render_python_dashboard(
    specification: &SVPythonDashboardSpec,
) -> Result<SVPythonRenderOutput, SVError> {
    let backend_kind = sv_require_backend_kind(specification.backend)?;
    if specification.panels.is_empty() {
        return Err(SVError::InvalidConfiguration {
            message: "a dashboard needs at least one panel".to_string(),
        });
    }
    let grid = SVDashboardGrid::new(specification.rows, specification.columns)?;
    let sources: Vec<SVPythonData> = specification
        .panels
        .iter()
        .map(|panel| panel.figure.data.clone())
        .collect();
    let identifiers: Vec<String> = specification
        .panels
        .iter()
        .map(|panel| panel.identifier.clone())
        .collect();

    let mut builder = SVDashboardBuilder::new().grid(grid).backend(backend_kind);
    if let Some(title) = specification.title.as_deref() {
        builder = builder.title(title);
    }
    for (identifier, source) in identifiers.iter().zip(sources.iter()) {
        builder = builder.source(SVSharedSource::new(identifier, source));
    }
    for panel in &specification.panels {
        let placement = SVGridPlacement::new(panel.row, panel.column)
            .with_row_span(panel.row_span)
            .with_column_span(panel.column_span);
        let built = SVDashboardPanelBuilder::new()
            .identifier(panel.identifier.clone())
            .title(
                panel
                    .title
                    .clone()
                    .unwrap_or_else(|| panel.identifier.clone()),
            )
            .placement(placement)
            .source(panel.identifier.clone())
            .content(SVPythonPanelContent {
                figure: panel.figure.clone(),
            })
            .build()?;
        builder = builder.panel(built);
    }
    let dashboard = builder.build()?;
    let output = dashboard.render()?;
    Ok(SVPythonRenderOutput::new(
        backend_kind,
        SVRepresentation::Dense,
        output.bytes().to_vec(),
    ))
}

/// The dashboard panel content that embeds one bound figure.
struct SVPythonPanelContent {
    figure: SVPythonFigureSpec,
}

impl SVDashboardPanelContent for SVPythonPanelContent {
    fn render_svg(&self, _request: &SVPanelRenderRequest<'_>) -> Result<String, SVError> {
        sv_render_python_figure_svg(&self.figure)
    }
}
