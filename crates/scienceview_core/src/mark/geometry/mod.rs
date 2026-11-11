//! The backend-agnostic primitives a mark produces.

mod point;
mod primitive;

pub use point::SVPoint;
pub use primitive::{
    SVArcTo, SVGeometryCircle, SVGeometryImage, SVGeometryPath, SVGeometryRectangle,
    SVGeometryText, SVMarkGeometry, SVMarkPrimitive, SVMarkShape, SVPathCommand,
};
