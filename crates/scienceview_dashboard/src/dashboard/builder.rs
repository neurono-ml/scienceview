//! The validating builder of a whole dashboard.

use super::core_implementation::SVDashboard;
use super::linked_axis::SVLinkedAxisRegistration;
use super::validation::{
    sv_validate_linked_axes, sv_validate_no_overlap, sv_validate_panel_references,
    sv_validate_unique_panels, sv_validate_unique_sources,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_control::SVControlAxis;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_grid::SVDashboardGrid;
use crate::sv_layout::SVViewport;
use crate::sv_panel::SVDashboardPanel;
use crate::sv_scale::SVDomain;
use crate::sv_source::SVSharedSource;

/// Builds a [`SVDashboard`].
///
/// The builder is the only public constructor. The builder requires an explicit
/// backend kind — there is no automatic backend — and defaults the execution
/// mode to [`SVExecutionMode::Automatic`]. Every cross-reference between panels,
/// shared sources and linked axes is validated at `build`, so a dashboard that
/// exists can route events and render.
pub struct SVDashboardBuilder<'a> {
    pub(super) grid: SVDashboardGrid,
    pub(super) viewport: SVViewport,
    pub(super) title: String,
    pub(super) description: Option<String>,
    pub(super) sources: Vec<SVSharedSource<'a>>,
    pub(super) panels: Vec<SVDashboardPanel<'a>>,
    pub(super) linked_axes: Vec<SVLinkedAxisRegistration>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) cascade_depth_budget: usize,
}

impl<'a> SVDashboardBuilder<'a> {
    /// Starts a one-cell dashboard with no panel and no source.
    pub fn new() -> Self {
        Self {
            grid: SVDashboardGrid::new(1, 1).expect("a one-cell grid is valid"),
            viewport: SVViewport::new(800.0, 600.0),
            title: "scienceview dashboard".to_string(),
            description: None,
            sources: Vec::new(),
            panels: Vec::new(),
            linked_axes: Vec::new(),
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
            cascade_depth_budget: 8,
        }
    }

    /// Sets the grid shape and spacing.
    #[must_use]
    pub fn grid(mut self, grid: SVDashboardGrid) -> Self {
        self.grid = grid;
        self
    }

    /// Sets the dashboard viewport.
    #[must_use]
    pub fn viewport(mut self, viewport: SVViewport) -> Self {
        self.viewport = viewport;
        self
    }

    /// Sets the accessible dashboard title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets the accessible dashboard description.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Registers a shared source.
    #[must_use]
    pub fn source(mut self, source: SVSharedSource<'a>) -> Self {
        self.sources.push(source);
        self
    }

    /// Adds a panel.
    #[must_use]
    pub fn panel(mut self, panel: SVDashboardPanel<'a>) -> Self {
        self.panels.push(panel);
        self
    }

    /// Registers a linked axis other panels can bind to.
    #[must_use]
    pub fn linked_axis(
        mut self,
        identifier: impl Into<String>,
        axis: SVControlAxis,
        domain: SVDomain,
    ) -> Self {
        self.linked_axes.push(SVLinkedAxisRegistration::new(
            identifier.into(),
            axis,
            domain,
        ));
        self
    }

    /// Selects the rendering backend. Required.
    #[must_use]
    pub fn backend(mut self, backend_kind: SVBackendKind) -> Self {
        self.backend_kind = Some(backend_kind);
        self
    }

    /// Sets the execution mode.
    #[must_use]
    pub fn execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }

    /// Sets how deep a linked-selection cascade may go before it is stopped.
    #[must_use]
    pub fn cascade_depth_budget(mut self, cascade_depth_budget: usize) -> Self {
        self.cascade_depth_budget = cascade_depth_budget;
        self
    }

    /// Validates the composition and builds the dashboard.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingBackendSelection`] when no backend was chosen,
    /// a duplicate or unknown reference error, a placement error, an overlap
    /// error or an axis-direction mismatch.
    pub fn build(self) -> Result<SVDashboard<'a>, SVError> {
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        sv_validate_unique_sources(&self.sources)?;
        sv_validate_unique_panels(&self.panels)?;
        sv_validate_linked_axes(&self.linked_axes)?;
        for panel in &self.panels {
            panel.placement().validate(
                panel.identifier(),
                self.grid.number_of_rows(),
                self.grid.number_of_columns(),
            )?;
            sv_validate_panel_references(panel, &self.sources, &self.linked_axes)?;
        }
        sv_validate_no_overlap(&self.panels)?;

        let linked_axes = self
            .linked_axes
            .into_iter()
            .map(|registration| registration.into_linked_axis(&self.panels))
            .collect::<Result<Vec<_>, SVError>>()?;

        Ok(SVDashboard {
            grid: self.grid,
            viewport: self.viewport,
            title: self.title,
            description: self.description,
            sources: self.sources,
            panels: self.panels,
            linked_axes,
            selection: crate::sv_selection::SVLinkedSelectionState::new(self.cascade_depth_budget),
            backend_kind,
            execution_mode: self.execution_mode,
            refresh_sequence: 0,
        })
    }
}

impl Default for SVDashboardBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
