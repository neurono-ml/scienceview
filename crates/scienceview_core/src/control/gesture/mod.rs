//! Keyboard and pointer gestures as data, resolved to semantic actions.

mod binding;
mod defaults;
mod input;

pub use binding::{SVGestureBinding, SVGestureMap};
pub use defaults::sv_default_gesture_map;
pub use input::{SVGesture, SVGestureKind, SVKeyboardKey, SVPointerButton};
