//! Linked selection filters: derived views, shared state and cycle bounds.

use crate::sv_control::SVControlRange;
use crate::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema,
    SVSparseView,
};
use crate::sv_error::SVError;
use crate::sv_selection::{SVSelectionConstraint, SVSelectionFilter};

fn sv_filter() -> SVSelectionFilter {
    SVSelectionFilter::new(
        "measurements",
        SVSelectionConstraint::new(
            "x",
            SVControlRange::new(1.0, 3.0).expect("an ordered range"),
        )
        .expect("a column"),
        Some(
            SVSelectionConstraint::new(
                "y",
                SVControlRange::new(0.0, 1.0).expect("an ordered range"),
            )
            .expect("a column"),
        ),
    )
    .expect("the filter is well formed")
}

#[test]
fn a_selection_becomes_a_filter_over_the_shared_source() {
    let x = [0.0, 1.0, 2.0, 4.0];
    let y = [0.5, 0.5, 0.5, 0.5];
    let table = crate::sv_data::SVDataTableBuilder::new()
        .with_column("x", SVColumnRole::X, &x)
        .with_column("y", SVColumnRole::Y, &y)
        .build()
        .expect("the table is valid");

    assert_eq!(
        sv_filter()
            .selected_indices(&table)
            .expect("the columns resolve"),
        vec![1, 2]
    );
}

#[test]
fn filtering_reads_the_stored_values_and_copies_no_underlying_data() {
    let x = [0.0, 1.0, 2.0, 4.0];
    let y = [0.5, 0.5, 0.5, 0.5];
    let table = crate::sv_data::SVDataTableBuilder::new()
        .with_column("x", SVColumnRole::X, &x)
        .with_column("y", SVColumnRole::Y, &y)
        .build()
        .expect("the table is valid");
    let stored = table.values("x").expect("the column exists").as_ptr();

    let selected = sv_filter()
        .selected_indices(&table)
        .expect("the columns resolve");

    assert_eq!(selected.len(), 2);
    assert_eq!(
        table.values("x").expect("the column exists").as_ptr(),
        stored
    );
}

#[test]
fn a_sparse_source_is_filtered_without_densifying_it() {
    let sparse = SVSparseSource::new();
    let filter = SVSelectionFilter::new(
        "sparse",
        SVSelectionConstraint::new(
            "x",
            SVControlRange::new(1.0, 2.0).expect("an ordered range"),
        )
        .expect("a column"),
        None,
    )
    .expect("the filter is well formed");

    assert_eq!(
        filter
            .selected_indices(&sparse)
            .expect("the sparse view resolves"),
        vec![0]
    );
    assert_eq!(sparse.resolutions(), 1);
}

#[test]
fn a_filter_with_an_empty_column_name_is_rejected() {
    let error = SVSelectionConstraint::new("", SVControlRange::unit())
        .expect_err("a column name cannot be empty");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

/// A sparse source that records how many times it resolved its column.
struct SVSparseSource {
    resolutions: std::sync::atomic::AtomicUsize,
}

impl SVSparseSource {
    fn new() -> Self {
        Self {
            resolutions: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    fn resolutions(&self) -> usize {
        self.resolutions.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl SVDataSource for SVSparseSource {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::owned(vec![SVColumnDescriptor::new("x", SVColumnRole::X)])
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        const INDICES: [usize; 2] = [0, 2];
        const VALUES: [f64; 2] = [1.5, 4.0];
        if name != "x" {
            return Err(SVError::ColumnNotFound {
                name: name.to_owned(),
            });
        }
        self.resolutions
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let sparse = SVSparseView::new(&INDICES, &VALUES, 4)?;
        Ok(SVColumnSnapshot::Borrowed(SVColumnView::Sparse(sparse)))
    }
}
