//! Chart recipes: one module per chart, grouped by the family it belongs to.
//!
//! A recipe composes the core kernels — data boundary, stats, moves, scales,
//! marks and decorations — into the chart a reader names. It never reimplements
//! a kernel and never adds a variant to a closed enumeration (patterns.md P5).
//! Registration happens in the family module, never in a central list (P4), so
//! two recipes of different families never conflict.

pub mod relational;
