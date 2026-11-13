//! The public composed scene of one density-heatmap render.

use super::super::colour_scale::SVHeatmapColourScale;
use super::super::grid::SVDensityHeatmapGrid;
use super::super::options::SVEmptyBinPolicy;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_decoration::SVDecorationPrimitive;
use crate::sv_error::SVError;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_mark::SVMarkGeometry;
use crate::sv_scale::{SVColour, SVContinuousColourScale, SVScale};

/// Everything one density-heatmap composition produced.
#[derive(Clone)]
pub struct SVDensityHeatmapComposition {
    pub(super) backend_kind: SVBackendKind,
    pub(super) representation: SVRepresentation,
    pub(super) viewport: SVViewport,
    pub(super) plot_area: SVResolvedRectangle,
    pub(super) background: SVColour,
    pub(super) title: Option<String>,
    pub(super) horizontal_scale: SVScale<'static>,
    pub(super) vertical_scale: SVScale<'static>,
    pub(super) colour_scale: SVHeatmapColourScale,
    pub(super) colour_bar: bool,
    pub(super) empty_bin_policy: SVEmptyBinPolicy,
    pub(super) grids: Vec<SVDensityHeatmapGrid>,
    pub(super) series_labels: Vec<String>,
    pub(super) decorations_before_marks: Vec<SVDecorationPrimitive>,
    pub(super) decorations_after_marks: Vec<SVDecorationPrimitive>,
    pub(super) mark_geometries: Vec<SVMarkGeometry<'static>>,
    pub(super) number_of_drawn_marks: usize,
}

impl std::fmt::Debug for SVDensityHeatmapComposition {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVDensityHeatmapComposition")
            .field("backend_kind", &self.backend_kind)
            .field("grids", &self.grids.len())
            .field("number_of_drawn_marks", &self.number_of_drawn_marks)
            .finish_non_exhaustive()
    }
}

impl SVDensityHeatmapComposition {
    /// Returns the backend kind the scene was composed for.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// Returns the dominant representation of the bound columns.
    pub fn representation(&self) -> SVRepresentation {
        self.representation
    }

    /// Returns the viewport the scene was solved for.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the rectangle the cells occupy.
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

    /// Returns the horizontal scale.
    pub fn horizontal_scale(&self) -> &SVScale<'static> {
        &self.horizontal_scale
    }

    /// Returns the vertical scale.
    pub fn vertical_scale(&self) -> &SVScale<'static> {
        &self.vertical_scale
    }

    /// Returns the continuous colour scale the cells use.
    pub fn colour_scale(&self) -> &SVContinuousColourScale {
        &self.colour_scale.scale
    }

    /// Returns the lowest value of the colour domain.
    pub fn colour_scale_lower(&self) -> f64 {
        self.colour_scale.lower
    }

    /// Returns the highest value of the colour domain.
    pub fn colour_scale_upper(&self) -> f64 {
        self.colour_scale.upper
    }

    /// Returns `true` when the colour bar is drawn.
    pub fn colour_bar(&self) -> bool {
        self.colour_bar
    }

    /// Returns the policy applied to empty bins.
    pub fn empty_bin_policy(&self) -> SVEmptyBinPolicy {
        self.empty_bin_policy
    }

    /// Returns one binned grid per series, in first-appearance order.
    pub fn grids(&self) -> &[SVDensityHeatmapGrid] {
        &self.grids
    }

    /// Returns the series labels, empty when the chart is ungrouped.
    pub fn series_labels(&self) -> &[String] {
        &self.series_labels
    }

    /// Returns the decorations drawn below the cells.
    pub fn decoration_primitives_before_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decorations_before_marks
    }

    /// Returns the decorations drawn above the cells.
    pub fn decoration_primitives_after_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decorations_after_marks
    }

    /// Visits every cell geometry in draw order.
    ///
    /// # Errors
    ///
    /// The geometry is prebuilt; the callback never fails, so the result is
    /// always `Ok`. The signature matches the other recipes so a backend can
    /// treat every composition alike.
    pub fn visit_mark_geometries(
        &self,
        mut visitor: impl FnMut(&SVMarkGeometry<'static>),
    ) -> Result<(), SVError> {
        for geometry in &self.mark_geometries {
            visitor(geometry);
        }
        Ok(())
    }

    /// Returns the number of cell rectangles the scene draws.
    pub fn number_of_drawn_marks(&self) -> usize {
        self.number_of_drawn_marks
    }
}
