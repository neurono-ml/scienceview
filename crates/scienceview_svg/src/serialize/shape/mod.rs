//! Mark shapes become SVG elements, with large circle runs grouped.

mod elements;
mod grouping;
mod path;

pub use elements::sv_serialize_mark_primitive;

pub(crate) use grouping::sv_serialize_mark_geometry;
