//! The backend-agnostic result of one clustered heatmap composition.

mod serialization;

use super::SVClusterAxis;
use crate::sv_backend::SVBackendKind;
use crate::sv_decoration::SVDecorationPrimitive;
use crate::sv_layout::SVViewport;
use crate::sv_mark::SVMarkGeometry;
use crate::sv_scale::{SVColour, SVContinuousColourScale};

/// One drawn matrix cell, in slot space after the supplied order was applied.
#[derive(Clone, Debug, PartialEq)]
pub struct SVClusteredHeatmapCell {
    slot_row: usize,
    slot_column: usize,
    original_row: usize,
    original_column: usize,
    value: Option<f64>,
    colour: Option<SVColour>,
}

impl SVClusteredHeatmapCell {
    pub(super) fn new(
        slot_row: usize,
        slot_column: usize,
        original_row: usize,
        original_column: usize,
        value: Option<f64>,
        colour: Option<SVColour>,
    ) -> Self {
        Self {
            slot_row,
            slot_column,
            original_row,
            original_column,
            value,
            colour,
        }
    }

    /// Returns the row slot after reordering.
    pub fn slot_row(&self) -> usize {
        self.slot_row
    }

    /// Returns the column slot after reordering.
    pub fn slot_column(&self) -> usize {
        self.slot_column
    }

    /// Returns the original matrix row, which survives the reorder.
    pub fn original_row(&self) -> usize {
        self.original_row
    }

    /// Returns the original matrix column, which survives the reorder.
    pub fn original_column(&self) -> usize {
        self.original_column
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

/// One drawn merge of a supplied dendrogram.
///
/// Positions are normalized slot coordinates: `0` is the first leaf slot and
/// `1` the last. The two child distances are the heights the merge connects,
/// so a caller can verify the drawn tree reproduces the supplied distances.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVDendrogramMerge {
    position: usize,
    distance: f64,
    left_distance: f64,
    right_distance: f64,
    left_position: f64,
    right_position: f64,
    position_along_axis: f64,
}

impl SVDendrogramMerge {
    pub(super) fn new(
        position: usize,
        distance: f64,
        left_distance: f64,
        right_distance: f64,
        left_position: f64,
        right_position: f64,
        position_along_axis: f64,
    ) -> Self {
        Self {
            position,
            distance,
            left_distance,
            right_distance,
            left_position,
            right_position,
            position_along_axis,
        }
    }

    /// Returns the merge position in the supplied tree.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Returns the supplied merge distance.
    pub fn distance(&self) -> f64 {
        self.distance
    }

    /// Returns the distance of the left child (a leaf is `0`).
    pub fn left_distance(&self) -> f64 {
        self.left_distance
    }

    /// Returns the distance of the right child (a leaf is `0`).
    pub fn right_distance(&self) -> f64 {
        self.right_distance
    }

    /// Returns the normalized position of the left child along the leaf axis.
    pub fn left_position(&self) -> f64 {
        self.left_position
    }

    /// Returns the normalized position of the right child along the leaf axis.
    pub fn right_position(&self) -> f64 {
        self.right_position
    }

    /// Returns the normalized position the merged node sits at.
    pub fn position_along_axis(&self) -> f64 {
        self.position_along_axis
    }
}

/// One supplied dendrogram, resolved into drawable coordinates.
#[derive(Clone, Debug, PartialEq)]
pub struct SVDendrogram {
    axis: SVClusterAxis,
    maximum_distance: f64,
    merges: Vec<SVDendrogramMerge>,
}

impl SVDendrogram {
    pub(super) fn new(
        axis: SVClusterAxis,
        maximum_distance: f64,
        merges: Vec<SVDendrogramMerge>,
    ) -> Self {
        Self {
            axis,
            maximum_distance,
            merges,
        }
    }

    /// Returns the axis the dendrogram describes.
    pub fn axis(&self) -> SVClusterAxis {
        self.axis
    }

    /// Returns the largest supplied merge distance.
    pub fn maximum_distance(&self) -> f64 {
        self.maximum_distance
    }

    /// Returns the drawn merges in supplied order.
    pub fn merges(&self) -> &[SVDendrogramMerge] {
        &self.merges
    }

    /// Returns the number of drawn merges.
    pub fn number_of_merges(&self) -> usize {
        self.merges.len()
    }
}

/// Everything one clustered heatmap composition produced.
#[derive(Clone, Debug, PartialEq)]
pub struct SVClusteredHeatmapScene {
    pub(super) backend_kind: SVBackendKind,
    pub(super) viewport: SVViewport,
    pub(super) background: SVColour,
    pub(super) title: Option<String>,
    pub(super) description: Option<String>,
    pub(super) decoration_primitives_before_marks: Vec<SVDecorationPrimitive>,
    pub(super) mark_geometries: Vec<SVMarkGeometry<'static>>,
    pub(super) decoration_primitives_after_marks: Vec<SVDecorationPrimitive>,
    pub(super) row_labels: Vec<String>,
    pub(super) column_labels: Vec<String>,
    pub(super) cells: Vec<SVClusteredHeatmapCell>,
    pub(super) colour_scale: SVContinuousColourScale,
    pub(super) row_dendrogram: Option<SVDendrogram>,
    pub(super) column_dendrogram: Option<SVDendrogram>,
    pub(super) number_of_marks: usize,
}

impl SVClusteredHeatmapScene {
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

    /// Returns the scene title.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the accessible description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the decorations drawn below the marks.
    pub fn decoration_primitives_before_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decoration_primitives_before_marks
    }

    /// Returns the mark geometries in draw order.
    pub fn mark_geometries(&self) -> &[SVMarkGeometry<'static>] {
        &self.mark_geometries
    }

    /// Returns the decorations drawn above the marks.
    pub fn decoration_primitives_after_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decoration_primitives_after_marks
    }

    /// Returns the row labels in the supplied order.
    pub fn row_labels(&self) -> &[String] {
        &self.row_labels
    }

    /// Returns the column labels in the supplied order.
    pub fn column_labels(&self) -> &[String] {
        &self.column_labels
    }

    /// Returns every drawn cell, in slot-major order.
    pub fn cells(&self) -> &[SVClusteredHeatmapCell] {
        &self.cells
    }

    /// Returns the number of drawn cells.
    pub fn number_of_marks(&self) -> usize {
        self.number_of_marks
    }

    /// Returns the number of primitives the composition produced, including the
    /// dendrogram paths.
    pub fn number_of_mark_primitives(&self) -> usize {
        self.mark_geometries.iter().map(SVMarkGeometry::len).sum()
    }

    /// Returns the continuous colour scale the cells and the colour bar share.
    pub fn colour_scale(&self) -> &SVContinuousColourScale {
        &self.colour_scale
    }

    /// Returns the supplied row dendrogram, when it was drawn.
    pub fn row_dendrogram(&self) -> Option<&SVDendrogram> {
        self.row_dendrogram.as_ref()
    }

    /// Returns the supplied column dendrogram, when it was drawn.
    pub fn column_dendrogram(&self) -> Option<&SVDendrogram> {
        self.column_dendrogram.as_ref()
    }
}
