//! A binned density with contour lines inside one pair cell.
//!
//! This module is a pure dispatcher (patterns.md P3): the joint grid, the
//! contour geometry and the cell composition each live in a named sibling file.

mod cell;
mod geometry;
mod grid;

pub(crate) use cell::sv_contour_cell;
