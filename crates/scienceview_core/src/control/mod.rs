//! Controls: model objects that dispatch one semantic action vocabulary.
//!
//! A control is not an artist. It never enters the mark pipeline: it carries a
//! label, a size, a placement, a style resolved from theme tokens and a
//! behaviour, and activating it produces an [`SVControlAction`]. The
//! [`SVControlState`] reducer applies that action to axis domains, series
//! visibility, the selection and the animation timeline, and emits
//! [`SVControlEvent`] payloads. A third-party widget is an adapter over the very
//! same vocabulary, so the built-in controls and an external toolkit cannot
//! diverge.

mod action;
mod appearance;
mod axis;
mod builder;
mod definition;
mod event;
mod gesture;
mod kind;
mod range;
mod selection;
mod series;
mod session;
mod standard;
mod state;
mod surface;

pub use action::SVControlAction;
pub use appearance::{SVControlPlacement, SVControlSize, SVControlStyle};
pub use axis::SVControlAxis;
pub use builder::SVControlBuilder;
pub use definition::SVControl;
pub use event::{
    SVControlChange, SVControlEvent, SVControlOrigin, SVCustomPayload, SVPlaybackChange,
    SVRangeChange, SVSelectionChange, SVSeriesVisibilityChange, SVViewReset,
};
pub use gesture::{
    SVGesture, SVGestureBinding, SVGestureKind, SVGestureMap, SVKeyboardKey, SVPointerButton,
    sv_default_gesture_map,
};
pub use kind::{SVControlKind, SVPlaybackCommand};
pub use range::{SVAxisRange, SVControlRange};
pub use selection::SVSelectionRegion;
pub use series::SVSeriesReference;
pub use session::{SVControlSession, SVControlSubscription};
pub use standard::{sv_playback_controls, sv_standard_controls};
pub use state::{SVControlDispatch, SVControlState};
pub use surface::{SVControlSurface, SVWidgetAdapter};

#[cfg(test)]
#[path = "concurrency_tests.rs"]
mod concurrency_tests;
#[cfg(test)]
#[path = "control_tests.rs"]
mod control_tests;
#[cfg(test)]
#[path = "dispatch_model_tests.rs"]
mod dispatch_model_tests;
#[cfg(test)]
#[path = "dispatch_tests.rs"]
mod dispatch_tests;
#[cfg(test)]
#[path = "gesture_tests.rs"]
mod gesture_tests;
#[cfg(test)]
#[path = "integration_tests.rs"]
mod integration_tests;
#[cfg(test)]
#[path = "playback_tests.rs"]
mod playback_tests;
#[cfg(test)]
#[path = "representation_tests.rs"]
mod representation_tests;
