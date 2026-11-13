//! The builder: the only public way to assemble an [`SVControl`].

use crate::sv_control::{
    SVControlAction, SVControlAxis, SVControlKind, SVControlPlacement, SVControlRange,
    SVControlSize, SVControlStyle, SVPlaybackCommand, SVSelectionRegion, SVSeriesReference,
};

/// Builds a configured [`SVControl`].
///
/// Every option is validated at [`build`](SVControlBuilder::build): a size has
/// two positive extents, a step is positive, a range is ordered and lies within
/// its bounds, and a playback speed is positive. The control's direct
/// constructor is private, so an invalid control cannot be produced.
#[derive(Clone, Debug)]
pub struct SVControlBuilder {
    pub(super) kind: SVControlKind,
    pub(super) name: Option<String>,
    pub(super) label: Option<String>,
    pub(super) size: Option<SVControlSize>,
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

impl SVControlBuilder {
    /// Starts a control of the given kind.
    pub fn new(kind: SVControlKind) -> Self {
        Self {
            kind,
            name: None,
            label: None,
            size: None,
            placement: SVControlPlacement::InFrame,
            style: SVControlStyle::built_in(),
            enabled: true,
            target_axis: SVControlAxis::Horizontal,
            step: 0.1,
            bounds: None,
            range: SVControlRange::unit(),
            region: SVSelectionRegion::unit(),
            action_override: None,
            script: None,
        }
    }

    /// Starts a zoom-in control.
    pub fn zoom_in() -> Self {
        Self::new(SVControlKind::ZoomIn)
    }

    /// Starts a zoom-out control.
    pub fn zoom_out() -> Self {
        Self::new(SVControlKind::ZoomOut)
    }

    /// Starts a reset control.
    pub fn reset() -> Self {
        Self::new(SVControlKind::Reset)
    }

    /// Starts a pan control.
    pub fn pan() -> Self {
        Self::new(SVControlKind::Pan)
    }

    /// Starts a series-visibility control for one series.
    pub fn series_toggle(series: SVSeriesReference) -> Self {
        Self::new(SVControlKind::SeriesToggle(series))
    }

    /// Starts a range slider for one axis.
    pub fn range_slider(axis: SVControlAxis) -> Self {
        Self::new(SVControlKind::RangeSlider { axis }).target_axis(axis)
    }

    /// Starts a region-selection control.
    pub fn region_select() -> Self {
        Self::new(SVControlKind::RegionSelect)
    }

    /// Starts a playback control carrying one animation command.
    pub fn playback(command: SVPlaybackCommand) -> Self {
        Self::new(SVControlKind::Playback(command))
    }

    /// Sets the control identity used as the event origin.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the label the control shows.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the reserved size.
    #[must_use]
    pub fn size(mut self, size: SVControlSize) -> Self {
        self.size = Some(size);
        self
    }

    /// Places the control inside the frame or in an external host.
    #[must_use]
    pub fn placement(mut self, placement: SVControlPlacement) -> Self {
        self.placement = placement;
        self
    }

    /// Sets the style the control draws with.
    #[must_use]
    pub fn style(mut self, style: SVControlStyle) -> Self {
        self.style = style;
        self
    }

    /// Enables or disables the control.
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Sets the axis a zoom, pan or range control targets.
    #[must_use]
    pub fn target_axis(mut self, axis: SVControlAxis) -> Self {
        self.target_axis = axis;
        self
    }

    /// Sets the zoom or pan step.
    #[must_use]
    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    /// Sets the range-slider bounds.
    #[must_use]
    pub fn bounds(mut self, bounds: SVControlRange) -> Self {
        self.bounds = Some(bounds);
        self
    }

    /// Sets the range-slider interval.
    #[must_use]
    pub fn range(mut self, range: SVControlRange) -> Self {
        self.range = range;
        self
    }

    /// Sets the region selector's region.
    #[must_use]
    pub fn region(mut self, region: SVSelectionRegion) -> Self {
        self.region = region;
        self
    }

    /// Overrides the action the control dispatches.
    #[must_use]
    pub fn action(mut self, action: SVControlAction) -> Self {
        self.action_override = Some(action);
        self
    }

    /// Attaches a script source.
    #[must_use]
    pub fn script(mut self, source: impl Into<String>) -> Self {
        self.script = Some(source.into());
        self
    }
}
