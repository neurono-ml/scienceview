//! The backend-agnostic scene one area figure carries.

use scienceview_core::sv_decoration::SVDecorationPrimitive;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::{SVBaselineMode, SVMarkPrimitive};
use scienceview_core::sv_scale::SVColour;

use super::super::configuration::SVAreaInterpolation;
use super::super::missing_value_policy::SVAreaMissingValuePolicy;

/// The backend-agnostic scene of one area figure.
///
/// This is the recipe-level scene contract: the value carries the viewport, the
/// background, and every primitive in final draw order — `grid_and_frame`
/// first, the mark `primitives` next and `axes_and_title` last. A backend
/// serializer replays exactly those lists; it never re-derives a position, and
/// it can never substitute the output of another backend. `SVAreaPlot::render`
/// selects the serializer from the configured kind: the SVG document for
/// `Svg`, the raster frame for `Native`, and an explicit missing-capability
/// error for every other kind.
#[derive(Clone, Debug)]
pub struct SVAreaRendering {
    pub(super) viewport: SVViewport,
    pub(super) title: Option<String>,
    pub(super) background: SVColour,
    pub(super) grid_and_frame: Vec<SVDecorationPrimitive>,
    pub(super) primitives: Vec<SVMarkPrimitive<'static>>,
    pub(super) axes_and_title: Vec<SVDecorationPrimitive>,
    pub(super) representation: scienceview_core::sv_data::SVRepresentation,
    pub(super) point_count: usize,
    pub(super) stack: bool,
    pub(super) baseline: SVBaselineMode,
    pub(super) opacity: Option<f32>,
    pub(super) line: bool,
    pub(super) interpolation: SVAreaInterpolation,
    pub(super) missing_value_policy: SVAreaMissingValuePolicy,
}

impl SVAreaRendering {
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

    /// Returns the grid and frame primitives, drawn before the areas.
    pub fn grid_and_frame(&self) -> &[SVDecorationPrimitive] {
        &self.grid_and_frame
    }

    /// Returns the area and boundary primitives, in draw order.
    pub fn primitives(&self) -> &[SVMarkPrimitive<'static>] {
        &self.primitives
    }

    /// Returns the axis and title primitives, drawn after the areas.
    pub fn axes_and_title(&self) -> &[SVDecorationPrimitive] {
        &self.axes_and_title
    }

    /// Returns the representation of the value column.
    pub fn representation(&self) -> scienceview_core::sv_data::SVRepresentation {
        self.representation
    }

    /// Returns the number of drawn points.
    pub fn point_count(&self) -> usize {
        self.point_count
    }

    /// Returns whether the areas are stacked.
    pub fn stack_enabled(&self) -> bool {
        self.stack
    }

    /// Returns the configured baseline of a non-stacked area.
    pub fn baseline_value(&self) -> SVBaselineMode {
        self.baseline
    }

    /// Returns the explicit opacity, or `None` when the token applies.
    pub fn opacity_value(&self) -> Option<f32> {
        self.opacity
    }

    /// Returns whether the boundary line is drawn.
    pub fn line_enabled(&self) -> bool {
        self.line
    }

    /// Returns the interpolation of the ordered boundary.
    pub fn interpolation_value(&self) -> SVAreaInterpolation {
        self.interpolation
    }

    /// Returns the missing-value policy in force.
    pub fn missing_value_policy_value(&self) -> SVAreaMissingValuePolicy {
        self.missing_value_policy
    }
}
