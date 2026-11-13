//! Turning the data source into the density estimates the marks draw.
//!
//! This module is a pure dispatcher (patterns.md P3): the representation-aware
//! group extraction and the prepared-density types each live in their own file.

mod flow;
mod group_values;
mod prepared_density;

pub(crate) use flow::sv_prepare_density;
pub(crate) use prepared_density::{SVPreparedDensity, SVPreparedDensitySeries};
