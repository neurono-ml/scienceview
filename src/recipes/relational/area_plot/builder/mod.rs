//! The area-plot builder and its backend typestate.
//!
//! This module is a pure dispatcher (patterns.md P3): the builder definition,
//! its option surface, the backend typestate transition, the build-time
//! validation and the validation helpers each live in their own file.

mod backend_selection;
mod build;
mod builder_definition;
mod options;
mod validation;

pub use builder_definition::SVAreaPlotBuilder;
