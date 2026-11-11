//! Scales: the only place where a data value becomes a visual property.
//!
//! A scale owns the data domain and the display range together (design D1), so
//! a moving domain reprojects every value through the same object and an axis
//! can never disagree with the marks on it. The model is closed (patterns P5):
//! [`SVScaleKind`] has exactly continuous, nominal, temporal and boolean
//! variants, and [`SVScaleValue`] mirrors them, so a kernel dispatches
//! exhaustively and a new kind is a change to the scale boundary.
//!
//! ## Builder
//!
//! The direct constructors are private; [`SVScaleBuilder`] is the only public
//! way to assemble a scale. The kind is fixed at construction and every other
//! option is validated at `build`, so an option that does not apply to the
//! chosen kind fails with [`SVError::InvalidConfiguration`](crate::sv_error::SVError::InvalidConfiguration)
//! instead of being ignored.
//!
//! ```
//! use scienceview_core::sv_scale::{SVScaleBuilder, SVScaleKind, SVScaleValue};
//!
//! let scale = SVScaleBuilder::new(SVScaleKind::Continuous)
//!     .domain(0.0, 10.0)
//!     .range(0.0, 100.0)
//!     .build()
//!     .expect("the configuration is complete");
//!
//! assert_eq!(scale.position(SVScaleValue::Numeric(5.0)), Ok(50.0));
//! ```
//!
//! ## Data boundary
//!
//! A numeric scale accepts the [`SVColumnView`](crate::sv_data::SVColumnView)
//! views of the data boundary and dispatches on the representation exactly once
//! (spec `data-view-boundary`): a dense column is scanned as a slice, a sparse
//! column is scanned over its stored entries without ever being densified, and
//! a streaming column is read from the bounded snapshot it already carries, so
//! domain derivation never blocks the render thread. Nominal and boolean scales
//! do not consume the numeric column view: their domain is their own label or
//! slot model.

mod boolean;
mod builder;
mod continuous;
mod domain;
mod kind;
mod nominal;
mod range;
mod scale_definition;
mod temporal;

pub use boolean::SVBooleanScale;
pub use builder::SVScaleBuilder;
pub use continuous::SVContinuousScale;
pub use domain::{SVDomain, SVDomainSource};
pub use kind::{SVScaleKind, SVScaleValue};
pub use nominal::{SVCategoryOrder, SVNominalScale};
pub use range::SVScaleRange;
pub use scale_definition::SVScale;
pub use temporal::{SVTemporalScale, SVTemporalUnit};

#[cfg(test)]
#[path = "kind_tests.rs"]
mod kind_tests;
