//! Resolving the columns, the shared domains and the hue groups.
//!
//! This module is a pure dispatcher (patterns.md P3): every helper lives in a
//! named sibling file and is re-exported here.

mod cell_points;
mod domain;
mod groups;
mod probe;
mod value;

pub(crate) use cell_points::{SVPairCellPoints, sv_cell_points};
pub(crate) use domain::{SVPairDomain, sv_variable_domains};
pub(crate) use groups::{SVPairGroups, sv_group_values, sv_hue_groups};
pub(crate) use probe::sv_probe_columns;
