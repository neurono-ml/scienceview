//! The parity harness every kernel adopts.

mod assertions;
mod harness;

pub use harness::{SVRepresentationParityHarness, sv_assert_representation_parity};

#[cfg(test)]
#[path = "parity_harness_tests.rs"]
mod parity_harness_tests;
