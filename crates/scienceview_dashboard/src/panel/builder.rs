//! The validating builder of one dashboard panel.

use super::{SVDashboardAxisBinding, SVDashboardPanel, SVDashboardPanelContent};
use crate::sv_control::SVControlAxis;
use crate::sv_error::SVError;
use crate::sv_grid::SVGridPlacement;

/// Builds one dashboard panel.
///
/// The builder is the only public constructor. Identifier, placement, shared
/// source and content are required; the title defaults to the identifier and a
/// panel may bind no axis at all.
pub struct SVDashboardPanelBuilder<'a> {
    identifier: Option<String>,
    title: Option<String>,
    description: Option<String>,
    placement: Option<SVGridPlacement>,
    source_identifier: Option<String>,
    axes: Vec<SVDashboardAxisBinding>,
    content: Option<Box<dyn SVDashboardPanelContent + Send + Sync + 'a>>,
}

impl<'a> SVDashboardPanelBuilder<'a> {
    /// Starts an empty panel.
    pub fn new() -> Self {
        Self {
            identifier: None,
            title: None,
            description: None,
            placement: None,
            source_identifier: None,
            axes: Vec::new(),
            content: None,
        }
    }

    /// Sets the stable panel identifier.
    #[must_use]
    pub fn identifier(mut self, identifier: impl Into<String>) -> Self {
        self.identifier = Some(identifier.into());
        self
    }

    /// Sets the accessible title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the accessible description.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Places the panel in the grid.
    #[must_use]
    pub fn placement(mut self, placement: SVGridPlacement) -> Self {
        self.placement = Some(placement);
        self
    }

    /// Binds the panel to a shared source identifier.
    #[must_use]
    pub fn source(mut self, source_identifier: impl Into<String>) -> Self {
        self.source_identifier = Some(source_identifier.into());
        self
    }

    /// Binds a local axis to a dashboard linked axis and a source column.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] for an empty identifier or
    /// column name.
    pub fn axis(
        mut self,
        identifier: impl Into<String>,
        axis: SVControlAxis,
        column: impl Into<String>,
    ) -> Result<Self, SVError> {
        self.axes
            .push(SVDashboardAxisBinding::new(identifier, axis, column)?);
        Ok(self)
    }

    /// Sets the object that draws the panel.
    #[must_use]
    pub fn content(mut self, content: impl SVDashboardPanelContent + 'a) -> Self {
        self.content = Some(Box::new(content));
        self
    }

    /// Validates and builds the panel.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] for a missing required step or
    /// an empty identifier, and [`SVError::DuplicateDashboardAxis`] for an axis
    /// bound twice in the same panel.
    pub fn build(self) -> Result<SVDashboardPanel<'a>, SVError> {
        let identifier = sv_required_identifier(self.identifier)?;
        let placement = self
            .placement
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: format!(
                    "the panel `{identifier}` needs a placement; pass it with `placement(...)`"
                ),
            })?;
        let source_identifier =
            self.source_identifier
                .ok_or_else(|| SVError::InvalidConfiguration {
                    message: format!(
                        "the panel `{identifier}` needs a shared source; pass it with `source(...)`"
                    ),
                })?;
        if source_identifier.is_empty() {
            return Err(SVError::InvalidConfiguration {
                message: format!("the panel `{identifier}` binds an empty source identifier"),
            });
        }
        let content = self.content.ok_or_else(|| SVError::InvalidConfiguration {
            message: format!("the panel `{identifier}` needs content; pass it with `content(...)`"),
        })?;
        for (position, binding) in self.axes.iter().enumerate() {
            if self.axes[..position]
                .iter()
                .any(|earlier| earlier.identifier() == binding.identifier())
            {
                return Err(SVError::DuplicateDashboardAxis {
                    axis: binding.identifier().to_owned(),
                });
            }
        }

        Ok(SVDashboardPanel {
            title: self.title.unwrap_or_else(|| identifier.clone()),
            identifier,
            description: self.description,
            placement,
            source_identifier,
            axes: self.axes,
            content,
        })
    }
}

impl Default for SVDashboardPanelBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

fn sv_required_identifier(identifier: Option<String>) -> Result<String, SVError> {
    match identifier {
        Some(identifier) if !identifier.is_empty() => Ok(identifier),
        Some(_) => Err(SVError::InvalidConfiguration {
            message: "a dashboard panel needs a non-empty identifier".to_string(),
        }),
        None => Err(SVError::InvalidConfiguration {
            message: "a dashboard panel needs an identifier; pass it with `identifier(...)`"
                .to_string(),
        }),
    }
}
