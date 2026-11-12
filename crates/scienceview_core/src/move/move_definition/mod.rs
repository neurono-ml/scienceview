//! The closed move vocabulary: its kind, its enumeration, its validation and
//! its conversions.
//!
//! This module is a pure dispatcher. The kind lives in [`kind`], the closed
//! enumeration and its stage mapping in [`move_enum`], the option validation in
//! [`validation`] and the option conversions in [`conversions`].

mod conversions;
mod kind;
mod move_enum;
mod validation;

pub use kind::SVMoveKind;
pub use move_enum::SVMove;
