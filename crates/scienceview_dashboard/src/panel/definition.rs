//! The panel model: what a cell hosts and what the content is asked to draw.

use crate::sv_control::SVControlAxis;
use crate::sv_error::SVError;
use crate::sv_grid::SVGridPlacement;
use crate::sv_layout::SVViewport;
use crate::sv_selection::SVSelectionFilter;

/// The environment one panel is rendered in.
///
/// The request is read-only: it names the cell, the shared source, the current
/// filter and the viewport the panel content must draw into. A content
/// implementation never owns dashboard state.
#[derive(Clone, Copy, Debug)]
pub struct SVPanelRenderRequest<'a> {
    panel_identifier: &'a str,
    source_identifier: &'a str,
    viewport: SVViewport,
    filter: Option<&'a SVSelectionFilter>,
}

impl<'a> SVPanelRenderRequest<'a> {
    pub(crate) fn new(
        panel_identifier: &'a str,
        source_identifier: &'a str,
        viewport: SVViewport,
        filter: Option<&'a SVSelectionFilter>,
    ) -> Self {
        Self {
            panel_identifier,
            source_identifier,
            viewport,
            filter,
        }
    }

    /// Returns the panel identifier.
    pub fn panel_identifier(&self) -> &'a str {
        self.panel_identifier
    }

    /// Returns the shared source the panel is bound to.
    pub fn source_identifier(&self) -> &'a str {
        self.source_identifier
    }

    /// Returns the viewport the panel must draw into.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the linked filter currently applied to the shared source.
    pub fn filter(&self) -> Option<&'a SVSelectionFilter> {
        self.filter
    }
}

/// What a dashboard cell draws.
///
/// The content returns one self-contained `<svg>` element with a `viewBox`; the
/// dashboard places it into the solved cell rectangle and keeps the viewBox, so
/// the same content scales to every viewport without re-deriving geometry.
pub trait SVDashboardPanelContent: Send + Sync {
    /// Renders the panel into one SVG fragment.
    ///
    /// # Errors
    ///
    /// Returns the content's own composition error.
    fn render_svg(&self, request: &SVPanelRenderRequest<'_>) -> Result<String, SVError>;
}

/// How one panel binds a local control axis to a dashboard linked axis.
///
/// The identifier is the dashboard-wide axis name that coordinated events
/// travel on; the column is the shared-source column a selection in this panel
/// becomes a filter over.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SVDashboardAxisBinding {
    identifier: String,
    axis: SVControlAxis,
    column: String,
}

impl SVDashboardAxisBinding {
    /// Binds a linked axis identifier to a local axis and a source column.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] for an empty identifier or
    /// column name.
    pub fn new(
        identifier: impl Into<String>,
        axis: SVControlAxis,
        column: impl Into<String>,
    ) -> Result<Self, SVError> {
        let identifier = identifier.into();
        let column = column.into();
        if identifier.is_empty() || column.is_empty() {
            return Err(SVError::InvalidConfiguration {
                message: "a dashboard axis binding needs a non-empty identifier and column"
                    .to_string(),
            });
        }
        Ok(Self {
            identifier,
            axis,
            column,
        })
    }

    /// Returns the linked axis identifier.
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns the local control axis.
    pub fn axis(&self) -> SVControlAxis {
        self.axis
    }

    /// Returns the shared-source column a selection maps to.
    pub fn column(&self) -> &str {
        &self.column
    }
}

/// One cell of a dashboard.
///
/// A panel is data plus borrowed content: its placement, title, shared source,
/// axis bindings and the object that draws it. Direct construction is private;
/// [`SVDashboardPanelBuilder`](super::SVDashboardPanelBuilder) is the only entry
/// point, so an invalid panel cannot exist.
pub struct SVDashboardPanel<'a> {
    pub(super) identifier: String,
    pub(super) title: String,
    pub(super) description: Option<String>,
    pub(super) placement: SVGridPlacement,
    pub(super) source_identifier: String,
    pub(super) axes: Vec<SVDashboardAxisBinding>,
    pub(super) content: Box<dyn SVDashboardPanelContent + Send + Sync + 'a>,
}

impl<'a> SVDashboardPanel<'a> {
    /// Starts building a panel.
    pub fn builder() -> super::SVDashboardPanelBuilder<'a> {
        super::SVDashboardPanelBuilder::new()
    }

    /// Returns the panel identifier.
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns the accessible title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the accessible description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the grid placement.
    pub fn placement(&self) -> SVGridPlacement {
        self.placement
    }

    /// Returns the shared source the panel reads.
    pub fn source_identifier(&self) -> &str {
        &self.source_identifier
    }

    /// Returns the axis bindings in declaration order.
    pub fn axes(&self) -> &[SVDashboardAxisBinding] {
        &self.axes
    }

    /// Returns the binding of one linked axis identifier.
    pub fn axis_binding(&self, identifier: &str) -> Option<&SVDashboardAxisBinding> {
        self.axes.iter().find(|binding| binding.identifier() == identifier)
    }

    /// Renders the panel through its content.
    ///
    /// # Errors
    ///
    /// Propagates the content error.
    pub fn render_svg(&self, request: &SVPanelRenderRequest<'_>) -> Result<String, SVError> {
        self.content.render_svg(request)
    }
}

impl std::fmt::Debug for SVDashboardPanel<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVDashboardPanel")
            .field("identifier", &self.identifier)
            .field("placement", &self.placement)
            .field("source_identifier", &self.source_identifier)
            .field("axes", &self.axes.len())
            .finish_non_exhaustive()
    }
}
