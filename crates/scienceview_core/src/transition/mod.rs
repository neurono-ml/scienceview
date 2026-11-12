//! Mark transitions: key matching and property interpolation (`FDR-005`).
//!
//! A figure that updates by replacing its whole contents is unreadable: the eye
//! cannot follow a mark from one state to the next. [`SVTransition`] is the
//! layer that turns a sequence of static figures into a continuous experience.
//! It sits between the model deltas and the backend frame loop:
//!
//! ```text
//! state A --(delta)--> state B
//!    |                     |
//!    |                  resolve keys
//!    |                     |
//!    +--> enter / update / exit sets
//! ```
//!
//! ## Matching is by key, never by position
//!
//! Marks are matched on a stable identity (design D1). [`SVMatchKey`] is a
//! closed enumeration — `Category`, `Index`, `Identity` and `Derived` — and a
//! key resolves into a typed [`SVMatchKeyValue`], so a category code never
//! equals an identity code by accident. [`sv_resolve_match_keys`] reads the key
//! column from the closed [`SVColumnView`](crate::sv_data::SVColumnView)
//! representation exactly once: a sparse column is read over its stored
//! observations and never densified, and a streaming column is read from the
//! bounded window snapshot it already carries, so a live figure never blocks the
//! render thread. Two marks of one state that resolve to the same key are
//! rejected before any interpolation, because the match would be ambiguous.
//!
//! ## Only the declared properties tween
//!
//! [`SVTransition::plan`] computes the three sets from two states:
//!
//! | Set | Condition | Treatment |
//! |---|---|---|
//! | enter | the key is in the next state but not in the previous one | the enter rule applies |
//! | update | the key is in both states | properties interpolate |
//! | exit | the key is in the previous state but not in the next one | the exit rule applies |
//!
//! Interpolation is driven by the interpolable-property table of the mark
//! kernel ([`sv_mark_property_table`](crate::sv_mark::sv_mark_property_table)):
//! position, size, colour, opacity and font size tween while shape, dash pattern
//! and font family hold their previous value and swap at the end of the
//! transition. The transition keeps no second copy of that table.
//!
//! ```
//! use scienceview_core::sv_mark::SVMarkPoint;
//! use scienceview_core::sv_transition::{
//!     SVEasing, SVMarkPropertyValue, SVMarkState, SVMatchKeyValue, SVTransition,
//! };
//!
//! let key = SVMatchKeyValue::category(1.0).expect("the category code is finite");
//! let previous = [SVMarkState::new(key)
//!     .with_property(SVMarkPropertyValue::position(SVMarkPoint::new(0.0, 0.0)))];
//! let next = [SVMarkState::new(key)
//!     .with_property(SVMarkPropertyValue::position(SVMarkPoint::new(10.0, 0.0)))];
//!
//! let plan = SVTransition::new()
//!     .easing(SVEasing::Linear)
//!     .plan(&previous, &next)
//!     .expect("the keys are unique");
//! let frame = plan.evaluate(0.5).expect("halfway is a valid progress");
//!
//! assert_eq!(
//!     frame.mark(key).and_then(|mark| mark.properties().position()),
//!     Some(SVMarkPoint::new(5.0, 0.0))
//! );
//! ```
//!
//! ## Motion follows the theme
//!
//! Durations and easings are motion tokens (design D4). With the `theme`
//! feature `SVMotionStyle::from_motion_tokens` reads them from a compiled
//! theme; without it [`SVMotionStyle::built_in`] supplies the documented
//! defaults (fast 120 ms, normal 240 ms, slow 400 ms, standard easing). A theme
//! that declares reduced motion collapses every duration to zero and the plan
//! draws its final state at any progress, with no override offered.
//!
//! ## The kernel owns no clock
//!
//! Time belongs to the caller — the animation timeline. [`SVTransitionPlan`]
//! evaluates a frame from a progress in `0..=1` ([`SVTransitionPlan::evaluate`])
//! or from a caller duration ([`SVTransitionPlan::progress_at`]). A finite
//! progress outside the unit interval is clamped and a non-finite one is
//! rejected. Evaluation is a pure function of the plan and the progress, so the
//! same inputs always produce the same frame, which is also what makes a plan
//! safe to evaluate from several threads.
//!
//! ## Backends declare transition support
//!
//! [`sv_transition_support`] derives the animation capability from the backend
//! capability matrix: a backend that declares in-place update animates, and a
//! static export backend resolves the transition to its final state instead of
//! pretending to own a clock.

mod easing;
mod enter_exit;
mod frame;
mod interpolated_properties;
mod interpolation;
mod key_resolution;
mod mark_properties;
mod mark_property_value;
mod mark_state;
mod match_key;
mod motion_style;
mod plan;
mod plan_evaluation;
mod support;
mod transition;
mod transition_span;

pub use easing::SVEasing;
pub use enter_exit::{SV_TRANSITION_SLIDE_DISTANCE, SVEnterRule, SVExitRule};
pub use frame::{SVResolvedMark, SVTransitionFrame, SVTransitionPhase};
pub use interpolated_properties::SVInterpolatedProperties;
pub use key_resolution::{sv_resolve_match_keys, sv_verify_unique_match_keys};
pub use mark_properties::SVMarkProperties;
pub use mark_property_value::SVMarkPropertyValue;
pub use mark_state::SVMarkState;
pub use match_key::{SVMatchKey, SVMatchKeyValue};
pub use motion_style::SVMotionStyle;
pub use plan::{SVTransitionEnter, SVTransitionExit, SVTransitionPlan, SVTransitionUpdate};
pub use support::{SVTransitionSupport, sv_transition_support};
pub use transition::SVTransition;

#[cfg(test)]
#[path = "backend_contract_tests.rs"]
mod backend_contract_tests;
#[cfg(test)]
#[path = "concurrency_tests.rs"]
mod concurrency_tests;
#[cfg(test)]
#[path = "discrete_property_tests.rs"]
mod discrete_property_tests;
#[cfg(test)]
#[path = "enter_exit_tests.rs"]
mod enter_exit_tests;
#[cfg(test)]
#[path = "interpolation_tests.rs"]
mod interpolation_tests;
#[cfg(test)]
#[path = "match_key_tests.rs"]
mod match_key_tests;
#[cfg(test)]
#[path = "motion_tests.rs"]
mod motion_tests;
#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
#[cfg(test)]
#[path = "property_set_tests.rs"]
mod property_set_tests;
#[cfg(test)]
#[path = "representation_read_tests.rs"]
mod representation_read_tests;
#[cfg(test)]
#[path = "set_tests.rs"]
mod set_tests;
#[cfg(test)]
mod transition_test_support;
