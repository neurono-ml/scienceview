//! Tick derivation: the hook the layout engine consumes to draw an axis.
//!
//! The scale owns the mathematics of the ticks; the layout engine owns the
//! count. [`SVTickSpace`] carries the length the axis was granted and the
//! minimum spacing a label needs, and [`SVTickDerivation`] turns that budget
//! into values, positions and labels.

mod derivation;
mod format;
mod space;
mod temporal_ticks;
mod tick;

pub use derivation::SVTickDerivation;
pub use format::SVTickFormat;
pub use space::SVTickSpace;
pub use tick::SVTick;
