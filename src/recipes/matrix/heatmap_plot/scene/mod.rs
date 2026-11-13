//! The backend-agnostic result of one heatmap composition.

mod serialization;

use crate::sv_backend::SVBackendKind;
use crate::sv_decoration::SVDecorationPrimitive;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_mark::SVMarkGeometry;
use crate::sv_scale::{SVColour, SVContinuousColourScale};

/// One drawn matrix cell, in data space.
#[derive(Clone, Debug, PartialEq)]
pub struct SVHeatmapCell {
    row: usize,
    column: usize,
    value: Option<f64>,
    colour: Option<SVColour>,
}

impl SVHeatmapCell {
    pub(super) fn new(
        row: usize,
        column: usize,
        value: Option<f64>,
        colour: Option<SVColour>,
    ) -> Self {
        Self {
            row,
            column,
            value,
            colour,
        }
    }

    /// Returns the row index.
    pub fn row(&self) -> usize {
        self.row
    }

    /// Returns the column index.
    pub fn column(&self) -> usize {
        self.column
    }

    /// Returns the cell value, or `None` when the cell is missing.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Returns the resolved fill colour, or `None` when the cell has no fill.
    pub fn colour(&self) -> Option<SVColour> {
        self.colour
    }
}

/// Everything one heatmap composition produced.
#[derive(Clone, Debug, PartialEq)]
pub struct SVHeatmapScene {
    pub(super) backend_kind: SVBackendKind,
    pub(super) viewport: SVViewport,
    pub(super) plot_area: SVResolvedRectangle,
    pub(super) background: SVColour,
    pub(super) title: Option<String>,
    pub(super) description: Option<String>,
    pub(super) decoration_primitives_before_marks: Vec<SVDecorationPrimitive>,
    pub(super) mark_geometries: Vec<SVMarkGeometry<'static>>,
    pub(super) decoration_primitives_after_marks: Vec<SVDecorationPrimitive>,
    pub(super) row_labels: Vec<String>,
    pub(super) column_labels: Vec<String>,
    pub(super) cells: Vec<SVHeatmapCell>,
    pub(super) colour_scale: SVContinuousColourScale,
    pub(super) number_of_marks: usize,
}

impl SVHeatmapScene {
    /// Returns the backend kind the scene was composed for.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
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

    /// Returns the scene title.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the accessible description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the decorations drawn below the cells.
    pub fn decoration_primitives_before_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decoration_primitives_before_marks
    }

    /// Returns the mark geometries in draw order.
    pub fn mark_geometries(&self) -> &[SVMarkGeometry<'static>] {
        &self.mark_geometries
    }

    /// Returns the decorations drawn above the cells: axes, annotations and
    /// the colour bar.
    pub fn decoration_primitives_after_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decoration_primitives_after_marks
    }

    /// Returns the row labels in slot order.
    pub fn row_labels(&self) -> &[String] {
        &self.row_labels
    }

    /// Returns the column labels in slot order.
    pub fn column_labels(&self) -> &[String] {
        &self.column_labels
    }

    /// Returns every drawn cell, in row-major order.
    pub fn cells(&self) -> &[SVHeatmapCell] {
        &self.cells
    }

    /// Returns the number of drawn cells, implicit sparse zeros excluded.
    pub fn number_of_marks(&self) -> usize {
        self.number_of_marks
    }

    /// Returns the number of primitives the cell kernel produced.
    pub fn number_of_mark_primitives(&self) -> usize {
        self.mark_geometries.iter().map(SVMarkGeometry::len).sum()
    }

    /// Returns the continuous colour scale the cells and the colour bar share.
    pub fn colour_scale(&self) -> &SVContinuousColourScale {
        &self.colour_scale
    }
}
