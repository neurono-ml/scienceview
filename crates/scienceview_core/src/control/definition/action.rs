//! The action a configured control derives.

use crate::sv_control::{SVControlAction, SVControlKind};

use super::control::SVControl;

impl SVControl {
    /// Returns the action this control dispatches.
    ///
    /// An explicit override wins. Otherwise the kind derives the action from
    /// the target axis, the step, the current range or the current region.
    pub fn action(&self) -> SVControlAction {
        if let Some(action) = &self.action_override {
            return action.clone();
        }
        match &self.kind {
            SVControlKind::ZoomIn => SVControlAction::Zoom {
                axis: self.target_axis,
                factor: 1.0 / (1.0 + self.step),
                anchor: 0.5,
            },
            SVControlKind::ZoomOut => SVControlAction::Zoom {
                axis: self.target_axis,
                factor: 1.0 + self.step,
                anchor: 0.5,
            },
            SVControlKind::Reset => SVControlAction::Reset,
            SVControlKind::Pan => SVControlAction::Pan {
                axis: self.target_axis,
                fraction: self.step,
            },
            SVControlKind::SeriesToggle(series) => SVControlAction::ToggleSeries(series.clone()),
            SVControlKind::RangeSlider { axis } => SVControlAction::SetRange {
                axis: *axis,
                range: self.range,
            },
            SVControlKind::RegionSelect => SVControlAction::SelectRegion(self.region),
            SVControlKind::Playback(command) => SVControlAction::SetPlayback(command.action()),
        }
    }
}
