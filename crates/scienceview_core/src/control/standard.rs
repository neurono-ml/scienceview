//! The standard control set every figure can ask for.

use super::definition::{SVControl, SVControlConfiguration};
use super::{
    SVControlAxis, SVControlKind, SVControlPlacement, SVControlRange, SVControlSize,
    SVControlStyle, SVPlaybackCommand, SVSelectionRegion, SVSeriesReference,
};

/// Returns the standard controls of the kit.
///
/// The set covers the specification minimum: zoom in and out, reset, pan,
/// one visibility toggle per series, a horizontal range slider and the region
/// selector. A host that wants another axis adds its own range slider with
/// [`SVControlBuilder`](super::SVControlBuilder).
pub fn sv_standard_controls(series: impl IntoIterator<Item = SVSeriesReference>) -> Vec<SVControl> {
    let mut controls = vec![
        sv_standard_control(SVControlKind::ZoomIn),
        sv_standard_control(SVControlKind::ZoomOut),
        sv_standard_control(SVControlKind::Reset),
        sv_standard_control(SVControlKind::Pan),
    ];
    for series in series {
        controls.push(sv_standard_control(SVControlKind::SeriesToggle(series)));
    }
    controls.push(sv_standard_control(SVControlKind::RangeSlider {
        axis: SVControlAxis::Horizontal,
    }));
    controls.push(sv_standard_control(SVControlKind::RegionSelect));
    controls
}

/// Returns the playback controls: play, pause, seek, step and speed.
pub fn sv_playback_controls() -> Vec<SVControl> {
    vec![
        sv_standard_control(SVControlKind::Playback(SVPlaybackCommand::Play)),
        sv_standard_control(SVControlKind::Playback(SVPlaybackCommand::Pause)),
        sv_standard_control(SVControlKind::Playback(SVPlaybackCommand::Seek(
            crate::sv_animation::SVDuration::zero(),
        ))),
        sv_standard_control(SVControlKind::Playback(SVPlaybackCommand::Step {
            forward: true,
        })),
        sv_standard_control(SVControlKind::Playback(SVPlaybackCommand::Step {
            forward: false,
        })),
        sv_standard_control(SVControlKind::Playback(SVPlaybackCommand::Speed(1.5))),
    ]
}

/// Builds one standard control from constants the builder also validates.
///
/// The standard set must be infallible, so it transfers a configuration whose
/// every value is a compile-time constant: the default label, the documented
/// size, the default step and the unit range. No user input reaches this path.
fn sv_standard_control(kind: SVControlKind) -> SVControl {
    let (width, height) = kind.default_size();
    SVControl::sv_from_configuration(SVControlConfiguration {
        name: kind.default_name(),
        label: kind.default_label(),
        size: SVControlSize::sv_from_validated(width, height),
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
        kind,
    })
}
