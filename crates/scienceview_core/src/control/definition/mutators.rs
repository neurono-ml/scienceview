//! The mutators a host uses to update a control at runtime.

use crate::sv_animation::SVDuration;
use crate::sv_control::{
    SVControlAction, SVControlAxis, SVControlKind, SVControlPlacement, SVControlRange,
    SVControlSize, SVControlStyle, SVPlaybackCommand, SVSelectionRegion, SVSeriesReference,
};

use super::control::SVControl;
use super::step::sv_validated_step;

impl SVControl {
    /// Enables or disables the control.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Replaces the label.
    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = label.into();
    }

    /// Replaces the reserved size.
    pub fn set_size(&mut self, size: SVControlSize) {
        self.size = size;
    }

    /// Replaces the placement.
    pub fn set_placement(&mut self, placement: SVControlPlacement) {
        self.placement = placement;
    }

    /// Replaces the style.
    pub fn set_style(&mut self, style: SVControlStyle) {
        self.style = style;
    }

    /// Replaces the zoom or pan step.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`](crate::sv_error::SVError::InvalidConfiguration)
    /// when the step is not finite or not strictly positive.
    pub fn set_step(&mut self, step: f64) -> Result<(), crate::sv_error::SVError> {
        self.step = sv_validated_step(step)?;
        Ok(())
    }

    /// Replaces the target axis.
    pub fn set_target_axis(&mut self, axis: SVControlAxis) {
        self.target_axis = axis;
        if let SVControlKind::RangeSlider { axis: current } = &mut self.kind {
            *current = axis;
        }
    }

    /// Replaces the range-slider bounds and clamps the current range.
    pub fn set_bounds(&mut self, bounds: Option<SVControlRange>) {
        self.bounds = bounds;
        if let Some(bounds) = bounds {
            self.range = self.range.clamp_into(bounds);
        }
    }

    /// Replaces the range-slider interval, clamped into the bounds.
    pub fn set_range(&mut self, range: SVControlRange) {
        self.range = match self.bounds {
            Some(bounds) => range.clamp_into(bounds),
            None => range,
        };
    }

    /// Replaces the selected region.
    pub fn set_region(&mut self, region: SVSelectionRegion) {
        self.region = region;
    }

    /// Replaces the playback command of a playback control.
    pub fn set_playback_command(&mut self, command: SVPlaybackCommand) {
        if matches!(self.kind, SVControlKind::Playback(_)) {
            self.kind = SVControlKind::Playback(command);
        }
    }

    /// Replaces the playback target of a seek control.
    pub fn set_seek_target(&mut self, target: SVDuration) {
        self.set_playback_command(SVPlaybackCommand::Seek(target));
    }

    /// Overrides the action the control dispatches.
    pub fn set_action_override(&mut self, action: SVControlAction) {
        self.action_override = Some(action);
    }

    /// Removes the action override, restoring the derived action.
    pub fn clear_action_override(&mut self) {
        self.action_override = None;
    }

    /// Attaches a script source the scripting engine runs on activation.
    pub fn attach_script(&mut self, source: impl Into<String>) {
        self.script = Some(source.into());
    }

    /// Returns whether this control targets a series.
    pub fn series(&self) -> Option<&SVSeriesReference> {
        match &self.kind {
            SVControlKind::SeriesToggle(series) => Some(series),
            _ => None,
        }
    }
}
