//! Gestures are a data table from input to semantic action.

use crate::sv_control::{
    SVControlAction, SVControlAxis, SVGesture, SVGestureKind, SVKeyboardKey, SVPointerButton,
    sv_default_gesture_map,
};

#[test]
fn the_keyboard_defaults_resolve_to_the_documented_actions() {
    let map = sv_default_gesture_map();

    assert!(matches!(
        map.resolve(&SVGesture::Key(SVKeyboardKey::Plus)),
        Some(SVControlAction::Zoom {
            axis: SVControlAxis::Horizontal,
            factor,
            anchor: 0.5,
        }) if factor < 1.0
    ));
    assert_eq!(
        map.resolve(&SVGesture::Key(SVKeyboardKey::Home)),
        Some(SVControlAction::Reset)
    );
    assert_eq!(
        map.resolve(&SVGesture::Key(SVKeyboardKey::ArrowLeft)),
        Some(SVControlAction::Pan {
            axis: SVControlAxis::Horizontal,
            fraction: -0.1,
        })
    );
    assert_eq!(
        map.resolve(&SVGesture::Key(SVKeyboardKey::Space)),
        Some(SVControlAction::SetPlayback(
            crate::sv_animation::SVAnimationAction::Toggle
        ))
    );
}

#[test]
fn a_wheel_gesture_zooms_in_the_direction_of_its_delta() {
    let map = sv_default_gesture_map();

    let inward = map
        .resolve(&SVGesture::Wheel {
            axis: SVControlAxis::Horizontal,
            delta: 1.0,
        })
        .expect("the wheel is bound");
    let outward = map
        .resolve(&SVGesture::Wheel {
            axis: SVControlAxis::Horizontal,
            delta: -1.0,
        })
        .expect("the wheel is bound");

    match (inward, outward) {
        (
            SVControlAction::Zoom { factor: inward, .. },
            SVControlAction::Zoom {
                factor: outward, ..
            },
        ) => {
            assert!(inward < 1.0);
            assert!(outward > 1.0);
            assert!((inward * outward - 1.0).abs() < 1e-12);
        }
        other => panic!("the wheel must resolve to zoom actions, received {other:?}"),
    }
}

#[test]
fn a_drag_gesture_pans_by_the_dragged_fraction() {
    let map = sv_default_gesture_map();

    let action = map
        .resolve(&SVGesture::Drag {
            button: SVPointerButton::Primary,
            axis: SVControlAxis::Vertical,
            fraction: 0.25,
        })
        .expect("the drag is bound");

    assert_eq!(
        action,
        SVControlAction::Pan {
            axis: SVControlAxis::Vertical,
            fraction: 0.25,
        }
    );
}

#[test]
fn an_unbound_gesture_resolves_to_nothing() {
    let map = sv_default_gesture_map();

    assert_eq!(
        map.resolve(&SVGesture::Drag {
            button: SVPointerButton::Secondary,
            axis: SVControlAxis::Horizontal,
            fraction: 0.5,
        }),
        None
    );
}

#[test]
fn a_binding_can_be_replaced_without_touching_the_others() {
    let mut map = sv_default_gesture_map();
    map.bind(
        SVGestureKind::Key(SVKeyboardKey::Home),
        SVControlAction::Pan {
            axis: SVControlAxis::Horizontal,
            fraction: 1.0,
        },
    );

    assert_eq!(
        map.resolve(&SVGesture::Key(SVKeyboardKey::Home)),
        Some(SVControlAction::Pan {
            axis: SVControlAxis::Horizontal,
            fraction: 1.0,
        })
    );
    assert!(map.bindings().len() > 1);
}
