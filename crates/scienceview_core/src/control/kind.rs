//! The closed set of controls the kit provides.

use crate::sv_animation::{SVAnimationAction, SVDuration};

use super::{SVControlAxis, SVSeriesReference};

/// One playback command a playback control carries.
///
/// The kit does not own a second playback vocabulary: a control resolves to the
/// animation layer's own action, so the timeline and the controls agree on what
/// play, pause, step, seek and speed mean.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVPlaybackCommand {
    /// Start or resume playback.
    Play,
    /// Hold the current position.
    Pause,
    /// Pause a playing timeline and play a paused one.
    Toggle,
    /// Return to the start and become idle.
    Stop,
    /// Pause and move to the neighbouring keyframe.
    Step {
        /// Whether to step towards the end of the timeline.
        forward: bool,
    },
    /// Move the clock to an explicit position.
    Seek(SVDuration),
    /// Change the playback multiplier.
    Speed(f64),
}

impl SVPlaybackCommand {
    /// Returns the animation action this command dispatches.
    pub fn action(self) -> SVAnimationAction {
        match self {
            Self::Play => SVAnimationAction::Play,
            Self::Pause => SVAnimationAction::Pause,
            Self::Toggle => SVAnimationAction::Toggle,
            Self::Stop => SVAnimationAction::Stop,
            Self::Step { forward } => SVAnimationAction::Step { forward },
            Self::Seek(position) => SVAnimationAction::Seek(position),
            Self::Speed(speed) => SVAnimationAction::SetSpeed(speed),
        }
    }

    /// Returns the command name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Play => "play",
            Self::Pause => "pause",
            Self::Toggle => "toggle",
            Self::Stop => "stop",
            Self::Step { .. } => "step",
            Self::Seek(_) => "seek",
            Self::Speed(_) => "speed",
        }
    }
}

/// The kind of a control, a closed enumeration.
///
/// The kind decides the action a control derives; the control's own options
/// (target axis, step, bounds, explicit action) refine it. Adding a kind is a
/// change to the control boundary, exactly like adding a mark kind.
#[derive(Clone, Debug, PartialEq)]
pub enum SVControlKind {
    /// Zoom the target axis in by the configured step.
    ZoomIn,
    /// Zoom the target axis out by the configured step.
    ZoomOut,
    /// Restore every visible domain.
    Reset,
    /// Pan the target axis by the configured step.
    Pan,
    /// Toggle the visibility of one series.
    SeriesToggle(SVSeriesReference),
    /// A slider that sets the visible interval of one axis.
    RangeSlider {
        /// The axis the slider drives.
        axis: SVControlAxis,
    },
    /// A control that selects a rectangular region of the data.
    RegionSelect,
    /// A playback control bound to the animation timeline.
    Playback(SVPlaybackCommand),
}

impl SVControlKind {
    /// Returns the default name of the kind, used as the control identity.
    pub fn default_name(&self) -> String {
        match self {
            Self::ZoomIn => "zoom-in".to_string(),
            Self::ZoomOut => "zoom-out".to_string(),
            Self::Reset => "reset-view".to_string(),
            Self::Pan => "pan".to_string(),
            Self::SeriesToggle(series) => format!("series-toggle-{}", series.as_str()),
            Self::RangeSlider { axis } => format!("range-slider-{}", axis.as_str()),
            Self::RegionSelect => "region-select".to_string(),
            Self::Playback(command) => format!("playback-{}", command.as_str()),
        }
    }

    /// Returns the default label of the kind.
    pub fn default_label(&self) -> String {
        match self {
            Self::ZoomIn => "Zoom in".to_string(),
            Self::ZoomOut => "Zoom out".to_string(),
            Self::Reset => "Reset view".to_string(),
            Self::Pan => "Pan".to_string(),
            Self::SeriesToggle(series) => format!("Series {series}"),
            Self::RangeSlider { axis } => format!("{axis} range"),
            Self::RegionSelect => "Select region".to_string(),
            Self::Playback(command) => format!("Playback {}", command.as_str()),
        }
    }

    /// Returns the default size of the kind.
    pub fn default_size(&self) -> (f64, f64) {
        match self {
            Self::RangeSlider { .. } => (240.0, 16.0),
            Self::RegionSelect => (24.0, 24.0),
            Self::ZoomIn
            | Self::ZoomOut
            | Self::Reset
            | Self::Pan
            | Self::SeriesToggle(_)
            | Self::Playback(_) => (24.0, 24.0),
        }
    }
}

impl core::fmt::Display for SVControlKind {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(&self.default_label())
    }
}
