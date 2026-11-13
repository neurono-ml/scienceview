//! The semantic action vocabulary every control and every widget dispatches.

use core::fmt;

use crate::sv_animation::SVAnimationAction;

use super::{SVControlAxis, SVControlRange, SVSelectionRegion, SVSeriesReference};

/// One semantic change a control requests.
///
/// A control never mutates a figure: it produces one of these values and the
/// [`SVControlState`](super::SVControlState) reducer applies it. A third-party
/// widget produces the same values, which is the whole integration contract.
///
/// Every payload is data. Zoom carries a multiplicative factor applied to the
/// visible span (`< 1` zooms in, `> 1` zooms out) and an anchor fraction in
/// `0..=1` (`0.5` keeps the centre fixed). Pan carries a fraction of the visible
/// span added to both bounds.
#[derive(Clone, Debug, PartialEq)]
pub enum SVControlAction {
    /// Shrink or grow the visible span of one axis around an anchor.
    Zoom {
        /// The axis to zoom.
        axis: SVControlAxis,
        /// The multiplier applied to the visible span.
        factor: f64,
        /// The fraction of the visible span kept fixed, in `0..=1`.
        anchor: f64,
    },
    /// Shift the visible span of one axis by a fraction of itself.
    Pan {
        /// The axis to pan.
        axis: SVControlAxis,
        /// The signed fraction of the visible span to add to both bounds.
        fraction: f64,
    },
    /// Restore every visible domain to the registered bounds.
    Reset,
    /// Flip the visibility of one series.
    ToggleSeries(SVSeriesReference),
    /// Set the visibility of one series explicitly.
    SetSeriesVisibility {
        /// The series to show or hide.
        series: SVSeriesReference,
        /// Whether the series is visible.
        visible: bool,
    },
    /// Set the visible interval of one axis.
    SetRange {
        /// The axis whose range changes.
        axis: SVControlAxis,
        /// The interval to show.
        range: SVControlRange,
    },
    /// Select a rectangular region of the data.
    SelectRegion(SVSelectionRegion),
    /// Clear the current selection.
    ClearSelection,
    /// Apply one playback command to the animation timeline.
    SetPlayback(SVAnimationAction),
    /// Carry an application-defined action and payload unchanged.
    Custom {
        /// The name of the action.
        name: String,
        /// The opaque payload, preserved byte for byte.
        payload: String,
    },
}

impl SVControlAction {
    /// Returns the action family name used in event payloads.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Zoom { .. } => "zoom",
            Self::Pan { .. } => "pan",
            Self::Reset => "reset",
            Self::ToggleSeries(_) | Self::SetSeriesVisibility { .. } => "series",
            Self::SetRange { .. } => "range",
            Self::SelectRegion(_) | Self::ClearSelection => "selection",
            Self::SetPlayback(_) => "playback",
            Self::Custom { .. } => "custom",
        }
    }
}

impl fmt::Display for SVControlAction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
