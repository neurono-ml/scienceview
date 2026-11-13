//! Test fixtures: a deterministic panel content and a counting source.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use crate::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema,
};
use crate::sv_error::SVError;
use crate::sv_panel::{SVDashboardPanelContent, SVPanelRenderRequest};

/// What one panel render observed.
#[derive(Clone, Debug, PartialEq)]
pub struct SVPanelObservation {
    /// The observed width.
    pub width: f64,
    /// The observed height.
    pub height: f64,
    /// The filter source identifier, when a filter was applied.
    pub filter_source: Option<String>,
}

/// A deterministic panel that records the requests it receives.
pub struct SVTestPanelContent {
    pub(super) label: String,
    pub(super) observations: Arc<Mutex<Vec<SVPanelObservation>>>,
}

impl SVTestPanelContent {
    /// Builds a content labelled with the panel name.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            observations: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Returns the shared observation log.
    pub fn observations(&self) -> Arc<Mutex<Vec<SVPanelObservation>>> {
        Arc::clone(&self.observations)
    }
}

impl SVDashboardPanelContent for SVTestPanelContent {
    fn render_svg(&self, request: &SVPanelRenderRequest<'_>) -> Result<String, SVError> {
        if let Ok(mut observations) = self.observations.lock() {
            observations.push(SVPanelObservation {
                width: request.viewport().width(),
                height: request.viewport().height(),
                filter_source: request
                    .filter()
                    .map(|filter| filter.source_identifier().to_owned()),
            });
        }
        Ok(format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\"><title>{}</title><rect x=\"0\" y=\"0\" width=\"100\" height=\"100\"/></svg>",
            self.label
        ))
    }
}

/// A source that records how many times a column was resolved.
pub struct SVCountingSource {
    x: Vec<f64>,
    y: Vec<f64>,
    resolutions: AtomicUsize,
}

impl SVCountingSource {
    /// Builds a source with two columns.
    pub fn new() -> Self {
        Self {
            x: vec![0.0, 1.0, 2.0, 3.0],
            y: vec![3.0, 2.0, 1.0, 0.0],
            resolutions: AtomicUsize::new(0),
        }
    }

    /// Returns the number of column resolutions.
    pub fn resolutions(&self) -> usize {
        self.resolutions.load(Ordering::Relaxed)
    }

    /// Returns the borrowed horizontal values.
    pub fn x(&self) -> &[f64] {
        &self.x
    }
}

impl Default for SVCountingSource {
    fn default() -> Self {
        Self::new()
    }
}

impl SVDataSource for SVCountingSource {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::owned(vec![
            SVColumnDescriptor::new("x", SVColumnRole::X),
            SVColumnDescriptor::new("y", SVColumnRole::Y),
        ])
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        self.resolutions.fetch_add(1, Ordering::Relaxed);
        match name {
            "x" => Ok(SVColumnSnapshot::Borrowed(SVColumnView::Dense(&self.x))),
            "y" => Ok(SVColumnSnapshot::Borrowed(SVColumnView::Dense(&self.y))),
            other => Err(SVError::ColumnNotFound {
                name: other.to_owned(),
            }),
        }
    }
}

/// Builds one panel bound to the shared horizontal and vertical axes.
pub fn sv_panel_with_axes<'a>(
    identifier: &str,
    row: usize,
    column: usize,
    column_span: usize,
) -> crate::sv_panel::SVDashboardPanel<'a> {
    crate::sv_panel::SVDashboardPanel::builder()
        .identifier(identifier)
        .placement(crate::sv_grid::SVGridPlacement::new(row, column).with_column_span(column_span))
        .source("measurements")
        .axis(
            "shared-x",
            crate::sv_control::SVControlAxis::Horizontal,
            "x",
        )
        .expect("the horizontal binding is valid")
        .axis("shared-y", crate::sv_control::SVControlAxis::Vertical, "y")
        .expect("the vertical binding is valid")
        .content(SVTestPanelContent::new(identifier))
        .build()
        .expect("the panel is complete")
}

/// Builds a dashboard with two panels sharing one source and one axis.
pub fn sv_two_panel_dashboard(source: &SVCountingSource) -> crate::sv_dashboard::SVDashboard<'_> {
    crate::sv_dashboard::SVDashboardBuilder::new()
        .grid(crate::sv_grid::SVDashboardGrid::new(1, 2).expect("the grid is valid"))
        .source(crate::sv_source::SVSharedSource::new(
            "measurements",
            source,
        ))
        .linked_axis(
            "shared-x",
            crate::sv_control::SVControlAxis::Horizontal,
            sv_domain(),
        )
        .panel(
            crate::sv_panel::SVDashboardPanel::builder()
                .identifier("left")
                .placement(crate::sv_grid::SVGridPlacement::new(0, 0))
                .source("measurements")
                .axis(
                    "shared-x",
                    crate::sv_control::SVControlAxis::Horizontal,
                    "x",
                )
                .expect("the binding is valid")
                .content(SVTestPanelContent::new("left"))
                .build()
                .expect("the panel is complete"),
        )
        .panel(
            crate::sv_panel::SVDashboardPanel::builder()
                .identifier("right")
                .placement(crate::sv_grid::SVGridPlacement::new(0, 1))
                .source("measurements")
                .axis(
                    "shared-x",
                    crate::sv_control::SVControlAxis::Horizontal,
                    "x",
                )
                .expect("the binding is valid")
                .content(SVTestPanelContent::new("right"))
                .build()
                .expect("the panel is complete"),
        )
        .backend(crate::sv_backend::SVBackendKind::Svg)
        .build()
        .expect("the dashboard is valid")
}

/// Returns a valid scale domain used by the test axes.
pub fn sv_domain() -> crate::sv_scale::SVDomain {
    crate::sv_scale::SVDomain::new(0.0, 10.0).expect("the domain is valid")
}

/// Returns a region over the horizontal test interval and the full vertical one.
pub fn sv_region(lower: f64, upper: f64) -> crate::sv_control::SVSelectionRegion {
    crate::sv_control::SVSelectionRegion::new(
        crate::sv_control::SVControlRange::new(lower, upper).expect("an ordered range"),
        crate::sv_control::SVControlRange::new(0.0, 10.0).expect("an ordered range"),
    )
}
