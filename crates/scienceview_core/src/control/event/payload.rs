//! The payload values carried by an event.

use crate::sv_animation::SVPlaybackState;

use crate::sv_control::{
    SVAxisRange, SVControlAxis, SVControlRange, SVSelectionRegion, SVSeriesReference,
};

/// A visible-range change on one axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVRangeChange {
    /// The axis whose range changed.
    pub axis: SVControlAxis,
    /// The range before the action.
    pub previous: SVControlRange,
    /// The range after the action.
    pub current: SVControlRange,
}

/// A series visibility change.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SVSeriesVisibilityChange {
    /// The series whose visibility changed.
    pub series: SVSeriesReference,
    /// The visibility before the action.
    pub previous: bool,
    /// The visibility after the action.
    pub current: bool,
}

/// A selection change; either side may be absent.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVSelectionChange {
    /// The selection before the action.
    pub previous: Option<SVSelectionRegion>,
    /// The selection after the action.
    pub current: Option<SVSelectionRegion>,
}

/// A playback change on the animation timeline.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVPlaybackChange {
    /// The playback state before the action.
    pub previous_state: SVPlaybackState,
    /// The playback state after the action.
    pub state: SVPlaybackState,
    /// The position before the action, in seconds.
    pub previous_position_seconds: f64,
    /// The position after the action, in seconds.
    pub position_seconds: f64,
    /// The speed before the action.
    pub previous_speed: f64,
    /// The speed after the action.
    pub speed: f64,
}

/// The domains a reset restored.
#[derive(Clone, Debug, PartialEq)]
pub struct SVViewReset {
    /// The visible ranges before the reset, in registration order.
    pub previous: Vec<SVAxisRange>,
    /// The visible ranges after the reset, in registration order.
    pub current: Vec<SVAxisRange>,
}

/// An application-defined payload carried unchanged.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SVCustomPayload {
    /// The name of the custom action.
    pub name: String,
    /// The opaque payload.
    pub payload: String,
}
