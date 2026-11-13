//! Resolving the documented columns into observations.
//!
//! The value column decides which logical positions are observations: a dense
//! column contributes every row, a sparse column only its stored entries — the
//! implicit zeros are never expanded — and a streaming column its current
//! window. The category and series columns are read at those same positions.
//!
//! This module is a pure dispatcher (patterns.md P3).

mod observation;
mod validation;

pub(crate) use observation::{SVViolinData, SVViolinObservation, sv_resolve_violin_data};
pub(crate) use validation::{sv_category_label, sv_series_label, sv_validate_columns};
