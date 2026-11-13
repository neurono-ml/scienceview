//! The backend-agnostic result of one parallel categories composition.
//!
//! The scene owns no notation: it carries the solved viewport and plot
//! rectangle, the decorations, the ribbon geometries and a data-space summary
//! of every axis and every aggregated ribbon.

use crate::sv_backend::SVBackendKind;
use crate::sv_decoration::SVDecorationPrimitive;
#[cfg(any(feature = "render-svg", feature = "render-native"))]
use crate::sv_error::SVError;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_mark::SVMarkGeometry;
use crate::sv_scale::SVColour;

/// One categorical axis of a parallel categories plot, in data space.
#[derive(Clone, Debug, PartialEq)]
pub struct SVParallelCategoriesAxis {
    pub(super) name: String,
    pub(super) label: String,
    pub(super) categories: Vec<f64>,
    pub(super) category_labels: Vec<String>,
}

impl SVParallelCategoriesAxis {
    /// Returns the source column name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the display title of the axis.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the category values in display order.
    pub fn categories(&self) -> &[f64] {
        &self.categories
    }

    /// Returns the display label of every category, in display order.
    pub fn category_labels(&self) -> &[String] {
        &self.category_labels
    }

    /// Returns the number of category bands.
    pub fn number_of_categories(&self) -> usize {
        self.categories.len()
    }
}

/// One aggregated ribbon between two adjacent axes, in data space.
#[derive(Clone, Debug, PartialEq)]
pub struct SVParallelCategoriesRibbon {
    pub(super) left_variable: String,
    pub(super) right_variable: String,
    pub(super) left_category: f64,
    pub(super) right_category: f64,
    pub(super) count: u64,
    pub(super) colour: SVColour,
}

impl SVParallelCategoriesRibbon {
    /// Returns the source column the ribbon leaves.
    pub fn left_variable(&self) -> &str {
        &self.left_variable
    }

    /// Returns the source column the ribbon reaches.
    pub fn right_variable(&self) -> &str {
        &self.right_variable
    }

    /// Returns the category value the ribbon leaves.
    pub fn left_category(&self) -> f64 {
        self.left_category
    }

    /// Returns the category value the ribbon reaches.
    pub fn right_category(&self) -> f64 {
        self.right_category
    }

    /// Returns the number of observations the ribbon aggregates.
    pub fn count(&self) -> u64 {
        self.count
    }

    /// Returns the ribbon colour.
    pub fn colour(&self) -> SVColour {
        self.colour
    }
}

/// Everything one parallel categories composition produced.
#[derive(Clone, Debug, PartialEq)]
pub struct SVParallelCategoriesScene {
    pub(super) backend_kind: SVBackendKind,
    pub(super) viewport: SVViewport,
    pub(super) plot_area: SVResolvedRectangle,
    pub(super) background: SVColour,
    pub(super) title: Option<String>,
    pub(super) description: Option<String>,
    pub(super) decoration_primitives_before_marks: Vec<SVDecorationPrimitive>,
    pub(super) mark_geometries: Vec<SVMarkGeometry<'static>>,
    pub(super) decoration_primitives_after_marks: Vec<SVDecorationPrimitive>,
    pub(super) axes: Vec<SVParallelCategoriesAxis>,
    pub(super) ribbons: Vec<SVParallelCategoriesRibbon>,
    pub(super) number_of_observations: usize,
}

impl SVParallelCategoriesScene {
    /// Returns the backend kind the scene was composed for.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// Returns the viewport the scene was solved for.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the rectangle the ribbons occupy.
    pub fn plot_area(&self) -> SVResolvedRectangle {
        self.plot_area
    }

    /// Returns the background colour.
    pub fn background(&self) -> SVColour {
        self.background
    }

    /// Returns the scene title, when the builder set one.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the accessible description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the decorations drawn below the ribbons: the title.
    pub fn decoration_primitives_before_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decoration_primitives_before_marks
    }

    /// Returns the ribbon geometries in draw order.
    pub fn mark_geometries(&self) -> &[SVMarkGeometry<'static>] {
        &self.mark_geometries
    }

    /// Returns the decorations drawn above the ribbons: axes and labels.
    pub fn decoration_primitives_after_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decoration_primitives_after_marks
    }

    /// Returns the categorical axes, in axis order.
    pub fn axes(&self) -> &[SVParallelCategoriesAxis] {
        &self.axes
    }

    /// Returns the aggregated ribbons, in first-appearance order.
    pub fn ribbons(&self) -> &[SVParallelCategoriesRibbon] {
        &self.ribbons
    }

    /// Returns the number of axes.
    pub fn number_of_axes(&self) -> usize {
        self.axes.len()
    }

    /// Returns the number of aggregated ribbons.
    pub fn number_of_ribbons(&self) -> usize {
        self.ribbons.len()
    }

    /// Returns the number of observations the ribbons aggregate.
    pub fn number_of_observations(&self) -> usize {
        self.number_of_observations
    }

    /// Returns the number of primitives the mark kernel produced.
    pub fn number_of_mark_primitives(&self) -> usize {
        self.mark_geometries.iter().map(SVMarkGeometry::len).sum()
    }

    /// Serializes the scene as a self-contained SVG document.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::UnsupportedBackendCapability`] when the scene was
    /// composed for a kind other than `Svg`.
    #[cfg(feature = "render-svg")]
    pub fn to_svg(&self) -> Result<String, SVError> {
        if self.backend_kind != SVBackendKind::Svg {
            return Err(SVError::UnsupportedBackendCapability {
                backend_kind: self.backend_kind,
                capability: crate::sv_backend::SVBackendCapability::Export,
            });
        }

        let mut document = scienceview_svg::SVSvgDocument::new(self.viewport)
            .with_title(
                self.title
                    .clone()
                    .unwrap_or_else(|| "parallel categories plot".to_string()),
            )
            .with_description(
                self.description
                    .clone()
                    .unwrap_or_else(|| "parallel categories plot".to_string()),
            )
            .with_background(self.background);
        document.push_decoration_primitives(&self.decoration_primitives_before_marks);
        for geometry in &self.mark_geometries {
            document.push_mark_geometry(geometry);
        }
        document.push_decoration_primitives(&self.decoration_primitives_after_marks);
        Ok(document.to_svg())
    }

    /// Rasterizes the scene with the from-scratch native renderer.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::UnsupportedBackendCapability`] when the scene was
    /// composed for a kind other than `Native`.
    #[cfg(feature = "render-native")]
    pub fn to_native_frame(&self) -> Result<scienceview_native::SVNativeFrame, SVError> {
        if self.backend_kind != SVBackendKind::Native {
            return Err(SVError::UnsupportedBackendCapability {
                backend_kind: self.backend_kind,
                capability: crate::sv_backend::SVBackendCapability::Presentation,
            });
        }

        let mut canvas = scienceview_native::SVRasterCanvas::new(&self.viewport);
        canvas.fill(self.background);
        canvas.draw_decoration_primitives(&self.decoration_primitives_before_marks);
        for geometry in &self.mark_geometries {
            canvas.draw_mark_geometry(geometry);
        }
        canvas.draw_decoration_primitives(&self.decoration_primitives_after_marks);
        Ok(canvas.to_frame())
    }
}
