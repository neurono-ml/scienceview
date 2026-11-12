//! The backend-agnostic result of one scatter render.
//!
//! The scene owns no notation: it carries the solved viewport and plot
//! rectangle, the decoration primitives before and after the marks, and the
//! mark geometries. An SVG, native or component backend serializes the same
//! scene without re-deriving a position.

use super::SVScatterLegendPlacement;
use crate::sv_decoration::SVDecorationPrimitive;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_mark::SVMarkGeometry;
use crate::sv_scale::SVColour;

/// Everything one scatter render produced.
#[derive(Clone, Debug, PartialEq)]
pub struct SVScatterScene {
    pub(super) viewport: SVViewport,
    pub(super) plot_area: SVResolvedRectangle,
    pub(super) background: SVColour,
    pub(super) title: Option<String>,
    pub(super) description: Option<String>,
    pub(super) decoration_primitives_before_marks: Vec<SVDecorationPrimitive>,
    pub(super) mark_geometries: Vec<SVMarkGeometry<'static>>,
    pub(super) decoration_primitives_after_marks: Vec<SVDecorationPrimitive>,
    pub(super) legend_placement: SVScatterLegendPlacement,
    pub(super) legend_labels: Vec<String>,
    pub(super) number_of_series: usize,
    pub(super) number_of_marks: usize,
}

impl SVScatterScene {
    /// Returns the viewport the scene was solved for.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the rectangle the marks occupy.
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

    /// Returns the decorations drawn below the marks: title, grid and border.
    pub fn decoration_primitives_before_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decoration_primitives_before_marks
    }

    /// Returns the mark geometries in draw order.
    pub fn mark_geometries(&self) -> &[SVMarkGeometry<'static>] {
        &self.mark_geometries
    }

    /// Returns the decorations drawn above the marks: axes and legend.
    pub fn decoration_primitives_after_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decoration_primitives_after_marks
    }

    /// Returns the number of logical marks the scene draws.
    pub fn number_of_marks(&self) -> usize {
        self.number_of_marks
    }

    /// Returns the number of primitives the mark kernel produced.
    ///
    /// A large scatter groups many circles into one compound path, so this is
    /// smaller than [`Self::number_of_marks`] once the overplotting policy
    /// engages.
    pub fn number_of_mark_primitives(&self) -> usize {
        self.mark_geometries.iter().map(SVMarkGeometry::len).sum()
    }

    /// Returns the number of series the marks are grouped into.
    pub fn number_of_series(&self) -> usize {
        self.number_of_series
    }

    /// Returns the legend entries in assignment order.
    pub fn legend_labels(&self) -> &[String] {
        &self.legend_labels
    }

    /// Returns the placement the builder requested.
    pub fn legend_placement(&self) -> SVScatterLegendPlacement {
        self.legend_placement
    }
}
