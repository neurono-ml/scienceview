//! The marker types of the builder state machine.
//!
//! The data source and the backend selection are both required, so `build`
//! exists only on the state that carries both (patterns.md P2). The order of
//! the two calls is free.

/// The builder has not received a data source yet.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SVWithoutDataSource;

/// The builder carries a data source.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SVWithDataSource;

/// The builder has not received an explicit backend selection yet.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SVWithoutBackend;

/// The builder carries an explicit backend selection.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SVWithBackend;
