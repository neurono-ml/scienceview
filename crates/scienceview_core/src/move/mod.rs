//! Positional moves: stacking, dodging, jittering, shifting, swarming and
//! normalising marks in data space (`FDR-003`).
//!
//! A move sits between the statistical transform and the scale:
//!
//! ```text
//! data -> [SVStat] -> values -> [SVMove] -> [SVScale] -> SVMark
//! ```
//!
//! It never changes the data it was given (design D1): it produces a
//! displacement, a narrowed width, and — for stacking and normalising — a new
//! value computed from that data. The original values stay borrowed from the
//! request, so a tooltip or an event still reads what the statistic produced.
//!
//! ## The closed vocabulary
//!
//! [`SVMove`] is a closed enumeration (patterns P5) with exactly six variants,
//! each carrying only its typed option surface ([`SVStackOptions`],
//! [`SVDodgeOptions`], [`SVJitterOptions`], [`SVShiftOptions`],
//! [`SVSwarmOptions`], [`SVNormalizeOptions`]). A new variant is a change to
//! this kernel, never something a recipe adds.
//!
//! ## The fixed composition order
//!
//! Several moves compose in a documented order, because order changes the
//! result (design D2): the displacement stage first — `Stack`, `Dodge` or
//! `Jitter`, mutually exclusive — then `Shift`, then `Normalize`.
//! [`SVMoveSequence::new`] rejects any other ordering with a typed
//! [`SVError`](crate::sv_error::SVError) before a single value is touched, so
//! an invalid pipeline cannot reach a render.
//!
//! ```
//! use scienceview_core::sv_data::SVColumnView;
//! use scienceview_core::sv_move::{SVMove, SVMoveRequest, SVMoveSequence};
//!
//! let values = [1.0, 2.0, 3.0];
//! let group_keys = [0, 0, 0];
//! let sequence = SVMoveSequence::new(vec![
//!     SVMove::stack().into(),
//!     SVMove::normalize().into(),
//! ])
//! .expect("stack then normalize is the documented order");
//!
//! let request = SVMoveRequest::new(SVColumnView::Dense(&values), &group_keys);
//! let moved = sequence.apply(&request).expect("the move sequence applies");
//!
//! // A normalised diverging stack: the group totals to one.
//! let tops = moved.values().expect("normalize writes new values");
//! assert!((tops[2] - 1.0).abs() < 1e-12);
//! ```
//!
//! ## Determinism
//!
//! [`SVMove::jitter`] takes an optional seed and the swarm layout is
//! deterministic for the same data and order (design D3). Both derive their
//! offset from the mark's key — the group code and the point index — never from
//! a frame index, so an animated strip plot keeps every point in place while
//! its values change (design D4). The generator is implemented in-crate
//! ([`sv_split_mix_64`]) with plain 64-bit integer arithmetic; the crate takes
//! no dependency for it and the result is identical across platforms.
//!
//! ## Data boundary
//!
//! A move consumes [`SVColumnView`](crate::sv_data::SVColumnView) and
//! dispatches on the representation exactly once (spec `data-view-boundary`):
//! a dense column is read as a slice, a sparse column is read over its stored
//! entries without being expanded — an implicit zero keeps the baseline at
//! zero and never advances a stack — and a streaming column is read from the
//! bounded window snapshot it already carries, so a live figure never blocks
//! the render thread.
//!
//! `Stack` and `Normalize` need a group total. Over a stream the total is the
//! total of the window the snapshot carries, which is exactly the data a live
//! render can show; the FDR records that choice.
//!
//! ## Positions are data-space
//!
//! A move produces positions in data units. Mapping them onto the viewport is
//! [`sv_scale`](crate::sv_scale)'s job, and drawing them is the mark's; the move
//! knows nothing about pixels, viewports or backends.

mod dodge;
mod jitter;
mod move_definition;
mod move_span;
mod normalize;
mod options;
mod output;
mod random;
mod request;
mod sequence;
mod shift;
mod stack;
mod stage;
mod swarm;
mod working_state;

pub use move_definition::{SVMove, SVMoveKind};
pub use options::{
    SVDodgeOptions, SVJitterMode, SVJitterOptions, SVNormalisationTarget, SVNormalizeOptions,
    SVShiftDirection, SVShiftOptions, SVStackMode, SVStackOptions, SVSwarmOptions,
};
pub use output::SVMoveOutput;
pub use random::sv_split_mix_64;
pub use request::SVMoveRequest;
pub use sequence::SVMoveSequence;
pub use stage::SVMoveStage;

#[cfg(test)]
#[path = "dodge_tests.rs"]
mod dodge_tests;
#[cfg(test)]
#[path = "jitter_tests.rs"]
mod jitter_tests;
#[cfg(test)]
#[path = "move_concurrency_tests.rs"]
mod move_concurrency_tests;
#[cfg(test)]
#[path = "normalize_tests.rs"]
mod normalize_tests;
#[cfg(test)]
#[path = "option_tests.rs"]
mod option_tests;
#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
#[cfg(test)]
#[path = "sequence_tests.rs"]
mod sequence_tests;
#[cfg(test)]
#[path = "shift_tests.rs"]
mod shift_tests;
#[cfg(test)]
#[path = "stack_tests.rs"]
mod stack_tests;
#[cfg(test)]
#[path = "swarm_tests.rs"]
mod swarm_tests;
