//! Assembling a scale: the builder value and its validation.
//!
//! The folder separates the option surface — one method per setting, no
//! validation — from the validation performed once at build time, so the
//! option surface reads top-down and the rules live in one place.

mod build_scale;
mod scale_builder;

pub use scale_builder::SVScaleBuilder;
