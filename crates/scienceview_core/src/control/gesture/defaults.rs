//! The documented default gesture table.

use super::binding::{SVGestureBinding, SVGestureMap};
use super::input::{SVGestureKind, SVKeyboardKey, SVPointerButton};
use crate::sv_control::{SVControlAction, SVControlAxis};

/// Returns the documented default gesture table.
pub fn sv_default_gesture_map() -> SVGestureMap {
    let zoom_in = 1.0 / 1.1;
    let zoom_out = 1.1;
    SVGestureMap::new([
        SVGestureBinding::new(
            SVGestureKind::Key(SVKeyboardKey::Plus),
            SVControlAction::Zoom {
                axis: SVControlAxis::Horizontal,
                factor: zoom_in,
                anchor: 0.5,
            },
        ),
        SVGestureBinding::new(
            SVGestureKind::Key(SVKeyboardKey::Minus),
            SVControlAction::Zoom {
                axis: SVControlAxis::Horizontal,
                factor: zoom_out,
                anchor: 0.5,
            },
        ),
        SVGestureBinding::new(
            SVGestureKind::Key(SVKeyboardKey::PageUp),
            SVControlAction::Zoom {
                axis: SVControlAxis::Vertical,
                factor: zoom_in,
                anchor: 0.5,
            },
        ),
        SVGestureBinding::new(
            SVGestureKind::Key(SVKeyboardKey::PageDown),
            SVControlAction::Zoom {
                axis: SVControlAxis::Vertical,
                factor: zoom_out,
                anchor: 0.5,
            },
        ),
        SVGestureBinding::new(
            SVGestureKind::Key(SVKeyboardKey::ArrowLeft),
            SVControlAction::Pan {
                axis: SVControlAxis::Horizontal,
                fraction: -0.1,
            },
        ),
        SVGestureBinding::new(
            SVGestureKind::Key(SVKeyboardKey::ArrowRight),
            SVControlAction::Pan {
                axis: SVControlAxis::Horizontal,
                fraction: 0.1,
            },
        ),
        SVGestureBinding::new(
            SVGestureKind::Key(SVKeyboardKey::ArrowUp),
            SVControlAction::Pan {
                axis: SVControlAxis::Vertical,
                fraction: 0.1,
            },
        ),
        SVGestureBinding::new(
            SVGestureKind::Key(SVKeyboardKey::ArrowDown),
            SVControlAction::Pan {
                axis: SVControlAxis::Vertical,
                fraction: -0.1,
            },
        ),
        SVGestureBinding::new(
            SVGestureKind::Key(SVKeyboardKey::Home),
            SVControlAction::Reset,
        ),
        SVGestureBinding::new(
            SVGestureKind::Key(SVKeyboardKey::Space),
            SVControlAction::SetPlayback(crate::sv_animation::SVAnimationAction::Toggle),
        ),
        SVGestureBinding::new(
            SVGestureKind::Key(SVKeyboardKey::BracketLeft),
            SVControlAction::SetPlayback(crate::sv_animation::SVAnimationAction::Step {
                forward: false,
            }),
        ),
        SVGestureBinding::new(
            SVGestureKind::Key(SVKeyboardKey::BracketRight),
            SVControlAction::SetPlayback(crate::sv_animation::SVAnimationAction::Step {
                forward: true,
            }),
        ),
        SVGestureBinding::new(
            SVGestureKind::Wheel(SVControlAxis::Horizontal),
            SVControlAction::Zoom {
                axis: SVControlAxis::Horizontal,
                factor: 0.9,
                anchor: 0.5,
            },
        ),
        SVGestureBinding::new(
            SVGestureKind::Wheel(SVControlAxis::Vertical),
            SVControlAction::Zoom {
                axis: SVControlAxis::Vertical,
                factor: 0.9,
                anchor: 0.5,
            },
        ),
        SVGestureBinding::new(
            SVGestureKind::Drag(SVPointerButton::Primary, SVControlAxis::Horizontal),
            SVControlAction::Pan {
                axis: SVControlAxis::Horizontal,
                fraction: 0.0,
            },
        ),
        SVGestureBinding::new(
            SVGestureKind::Drag(SVPointerButton::Primary, SVControlAxis::Vertical),
            SVControlAction::Pan {
                axis: SVControlAxis::Vertical,
                fraction: 0.0,
            },
        ),
    ])
}
