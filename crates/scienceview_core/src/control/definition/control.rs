//! `SVControl`: a configured model object that derives a semantic action.

use crate::sv_control::{
    SVControlAction, SVControlAxis, SVControlKind, SVControlPlacement, SVControlRange,
    SVControlSize, SVControlStyle, SVSelectionRegion,
};

/// The configuration the builder hands over once it is validated.
pub(crate) struct SVControlConfiguration {
    pub(crate) kind: SVControlKind,
    pub(crate) name: String,
    pub(crate) label: String,
    pub(crate) size: SVControlSize,
    pub(crate) placement: SVControlPlacement,
    pub(crate) style: SVControlStyle,
    pub(crate) enabled: bool,
    pub(crate) target_axis: SVControlAxis,
    pub(crate) step: f64,
    pub(crate) bounds: Option<SVControlRange>,
    pub(crate) range: SVControlRange,
    pub(crate) region: SVSelectionRegion,
    pub(crate) action_override: Option<SVControlAction>,
    pub(crate) script: Option<String>,
}

/// A control as a model object.
///
/// It is not an artist and never enters the mark pipeline: activating it
/// produces one [`SVControlAction`] and the reducer applies it. The same object
/// can be placed inside the figure frame or in an external host arrangement;
/// placement changes nothing about the action it dispatches.
#[derive(Clone, Debug, PartialEq)]
pub struct SVControl {
    pub(super) kind: SVControlKind,
    pub(super) name: String,
    pub(super) label: String,
    pub(super) size: SVControlSize,
    pub(super) placement: SVControlPlacement,
    pub(super) style: SVControlStyle,
    pub(super) enabled: bool,
    pub(super) target_axis: SVControlAxis,
    pub(super) step: f64,
    pub(super) bounds: Option<SVControlRange>,
    pub(super) range: SVControlRange,
    pub(super) region: SVSelectionRegion,
    pub(super) action_override: Option<SVControlAction>,
    pub(super) script: Option<String>,
}

impl SVControl {
    pub(crate) fn sv_from_configuration(configuration: SVControlConfiguration) -> Self {
        Self {
            kind: configuration.kind,
            name: configuration.name,
            label: configuration.label,
            size: configuration.size,
            placement: configuration.placement,
            style: configuration.style,
            enabled: configuration.enabled,
            target_axis: configuration.target_axis,
            step: configuration.step,
            bounds: configuration.bounds,
            range: configuration.range,
            region: configuration.region,
            action_override: configuration.action_override,
            script: configuration.script,
        }
    }

    /// Returns the control identity used as the event origin.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the label the control shows.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the reserved size.
    pub fn size(&self) -> SVControlSize {
        self.size
    }

    /// Returns where the control is drawn.
    pub fn placement(&self) -> SVControlPlacement {
        self.placement
    }

    /// Returns the style the control draws with.
    pub fn style(&self) -> &SVControlStyle {
        &self.style
    }

    /// Returns the kind of the control.
    pub fn kind(&self) -> SVControlKind {
        self.kind.clone()
    }

    /// Returns whether the control reacts to activation.
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Returns the target axis of a zoom, pan or range control.
    pub fn target_axis(&self) -> SVControlAxis {
        self.target_axis
    }

    /// Returns the step of a zoom or pan control.
    pub fn step(&self) -> f64 {
        self.step
    }

    /// Returns the bounds of the range slider, when configured.
    pub fn bounds(&self) -> Option<SVControlRange> {
        self.bounds
    }

    /// Returns the current interval of the range slider.
    pub fn range(&self) -> Option<SVControlRange> {
        matches!(self.kind, SVControlKind::RangeSlider { .. }).then_some(self.range)
    }

    /// Returns the current region of the region selector.
    pub fn region(&self) -> SVSelectionRegion {
        self.region
    }

    /// Returns the explicit action override, when configured.
    pub fn action_override(&self) -> Option<&SVControlAction> {
        self.action_override.as_ref()
    }

    /// Returns the script source, when one is attached.
    ///
    /// The control only carries the source. Running it belongs to the scripting
    /// engine, which receives the same event payloads through the session.
    pub fn script(&self) -> Option<&str> {
        self.script.as_deref()
    }
}
