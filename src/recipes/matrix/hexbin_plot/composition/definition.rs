//! One hexbin render: the public cells and the backend-agnostic scene.

use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_decoration::SVDecorationPrimitive;
use crate::sv_error::SVError;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_mark::SVMarkGeometry;
use crate::sv_scale::{SVColour, SVContinuousColourScale, SVScale};

/// One occupied hexagonal cell.
#[derive(Clone, Debug, PartialEq)]
pub struct SVHexbinCell {
    pub(super) column: i64,
    pub(super) row: i64,
    pub(super) series: Option<f64>,
    pub(super) centre_x: f64,
    pub(super) centre_y: f64,
    pub(super) value: f64,
    pub(super) observations: usize,
    pub(super) colour: SVColour,
}

impl SVHexbinCell {
    /// Returns the axial column index of the cell.
    pub fn column(&self) -> i64 {
        self.column
    }

    /// Returns the axial row index of the cell.
    pub fn row(&self) -> i64 {
        self.row
    }

    /// Returns the series code, when the chart is grouped.
    pub fn series(&self) -> Option<f64> {
        self.series
    }

    /// Returns the cell centre on the horizontal data axis.
    pub fn centre_x(&self) -> f64 {
        self.centre_x
    }

    /// Returns the cell centre on the vertical data axis.
    pub fn centre_y(&self) -> f64 {
        self.centre_y
    }

    /// Returns the value the colour encodes: the count or the mean.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Returns the number of observations the cell accumulated.
    pub fn observations(&self) -> usize {
        self.observations
    }

    /// Returns the colour the cell is painted with.
    pub fn colour(&self) -> SVColour {
        self.colour
    }
}

/// Everything one hexbin composition produced.
#[derive(Clone)]
pub struct SVHexbinComposition {
    pub(super) backend_kind: SVBackendKind,
    pub(super) representation: SVRepresentation,
    pub(super) viewport: SVViewport,
    pub(super) plot_area: SVResolvedRectangle,
    pub(super) background: SVColour,
    pub(super) title: Option<String>,
    pub(super) horizontal_scale: SVScale<'static>,
    pub(super) vertical_scale: SVScale<'static>,
    pub(super) colour_scale: SVContinuousColourScale,
    pub(super) cells: Vec<SVHexbinCell>,
    pub(super) number_of_observations: usize,
    pub(super) number_of_series: usize,
    pub(super) decorations_before_marks: Vec<SVDecorationPrimitive>,
    pub(super) decorations_after_marks: Vec<SVDecorationPrimitive>,
    pub(super) mark_geometries: Vec<SVMarkGeometry<'static>>,
    pub(super) number_of_drawn_marks: usize,
}

impl std::fmt::Debug for SVHexbinComposition {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVHexbinComposition")
            .field("backend_kind", &self.backend_kind)
            .field("cells", &self.cells.len())
            .field("observations", &self.number_of_observations)
            .finish_non_exhaustive()
    }
}

impl SVHexbinComposition {
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

    /// Returns the rectangle the hexagons occupy.
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

    /// Returns the continuous colour scale the counts are painted through.
    pub fn colour_scale(&self) -> &SVContinuousColourScale {
        &self.colour_scale
    }

    /// Returns the occupied cells in draw order.
    pub fn cells(&self) -> &[SVHexbinCell] {
        &self.cells
    }

    /// Returns the number of occupied cells after the minimum-count filter.
    pub fn number_of_cells(&self) -> usize {
        self.cells.len()
    }

    /// Returns the number of finite observations the lattice consumed.
    pub fn number_of_observations(&self) -> usize {
        self.number_of_observations
    }

    /// Returns the number of distinct series, `1` when the chart is ungrouped.
    pub fn number_of_series(&self) -> usize {
        self.number_of_series
    }

    /// Returns the decorations drawn below the hexagons.
    pub fn decoration_primitives_before_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decorations_before_marks
    }

    /// Returns the decorations drawn above the hexagons.
    pub fn decoration_primitives_after_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decorations_after_marks
    }

    /// Returns the mark geometries in draw order.
    pub fn mark_geometries(&self) -> &[SVMarkGeometry<'static>] {
        &self.mark_geometries
    }

    /// Visits every mark geometry in draw order.
    ///
    /// # Errors
    ///
    /// The geometry is prebuilt, so the callback never fails and the result is
    /// always `Ok`; the signature matches the other recipes.
    pub fn visit_mark_geometries(
        &self,
        mut visitor: impl FnMut(&SVMarkGeometry<'static>),
    ) -> Result<(), SVError> {
        for geometry in &self.mark_geometries {
            visitor(geometry);
        }
        Ok(())
    }

    /// Returns the number of hexagon paths the scene draws.
    pub fn number_of_drawn_marks(&self) -> usize {
        self.number_of_drawn_marks
    }

    /// Serializes the composition as a self-contained SVG document.
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
            .with_title(self.title.as_deref().unwrap_or("hexbin plot"))
            .with_description("Hexbin plot generated by scienceview")
            .with_background(self.background);
        document.push_decoration_primitives(&self.decorations_before_marks);
        for geometry in &self.mark_geometries {
            document.push_mark_geometry(geometry);
        }
        document.push_decoration_primitives(&self.decorations_after_marks);
        Ok(document.to_svg())
    }

    /// Rasterizes the composition with the native renderer.
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
        canvas.draw_decoration_primitives(&self.decorations_before_marks);
        for geometry in &self.mark_geometries {
            canvas.draw_mark_geometry(geometry);
        }
        canvas.draw_decoration_primitives(&self.decorations_after_marks);
        Ok(canvas.to_frame())
    }
}
