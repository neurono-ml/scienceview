//! Resolving the documented columns into the curves the composition draws.

mod definition;
mod kernel;
mod representation;
mod resolve;

pub(crate) use definition::{SVEcdfCurve, SVEcdfData};
pub(crate) use resolve::sv_resolve_ecdf_data;
