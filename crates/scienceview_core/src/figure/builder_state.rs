//! The builder's typestate markers.
//!
//! The states encode the two required steps — a data source and a backend — as
//! type parameters. `build` is only implemented for the fully configured pair,
//! so a figure without either is a compile error rather than a runtime error
//! (spec `backend-selection`, design D2).

/// The builder has no data source yet.
#[derive(Debug)]
pub struct SVWithoutDataSource;

/// The builder already carries a data source.
#[derive(Debug)]
pub struct SVWithDataSource;

/// The builder has no backend selection yet.
#[derive(Debug)]
pub struct SVWithoutBackend;

/// The builder already carries a backend selection.
#[derive(Debug)]
pub struct SVWithBackend;
