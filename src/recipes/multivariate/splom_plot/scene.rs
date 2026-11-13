//! The backend-agnostic result of one scatterplot-matrix render.
//!
//! The scene owns no notation: it carries the solved viewport, the cell
//! rectangles, the decoration primitives before and after the marks, and one
//! mark geometry per cell. An SVG, native or component backend serializes the
//! same scene without re-deriving a position.

use crate::sv_backend::SVBackendKind;
use crate::sv_decoration::SVDecorationPrimitive;
#[cfg(any(feature = "render-svg", feature = "render-native"))]
use crate::sv_error::SVError;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_mark::SVMarkGeometry;
use crate::sv_scale::SVColour;

/// What one cell of the matrix draws.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVSplomCellKind {
    /// An off-diagonal scatter cell for one variable pair.
    Scatter,
    /// A diagonal cell for the univariate distribution of one variable.
    Diagonal,
}

/// One cell of the solved matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVSplomCell {
    row: usize,
    column: usize,
    rectangle: SVResolvedRectangle,
    kind: SVSplomCellKind,
}

impl SVSplomCell {
    pub(super) fn new(
        row: usize,
        column: usize,
        rectangle: SVResolvedRectangle,
        kind: SVSplomCellKind,
    ) -> Self {
        Self {
            row,
            column,
            rectangle,
            kind,
        }
    }

    /// Returns the row index, the variable on the vertical axis.
    pub fn row(&self) -> usize {
        self.row
    }

    /// Returns the column index, the variable on the horizontal axis.
    pub fn column(&self) -> usize {
        self.column
    }

    /// Returns the solved rectangle of the cell.
    pub fn rectangle(&self) -> SVResolvedRectangle {
        self.rectangle
    }

    /// Returns what the cell draws.
    pub fn kind(&self) -> SVSplomCellKind {
        self.kind
    }
}

/// Everything one scatterplot-matrix composition produced.
///
/// The scene is backend-agnostic, but it carries the kind it was composed for:
/// the recipe-level serializers exposed on it (`to_svg`, `to_native_frame`)
/// refuse a scene whose configured kind they do not serve.
#[derive(Clone, Debug, PartialEq)]
pub struct SVSplomScene {
    pub(super) backend_kind: SVBackendKind,
    pub(super) viewport: SVViewport,
    pub(super) background: SVColour,
    pub(super) title: Option<String>,
    pub(super) description: Option<String>,
    pub(super) cells: Vec<SVSplomCell>,
    pub(super) decoration_primitives_before_marks: Vec<SVDecorationPrimitive>,
    pub(super) mark_geometries: Vec<SVMarkGeometry<'static>>,
    pub(super) decoration_primitives_after_marks: Vec<SVDecorationPrimitive>,
    pub(super) number_of_variables: usize,
    pub(super) number_of_observations: usize,
    pub(super) number_of_marks: usize,
    pub(super) legend_labels: Vec<String>,
    pub(super) legend_colours: Vec<SVColour>,
}

impl SVSplomScene {
    /// Returns the backend kind the scene was composed for.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// Returns the viewport the scene was solved for.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
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

    /// Returns the solved cells in row-major order.
    pub fn cells(&self) -> &[SVSplomCell] {
        &self.cells
    }

    /// Returns the decorations drawn below the marks.
    pub fn decoration_primitives_before_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decoration_primitives_before_marks
    }

    /// Returns one mark geometry per cell, in the same order as [`Self::cells`].
    pub fn mark_geometries(&self) -> &[SVMarkGeometry<'static>] {
        &self.mark_geometries
    }

    /// Returns the decorations drawn above the marks.
    pub fn decoration_primitives_after_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decoration_primitives_after_marks
    }

    /// Returns the number of variables, the width and height of the grid.
    pub fn number_of_variables(&self) -> usize {
        self.number_of_variables
    }

    /// Returns the number of observations every scatter cell draws.
    pub fn number_of_observations(&self) -> usize {
        self.number_of_observations
    }

    /// Returns the number of logical marks the off-diagonal cells draw.
    pub fn number_of_marks(&self) -> usize {
        self.number_of_marks
    }

    /// Returns the number of primitives every cell produced together.
    pub fn number_of_mark_primitives(&self) -> usize {
        self.mark_geometries.iter().map(SVMarkGeometry::len).sum()
    }

    /// Returns the number of cells currently drawn.
    pub fn number_of_cells(&self) -> usize {
        self.cells.len()
    }

    /// Returns the number of diagonal cells currently drawn.
    pub fn number_of_diagonal_cells(&self) -> usize {
        self.cells
            .iter()
            .filter(|cell| cell.kind == SVSplomCellKind::Diagonal)
            .count()
    }

    /// Returns the legend entries in assignment order.
    pub fn legend_labels(&self) -> &[String] {
        &self.legend_labels
    }

    /// Returns the colour of every legend entry.
    pub fn legend_colours(&self) -> &[SVColour] {
        &self.legend_colours
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
                    .unwrap_or_else(|| "scatterplot matrix".to_string()),
            )
            .with_description(
                self.description
                    .clone()
                    .unwrap_or_else(|| "scatterplot matrix".to_string()),
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
