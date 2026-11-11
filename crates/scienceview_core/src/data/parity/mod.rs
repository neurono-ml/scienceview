//! The parity harness: one dataset, three representations, one answer.

mod parity_harness;
mod tolerance;

pub use parity_harness::{SVRepresentationParityHarness, sv_assert_representation_parity};
pub use tolerance::SVRepresentationTolerance;
