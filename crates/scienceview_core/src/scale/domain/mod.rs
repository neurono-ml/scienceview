//! The data interval a scale covers: its definition, derivation and rounding.
//!
//! The folder separates the three concerns: the value type and its provenance,
//! the derivation from a borrowed column view (one dispatch per representation)
//! and the "nice" rounding a derived domain receives before it becomes an axis.

mod definition;
mod derivation;
mod nice;

pub use definition::{SVDomain, SVDomainSource};
pub use nice::sv_nice_domain;
pub(crate) use nice::sv_nice_step;
