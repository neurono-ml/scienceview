//! Companion tests for the non-densifying observation reader.

use crate::sv_data::{SVColumnView, SVSparseView};
use crate::sv_stat::observations::SVObservationValues;

#[test]
fn a_dense_column_is_read_as_contiguous_storage() {
    let values = [3.0, 1.0, 2.0];
    let observations = SVObservationValues::from_view(SVColumnView::Dense(&values));

    assert_eq!(observations.length(), 3);
    assert_eq!(observations.implicit_zero_count(), 0);
    assert_eq!(observations.minimum_and_maximum(), Some((1.0, 3.0)));
    assert_eq!(observations.first_implicit_zero_position(), None);
}

#[test]
fn a_sparse_column_reports_its_implicit_zeros_without_expanding() {
    let indices = [1usize, 4];
    let values = [2.0, 3.0];
    let sparse = SVSparseView::new(&indices, &values, 6).expect("valid sparse storage");
    let observations = SVObservationValues::from_view(SVColumnView::Sparse(sparse));

    assert_eq!(observations.length(), 6);
    assert_eq!(observations.implicit_zero_count(), 4);
    assert_eq!(observations.minimum_and_maximum(), Some((0.0, 3.0)));
    assert_eq!(observations.first_implicit_zero_position(), Some(0));
}

#[test]
fn the_first_appearance_order_injects_the_implicit_zero_once() {
    let indices = [1usize, 3];
    let values = [2.0, 3.0];
    let sparse = SVSparseView::new(&indices, &values, 5).expect("valid sparse storage");
    let observations = SVObservationValues::from_view(SVColumnView::Sparse(sparse));
    let mut visited = Vec::new();

    observations.for_each_first_appearance(|value, multiplicity| {
        visited.push((value, multiplicity));
    });

    assert_eq!(visited, vec![(0.0, 3), (2.0, 1), (3.0, 1)]);
}

#[test]
fn a_stored_zero_is_not_counted_twice() {
    let indices = [0usize, 2];
    let values = [0.0, 3.0];
    let sparse = SVSparseView::new(&indices, &values, 4).expect("valid sparse storage");
    let observations = SVObservationValues::from_view(SVColumnView::Sparse(sparse));

    assert_eq!(observations.implicit_zero_count(), 2);
    assert_eq!(observations.first_implicit_zero_position(), Some(1));
}

#[test]
fn a_non_finite_stored_value_is_reported_at_its_logical_position() {
    let indices = [1usize, 4];
    let values = [2.0, f64::NAN];
    let sparse = SVSparseView::new(&indices, &values, 6).expect("valid sparse storage");
    let observations = SVObservationValues::from_view(SVColumnView::Sparse(sparse));

    let error = observations
        .validate_finite("bin")
        .expect_err("a non-finite value is not an observation");

    assert_eq!(
        error,
        crate::sv_error::SVError::NonFiniteValue {
            operation: "bin",
            position: 4,
        }
    );
}
