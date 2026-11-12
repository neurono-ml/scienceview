//! Deterministic text, number, paint and primitive serialization.

mod decoration;
mod escape;
mod number;
mod paint;
mod shape;

pub use escape::sv_escape_text;
pub use number::sv_format_number;
pub use shape::sv_serialize_mark_primitive;

pub(crate) use decoration::sv_serialize_decoration_primitive;
pub(crate) use paint::sv_mark_paint_attributes;
pub(crate) use shape::sv_serialize_mark_geometry;

#[cfg(test)]
mod number_tests;
#[cfg(test)]
mod serialize_tests;
