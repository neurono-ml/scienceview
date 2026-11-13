//! The composed ECDF plot and its read-only surface.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVRepresentation;
use scienceview_core::sv_decoration::SVDecorationPrimitive;
use scienceview_core::sv_layout::{SVLength, SVResolvedRectangle, SVViewport};
use scienceview_core::sv_mark::SVBarOrientation;
use scienceview_core::sv_scale::{SVColour, SVScale, SVScaleKind};

use super::series::SVEcdfSeries;

/// A composed ECDF plot, ready for any backend that serializes mark geometry.
///
/// The composition is backend-neutral on purpose: it owns the scales, the
/// expanded step vertices and the decoration primitives, so the SVG backend
/// (and the native rasterizer) serializes exactly the same picture without
/// recomputing the chart.
#[derive(Clone, Debug, PartialEq)]
pub struct SVEcdfPlotComposition {
    pub(crate) backend_kind: SVBackendKind,
    pub(crate) representation: SVRepresentation,
    pub(crate) point_count: usize,
    pub(crate) viewport: SVViewport,
    pub(crate) plot_area: SVResolvedRectangle,
    pub(crate) horizontal_scale: SVScale<'static>,
    pub(crate) vertical_scale: SVScale<'static>,
    pub(crate) series: Vec<SVEcdfSeries>,
    pub(crate) line_width: Option<SVLength>,
    pub(crate) orientation: SVBarOrientation,
    pub(crate) decorations_before_marks: Vec<SVDecorationPrimitive>,
    pub(crate) decorations_after_marks: Vec<SVDecorationPrimitive>,
    pub(crate) title: Option<String>,
    pub(crate) background: SVColour,
}

impl SVEcdfPlotComposition {
    /// Returns the configured backend kind.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// Returns the curves, in first-appearance series order.
    pub fn series(&self) -> &[SVEcdfSeries] {
        &self.series
    }

    /// Returns the number of curves.
    pub fn number_of_series(&self) -> usize {
        self.series.len()
    }

    /// Returns the representation the data arrived in.
    pub fn representation(&self) -> SVRepresentation {
        self.representation
    }

    /// Returns the number of observations the curves draw.
    pub fn point_count(&self) -> usize {
        self.point_count
    }

    /// Returns the orientation the two channels use.
    pub fn orientation(&self) -> SVBarOrientation {
        self.orientation
    }

    /// Returns the logical canvas size.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the solved plot rectangle.
    pub fn plot_area(&self) -> SVResolvedRectangle {
        self.plot_area
    }

    /// Returns the kind of the horizontal projection.
    pub fn horizontal_scale_kind(&self) -> SVScaleKind {
        self.horizontal_scale.kind()
    }

    /// Returns the kind of the vertical projection.
    pub fn vertical_scale_kind(&self) -> SVScaleKind {
        self.vertical_scale.kind()
    }

    /// Returns the title the document announces.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the fixed background colour.
    pub fn background(&self) -> SVColour {
        self.background
    }

    /// Returns the decorations drawn beneath the marks (title and grid).
    pub fn decorations_before_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decorations_before_marks
    }

    /// Returns the decorations drawn above the marks (axes and legend).
    pub fn decorations_after_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decorations_after_marks
    }
}
