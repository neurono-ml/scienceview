//! The control model: configuration, derived actions, enabling and bounds.

use crate::sv_control::{
    SVControlAction, SVControlAxis, SVControlBuilder, SVControlKind, SVControlPlacement,
    SVControlRange, SVControlSize, SVPlaybackCommand, SVSelectionRegion, SVSeriesReference,
    SVWidgetAdapter, sv_playback_controls, sv_standard_controls,
};
use crate::sv_error::SVError;

fn sv_alpha() -> SVSeriesReference {
    SVSeriesReference::Name("alpha".to_string())
}

#[test]
fn a_control_carries_its_configuration() {
    let control = SVControlBuilder::zoom_in()
        .name("zoom-in-primary")
        .label("Zoom in")
        .size(SVControlSize::new(32.0, 24.0).expect("finite size"))
        .placement(SVControlPlacement::Host)
        .target_axis(SVControlAxis::Vertical)
        .step(0.25)
        .build()
        .expect("the configuration is complete");

    assert_eq!(control.name(), "zoom-in-primary");
    assert_eq!(control.label(), "Zoom in");
    assert_eq!(control.size().width(), 32.0);
    assert_eq!(control.size().height(), 24.0);
    assert_eq!(control.placement(), SVControlPlacement::Host);
    assert_eq!(control.kind(), SVControlKind::ZoomIn);
    assert!(control.enabled());
}

#[test]
fn a_zoom_control_derives_a_zoom_action_with_its_step_and_axis() {
    let control = SVControlBuilder::zoom_out()
        .target_axis(SVControlAxis::Vertical)
        .step(0.5)
        .build()
        .expect("the configuration is complete");

    let action = control.action();
    assert_eq!(
        action,
        SVControlAction::Zoom {
            axis: SVControlAxis::Vertical,
            factor: 1.5,
            anchor: 0.5,
        }
    );
}

#[test]
fn a_disabled_control_refuses_to_activate() {
    let control = SVControlBuilder::reset()
        .build()
        .expect("the configuration is complete");
    let mut control = control;
    control.set_enabled(false);

    assert_eq!(
        control.activate(),
        Err(SVError::ControlDisabled {
            name: control.name().to_string(),
        })
    );
}

#[test]
fn a_range_slider_clamps_its_value_to_its_bounds() {
    let mut control = SVControlBuilder::range_slider(SVControlAxis::Horizontal)
        .bounds(SVControlRange::new(0.0, 100.0).expect("ordered bounds"))
        .range(SVControlRange::new(10.0, 50.0).expect("ordered range"))
        .build()
        .expect("the configuration is complete");

    control.set_range(SVControlRange::new(80.0, 140.0).expect("ordered range"));

    assert_eq!(
        control.range(),
        Some(SVControlRange::new(80.0, 100.0).expect("clamped range"))
    );
    assert_eq!(
        control.action(),
        SVControlAction::SetRange {
            axis: SVControlAxis::Horizontal,
            range: SVControlRange::new(80.0, 100.0).expect("clamped range"),
        }
    );
}

#[test]
fn an_oversized_size_is_rejected() {
    assert!(SVControlSize::new(0.0, 24.0).is_err());
    assert!(SVControlSize::new(f64::NAN, 24.0).is_err());
}

#[test]
fn an_action_override_replaces_the_derived_action() {
    let override_action = SVControlAction::Pan {
        axis: SVControlAxis::Horizontal,
        fraction: 0.2,
    };
    let control = SVControlBuilder::zoom_in()
        .action(override_action.clone())
        .build()
        .expect("the configuration is complete");

    assert_eq!(control.action(), override_action);
}

#[test]
fn a_series_toggle_targets_its_series() {
    let control = SVControlBuilder::series_toggle(sv_alpha())
        .build()
        .expect("the configuration is complete");

    assert_eq!(control.action(), SVControlAction::ToggleSeries(sv_alpha()));
}

#[test]
fn a_region_select_control_carries_its_region() {
    let region = SVSelectionRegion::new(
        SVControlRange::new(1.0, 2.0).expect("ordered range"),
        SVControlRange::new(3.0, 4.0).expect("ordered range"),
    );
    let control = SVControlBuilder::region_select()
        .region(region)
        .build()
        .expect("the configuration is complete");

    assert_eq!(control.action(), SVControlAction::SelectRegion(region));
}

#[test]
fn the_standard_controls_cover_the_specification_minimum() {
    let controls = sv_standard_controls([sv_alpha()]);

    let kinds: Vec<SVControlKind> = controls.iter().map(|control| control.kind()).collect();
    assert!(kinds.contains(&SVControlKind::ZoomIn));
    assert!(kinds.contains(&SVControlKind::ZoomOut));
    assert!(kinds.contains(&SVControlKind::Reset));
    assert!(kinds.contains(&SVControlKind::Pan));
    assert!(kinds.contains(&SVControlKind::SeriesToggle(sv_alpha())));
    assert!(kinds.contains(&SVControlKind::RangeSlider {
        axis: SVControlAxis::Horizontal,
    }));
    assert!(kinds.contains(&SVControlKind::RegionSelect));
}

#[test]
fn the_playback_controls_cover_play_pause_seek_step_and_speed() {
    let controls = sv_playback_controls();
    let kinds: Vec<SVControlKind> = controls.iter().map(|control| control.kind()).collect();

    assert!(kinds.contains(&SVControlKind::Playback(SVPlaybackCommand::Play)));
    assert!(kinds.contains(&SVControlKind::Playback(SVPlaybackCommand::Pause)));
    assert!(
        kinds.contains(&SVControlKind::Playback(SVPlaybackCommand::Seek(
            crate::sv_animation::SVDuration::zero()
        )))
    );
    assert!(
        kinds.contains(&SVControlKind::Playback(SVPlaybackCommand::Step {
            forward: true,
        }))
    );
    assert!(
        kinds.contains(&SVControlKind::Playback(SVPlaybackCommand::Step {
            forward: false,
        }))
    );
    assert!(kinds.contains(&SVControlKind::Playback(SVPlaybackCommand::Speed(1.5))));
}

#[test]
fn an_external_widget_is_an_adapter_over_the_same_action() {
    let action = SVControlAction::Zoom {
        axis: SVControlAxis::Horizontal,
        factor: 0.5,
        anchor: 0.5,
    };
    let widget = SVWidgetAdapter::new("framework-zoom", action.clone());

    assert_eq!(widget.name(), "framework-zoom");
    assert_eq!(widget.action(), action);
    assert!(widget.enabled());
}
