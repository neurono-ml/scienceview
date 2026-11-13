//! Keyboard and pointer gestures as data, resolved to semantic actions.
//!
//! The map owns no toolkit type and no clock: a backend translates its event
//! into an [`SVGesture`] and asks the map for exactly one action. The same map
//! therefore drives every backend, and a user can replace a binding without
//! rebuilding a control.

use crate::sv_control::SVControlAxis;

/// A key the kit binds by default.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVKeyboardKey {
    /// `+` / `=`.
    Plus,
    /// `-`.
    Minus,
    /// `Home`.
    Home,
    /// `PageUp`.
    PageUp,
    /// `PageDown`.
    PageDown,
    /// `ArrowLeft`.
    ArrowLeft,
    /// `ArrowRight`.
    ArrowRight,
    /// `ArrowUp`.
    ArrowUp,
    /// `ArrowDown`.
    ArrowDown,
    /// `Space`.
    Space,
    /// `[`.
    BracketLeft,
    /// `]`.
    BracketRight,
}

/// A pointer button the kit binds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVPointerButton {
    /// The primary button.
    Primary,
    /// The secondary button.
    Secondary,
}

/// A raw gesture a backend has already translated into toolkit-free data.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVGesture {
    /// A key press.
    Key(SVKeyboardKey),
    /// A wheel turn on one axis; the sign is the direction.
    Wheel {
        /// The axis the wheel scrolls.
        axis: SVControlAxis,
        /// The signed wheel amount.
        delta: f64,
    },
    /// A drag on one axis; the fraction is the drag distance over the axis span.
    Drag {
        /// The pressed button.
        button: SVPointerButton,
        /// The axis the drag follows.
        axis: SVControlAxis,
        /// The signed drag distance as a fraction of the axis span.
        fraction: f64,
    },
}

/// The gesture class a binding matches, without its measured amount.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVGestureKind {
    /// A key press.
    Key(SVKeyboardKey),
    /// A wheel on one axis.
    Wheel(SVControlAxis),
    /// A drag with one button on one axis.
    Drag(SVPointerButton, SVControlAxis),
}
