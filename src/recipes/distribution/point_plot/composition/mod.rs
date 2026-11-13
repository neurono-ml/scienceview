//! Composition of the dot, error-bar and line geometries.

mod lines;
mod orchestration;
mod ownership;
mod points;

pub(crate) use orchestration::sv_compose_geometries;
