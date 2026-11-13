//! The backend-agnostic scene one density figure carries.

use scienceview_core::sv_data::SVRepresentation;
use scienceview_core::sv_decoration::SVDecorationPrimitive;
use scienceview_core::sv_layout::{SVLength, SVViewport};
use scienceview_core::sv_mark::SVMarkPrimitive;
use scienceview_core::sv_scale::SVColour;
use scienceview_core::sv_stat::SVNormalisation;

/// One density estimate as the scene carries it: the grid the kernel
/// evaluated and the density at each sample.
#[derive(Clone, Debug, PartialEq)]
pub struct SVDensityCurve {
    pub(super) grid: Vec<f64>,
    pub(super) density: Vec<f64>,
    pub(super) bandwidth: f64,
}

impl SVDensityCurve {
    /// Returns the evaluation grid, in ascending order.
    pub fn grid(&self) -> &[f64] {
        &self.grid
    }

    /// Returns one density estimate per grid sample.
    pub fn density(&self) -> &[f64] {
        &self.density
    }

    /// Returns the bandwidth the estimate used.
    pub fn bandwidth(&self) -> f64 {
        self.bandwidth
    }
}

/// The backend-agnostic scene of one density figure.
///
/// This is the recipe-level scene contract: the value carries the viewport, the
/// background, and every primitive in final draw order — `grid_and_frame`
/// first, the mark `primitives` next and `axes_and_title` last. A backend
/// serializer replays exactly those lists; it never re-derives a position, and
/// it can never substitute the output of another backend.
#[derive(Clone, Debug)]
pub struct SVDensityRendering {
    pub(super) viewport: SVViewport,
    pub(super) title: Option<String>,
    pub(super) background: SVColour,
    pub(super) grid_and_frame: Vec<SVDecorationPrimitive>,
    pub(super) primitives: Vec<SVMarkPrimitive<'static>>,
    pub(super) axes_and_title: Vec<SVDecorationPrimitive>,
    pub(super) representation: SVRepresentation,
    pub(super) point_count: usize,
    pub(super) windowed: bool,
    pub(super) series_count: usize,
    pub(super) curves: Vec<SVDensityCurve>,
    pub(super) fill: bool,
    pub(super) line_width: Option<SVLength>,
    pub(super) normalisation: SVNormalisation,
}

impl SVDensityRendering {
    /// Returns the logical size of the figure.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the figure title, when one was set.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the background colour.
    pub fn background(&self) -> SVColour {
        self.background
    }

    /// Returns the grid and frame primitives, drawn before the curves.
    pub fn grid_and_frame(&self) -> &[SVDecorationPrimitive] {
        &self.grid_and_frame
    }

    /// Returns the area and line primitives, in draw order.
    pub fn primitives(&self) -> &[SVMarkPrimitive<'static>] {
        &self.primitives
    }

    /// Returns the axis and title primitives, drawn after the curves.
    pub fn axes_and_title(&self) -> &[SVDecorationPrimitive] {
        &self.axes_and_title
    }

    /// Returns the representation of the value column.
    pub fn representation(&self) -> SVRepresentation {
        self.representation
    }

    /// Returns the number of grid samples drawn.
    pub fn point_count(&self) -> usize {
        self.point_count
    }

    /// Returns `true` when the estimate describes only the current window.
    pub fn is_windowed(&self) -> bool {
        self.windowed
    }

    /// Returns the number of density curves drawn.
    pub fn series_count(&self) -> usize {
        self.series_count
    }

    /// Returns the density estimate each curve drew, in series order.
    pub fn curves(&self) -> &[SVDensityCurve] {
        &self.curves
    }

    /// Returns whether the curve is filled to the baseline.
    pub fn fill_enabled(&self) -> bool {
        self.fill
    }

    /// Returns the explicit line width, or `None` when the token applies.
    pub fn line_width_value(&self) -> Option<SVLength> {
        self.line_width
    }

    /// Returns the normalisation applied to the density.
    pub fn normalisation_value(&self) -> SVNormalisation {
        self.normalisation
    }
}
