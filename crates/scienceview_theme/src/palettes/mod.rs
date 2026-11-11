//! The palette model: discrete, continuous and data-derived.
//!
//! The three are distinct types because their animation rule differs: a
//! nominal colour assigned by a discrete palette stays stable when data
//! arrives, while a continuous colour interpolates and a derived palette
//! recomputes from the data.

mod continuous;
mod core_implementation;
mod derived;
mod discrete;

pub use continuous::{SVColourMapKind, SVContinuousPalette};
pub use core_implementation::SVColourPalettes;
pub use derived::{SVDerivationRule, SVDerivedPalette};
pub use discrete::SVDiscretePalette;

#[cfg(test)]
mod palettes_tests;
