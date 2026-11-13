//! Turning the data source into the data-space areas the marks draw.
//!
//! This module is a pure dispatcher (patterns.md P3): the representation-aware
//! column accessor, the prepared-area types, the main preparation flow and the
//! stack composition each live in their own file.

mod column_access;
mod flow;
mod prepared_area;
mod stacking;

pub(crate) use flow::sv_prepare_area;
pub(crate) use prepared_area::{SVPreparedArea, SVPreparedSegment};
