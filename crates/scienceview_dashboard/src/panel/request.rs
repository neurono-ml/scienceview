//! What a panel content is asked to draw.

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
