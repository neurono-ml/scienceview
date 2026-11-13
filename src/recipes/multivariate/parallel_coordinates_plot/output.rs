//! The backend-agnostic result of one parallel-coordinates composition.

use super::model::{SVParallelCoordinatesAxis, SVParallelCoordinatesLine};
use super::options::{SVParallelCoordinatesMissingPolicy, SVParallelCoordinatesNormalization};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_decoration::SVDecorationPrimitive;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_mark::SVMarkGeometry;
use crate::sv_scale::SVColour;

/// Everything one parallel-coordinates composition produced.
#[derive(Clone, Debug, PartialEq)]
pub struct SVParallelCoordinatesRenderOutput {
    backend_kind: SVBackendKind,
    representation: SVRepresentation,
    number_of_observations: usize,
    viewport: SVViewport,
    plot_area: SVResolvedRectangle,
    title: Option<String>,
    axes: Vec<SVParallelCoordinatesAxis>,
    lines: Vec<SVParallelCoordinatesLine>,
    colour_groups: Vec<(String, SVColour)>,
    geometries: Vec<SVMarkGeometry<'static>>,
    decorations: Vec<SVDecorationPrimitive>,
    background: SVColour,
    normalization: SVParallelCoordinatesNormalization,
    missing: SVParallelCoordinatesMissingPolicy,
}

impl SVParallelCoordinatesRenderOutput {
    /// Assembles one composition result.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        backend_kind: SVBackendKind,
        representation: SVRepresentation,
        number_of_observations: usize,
        viewport: SVViewport,
        plot_area: SVResolvedRectangle,
        title: Option<String>,
        axes: Vec<SVParallelCoordinatesAxis>,
        lines: Vec<SVParallelCoordinatesLine>,
        colour_groups: Vec<(String, SVColour)>,
        geometries: Vec<SVMarkGeometry<'static>>,
        decorations: Vec<SVDecorationPrimitive>,
        background: SVColour,
        normalization: SVParallelCoordinatesNormalization,
        missing: SVParallelCoordinatesMissingPolicy,
    ) -> Self {
        Self {
            backend_kind,
            representation,
            number_of_observations,
            viewport,
            plot_area,
            title,
            axes,
            lines,
            colour_groups,
            geometries,
            decorations,
            background,
            normalization,
            missing,
        }
    }

    /// Returns the backend kind the figure was configured for.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// Returns the representation the operation resolved.
    pub fn representation(&self) -> SVRepresentation {
        self.representation
    }

    /// Returns the number of logical observations.
    pub fn number_of_observations(&self) -> usize {
        self.number_of_observations
    }

    /// Returns the number of drawn polylines, one per observation unless a gap
    /// splits it.
    pub fn number_of_lines(&self) -> usize {
        self.lines.len()
    }

    /// Returns the logical viewport.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the solved plot rectangle.
    pub fn plot_area(&self) -> SVResolvedRectangle {
        self.plot_area
    }

    /// Returns the figure title.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the visible axes in draw order.
    pub fn axes(&self) -> &[SVParallelCoordinatesAxis] {
        &self.axes
    }

    /// Returns the drawn polylines in observation order.
    pub fn lines(&self) -> &[SVParallelCoordinatesLine] {
        &self.lines
    }

    /// Returns the colour groups in first-appearance order.
    pub fn colour_groups(&self) -> &[(String, SVColour)] {
        &self.colour_groups
    }

    /// Returns the polyline geometry in draw order.
    pub fn geometries(&self) -> &[SVMarkGeometry<'static>] {
        &self.geometries
    }

    /// Returns the axes, labels and title in draw order.
    pub fn decorations(&self) -> &[SVDecorationPrimitive] {
        &self.decorations
    }

    /// Returns the background the scene was composed against.
    pub fn background(&self) -> SVColour {
        self.background
    }

    /// Returns the normalisation rule the axes used.
    pub fn normalization(&self) -> SVParallelCoordinatesNormalization {
        self.normalization
    }

    /// Returns the missing-value policy the lines used.
    pub fn missing_policy(&self) -> SVParallelCoordinatesMissingPolicy {
        self.missing
    }
}
