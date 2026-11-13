//! The built dashboard: composition state, refresh cycles and filter exposure.

use super::SVDashboardBuilder;
use super::linked_axis::SVLinkedAxis;
use crate::sv_backend::SVBackendKind;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_grid::{SVDashboardGrid, SVGridSolution};
use crate::sv_layout::SVViewport;
use crate::sv_panel::SVDashboardPanel;
use crate::sv_selection::{SVLinkedSelectionState, SVSelectionFilter};
use crate::sv_source::SVSharedSource;

/// A composed dashboard.
///
/// The dashboard owns the grid, the panels, the borrowed shared sources and the
/// linked state. Rendering, event routing and refresh are pure operations on
/// that state; no wall clock, async runtime or global registry is involved.
pub struct SVDashboard<'a> {
    pub(super) grid: SVDashboardGrid,
    pub(super) viewport: SVViewport,
    pub(super) title: String,
    pub(super) description: Option<String>,
    pub(super) sources: Vec<SVSharedSource<'a>>,
    pub(super) panels: Vec<SVDashboardPanel<'a>>,
    pub(super) linked_axes: Vec<SVLinkedAxis>,
    pub(super) selection: SVLinkedSelectionState,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) refresh_sequence: u64,
}

impl<'a> SVDashboard<'a> {
    /// Starts building a dashboard.
    pub fn builder() -> SVDashboardBuilder<'a> {
        SVDashboardBuilder::new()
    }

    /// Returns the configured backend kind.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// Returns the configured execution mode.
    pub fn execution_mode(&self) -> SVExecutionMode {
        self.execution_mode
    }

    /// Returns the dashboard title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the dashboard description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the grid.
    pub fn grid(&self) -> SVDashboardGrid {
        self.grid
    }

    /// Returns the current viewport.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the panel identifiers in declaration order.
    pub fn panel_identifiers(&self) -> Vec<&str> {
        self.panels
            .iter()
            .map(SVDashboardPanel::identifier)
            .collect()
    }

    /// Returns the shared source identifiers in registration order.
    pub fn source_identifiers(&self) -> Vec<&str> {
        self.sources
            .iter()
            .map(SVSharedSource::identifier)
            .collect()
    }

    /// Returns the linked axis identifiers in registration order.
    pub fn linked_axis_identifiers(&self) -> Vec<&str> {
        self.linked_axes
            .iter()
            .map(SVLinkedAxis::identifier)
            .collect()
    }

    /// Returns one linked axis.
    pub fn linked_axis(&self, identifier: &str) -> Option<&SVLinkedAxis> {
        self.linked_axes
            .iter()
            .find(|axis| axis.identifier() == identifier)
    }

    /// Returns the current filter state exposed to the host.
    pub fn filters(&self) -> &std::collections::BTreeMap<String, SVSelectionFilter> {
        self.selection.filters()
    }

    /// Returns the filter of one shared source.
    pub fn filter_for(&self, source_identifier: &str) -> Option<&SVSelectionFilter> {
        self.selection.filter_for(source_identifier)
    }

    /// Returns the filter a panel's shared source currently carries.
    pub fn filter_for_panel(&self, panel_identifier: &str) -> Option<&SVSelectionFilter> {
        let panel = self
            .panels
            .iter()
            .find(|panel| panel.identifier() == panel_identifier)?;
        self.selection.filter_for(panel.source_identifier())
    }

    /// Changes the viewport so the next render re-solves every cell.
    pub fn set_viewport(&mut self, viewport: SVViewport) {
        self.viewport = viewport;
    }

    /// Solves the grid against the current viewport.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidDashboardGrid`] for an invalid gap.
    pub fn solve_layout(&self) -> Result<SVGridSolution, SVError> {
        self.grid.solve(self.viewport)
    }
}

impl std::fmt::Debug for SVDashboard<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVDashboard")
            .field("backend_kind", &self.backend_kind)
            .field("panels", &self.panels.len())
            .field("sources", &self.sources.len())
            .field("viewport", &self.viewport)
            .finish_non_exhaustive()
    }
}
