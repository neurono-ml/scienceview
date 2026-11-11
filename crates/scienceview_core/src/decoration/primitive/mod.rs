//! Backend-agnostic decoration primitives.

mod primitive_definition;

pub use primitive_definition::{
    SVDecorationPrimitive, SVPoint, SVRectanglePrimitive, SVStroke, SVTextAnchor,
};

#[cfg(test)]
#[path = "primitive_tests.rs"]
mod primitive_tests;
