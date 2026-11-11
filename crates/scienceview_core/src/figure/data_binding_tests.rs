//! Tests for the data binding and its logical point count.
//!
//! The logical length is never smaller than the values actually present: a
//! caller that supplies fewer points than values is rejected instead of being
//! silently corrected.

use super::SVDataBinding;
use super::SVDataRepresentation;
use crate::sv_error::SVError;

#[test]
fn a_sparse_point_count_smaller_than_the_values_is_rejected() {
    let values = [1.0, 2.0, 3.0];

    let error = SVDataBinding::sparse(&values, 2)
        .map(|_binding| ())
        .expect_err("two points cannot carry three values");

    assert_eq!(
        error,
        SVError::InconsistentPointCount {
            point_count: 2,
            value_count: 3,
        }
    );
}

#[test]
fn a_streaming_point_count_smaller_than_the_values_is_rejected() {
    let values = [1.0, 2.0, 3.0, 4.0];

    let error = SVDataBinding::streaming(&values, 1)
        .map(|_binding| ())
        .expect_err("one point cannot carry four values");

    assert_eq!(
        error,
        SVError::InconsistentPointCount {
            point_count: 1,
            value_count: 4,
        }
    );
}

#[test]
fn a_point_count_equal_to_or_larger_than_the_values_is_accepted() {
    let values = [1.0, 2.0, 3.0];

    let sparse = SVDataBinding::sparse(&values, 3).expect("three points carry three values");
    let streaming = SVDataBinding::streaming(&values, 30).expect("thirty points are a window");

    assert_eq!(sparse.point_count(), 3);
    assert_eq!(sparse.representation(), SVDataRepresentation::Sparse);
    assert_eq!(streaming.point_count(), 30);
    assert_eq!(streaming.representation(), SVDataRepresentation::Streaming);
}

#[test]
fn a_dense_binding_takes_its_point_count_from_the_values() {
    let values = [1.0, 2.0, 3.0, 4.0];

    let binding = SVDataBinding::dense(&values);

    assert_eq!(binding.point_count(), 4);
    assert_eq!(binding.representation(), SVDataRepresentation::Dense);
    assert_eq!(binding.values(), &values);
}
