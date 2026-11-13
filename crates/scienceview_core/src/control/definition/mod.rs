//! `SVControl`: a configured model object that derives a semantic action.

mod action;
mod control;
mod mutators;
mod step;

pub use control::SVControl;
pub(crate) use control::SVControlConfiguration;
pub(crate) use step::sv_validated_step;
