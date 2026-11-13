//! Shared synthetic data for the pure-layer tests.
//!
//! The columns are owned because the Python boundary owns its lists, but they
//! are small fixed vectors: no test reads a file or allocates a large table.

use scienceview::sv_data::SVColumnRole;

use super::data::{SVPythonColumn, SVPythonColumnValues, SVPythonTable};
use super::data_source::SVPythonData;

/// A six-row table with `x`, `y`, `series` and `value` columns.
pub(crate) fn sv_binding_table() -> SVPythonTable {
    SVPythonTable::new(vec![
        SVPythonColumn::dense("x", SVColumnRole::X, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0]),
        SVPythonColumn::dense("y", SVColumnRole::Y, vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]),
        SVPythonColumn::dense(
            "series",
            SVColumnRole::Grouping,
            vec![0.0, 0.0, 1.0, 1.0, 0.0, 1.0],
        ),
        SVPythonColumn::dense("value", SVColumnRole::Y, vec![0.5, 1.5, 2.5, 3.5, 4.5, 5.5]),
    ])
    .expect("the synthetic table is well formed")
}

/// The same four columns, with `y` stored sparsely.
pub(crate) fn sv_sparse_binding_table() -> SVPythonTable {
    SVPythonTable::new(vec![
        SVPythonColumn::dense("x", SVColumnRole::X, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0]),
        SVPythonColumn::new(
            "y",
            SVColumnRole::Y,
            SVPythonColumnValues::Sparse {
                indices: vec![0, 2, 4],
                values: vec![1.0, 2.0, 3.0],
                length: 6,
            },
        ),
        SVPythonColumn::dense(
            "series",
            SVColumnRole::Grouping,
            vec![0.0, 0.0, 1.0, 1.0, 0.0, 1.0],
        ),
        SVPythonColumn::dense("value", SVColumnRole::Y, vec![0.5, 1.5, 2.5, 3.5, 4.5, 5.5]),
    ])
    .expect("the sparse synthetic table is well formed")
}

/// The shared table as binding data.
pub(crate) fn sv_binding_data() -> SVPythonData {
    SVPythonData::Table(sv_binding_table())
}
