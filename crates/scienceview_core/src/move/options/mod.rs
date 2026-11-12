//! The typed option surface of every move.
//!
//! Each option struct follows the mark options' shape: [`Default`] carries the
//! documented defaults, a consuming setter changes one option, and a `_value`
//! accessor reads it back. Validation happens once, in
//! [`SVMove::validate`](crate::sv_move::SVMove::validate), so a sequence
//! rejects an impossible option before any data is read.

mod dodge;
mod jitter;
mod normalize;
mod shift;
mod stack;
mod swarm;

pub use dodge::SVDodgeOptions;
pub use jitter::{SVJitterMode, SVJitterOptions};
pub use normalize::{SVNormalisationTarget, SVNormalizeOptions};
pub use shift::{SVShiftDirection, SVShiftOptions};
pub use stack::{SVStackMode, SVStackOptions};
pub use swarm::SVSwarmOptions;
