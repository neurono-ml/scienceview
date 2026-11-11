//! Companion tests for the `ndarray` adapter.

use crate::sv_error::SVError;
use ndarray::{Array1, Array2, array};

use crate::sv_data::adapters::SVNdarrayColumn;
use crate::sv_data::{SVColumnRole, SVDataSource, SVRepresentation};

#[test]
fn an_ndarray_view_is_borrowed_without_copying() {
    let array: Array1<f64> = array![1.0, 2.0, 3.0];
    let view = array.view();
    let expected_pointer = view.as_slice().expect("a 1-D array is contiguous").as_ptr();

    let source = SVNdarrayColumn::new("value", view);
    let mut snapshot = source.column("value").expect("the column exists");

    assert_eq!(snapshot.representation(), SVRepresentation::Dense);
    let dense = snapshot.column_view().dense_values().expect("dense");
    assert!(std::ptr::eq(dense.as_ptr(), expected_pointer));
    assert_eq!(dense, &[1.0, 2.0, 3.0]);
}

#[test]
fn an_ndarray_column_declares_its_role() {
    let array: Array1<f64> = array![1.0, 2.0];
    let source = SVNdarrayColumn::new("time", array.view()).with_role(SVColumnRole::X);

    assert_eq!(
        source.schema().column("time").map(|value| value.role()),
        Some(SVColumnRole::X)
    );
}

#[test]
fn a_non_contiguous_view_names_the_alternative_instead_of_copying() {
    let matrix: Array2<f64> = array![[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]];
    let column = matrix.column(0);
    assert!(
        column.as_slice().is_none(),
        "the fixture must be non-contiguous"
    );

    let source = SVNdarrayColumn::new("value", column);
    let error = source
        .column("value")
        .expect_err("a non-contiguous view cannot be borrowed as a slice");

    match error {
        SVError::NonContiguousView {
            origin,
            alternative,
        } => {
            assert_eq!(origin, "ndarray");
            assert!(alternative.contains("to_owned"));
        }
        other => panic!("unexpected error {other:?}"),
    }
}

#[test]
fn a_2d_array_column_can_be_borrowed_when_contiguous() {
    let matrix: Array2<f64> = array![[1.0], [3.0]];
    let column = matrix.column(0);
    assert!(
        column.as_slice().is_some(),
        "a single-column matrix is contiguous"
    );

    let source = SVNdarrayColumn::new("value", column);
    let values = source
        .column("value")
        .expect("the column exists")
        .column_view()
        .dense_values()
        .expect("the column is contiguous")
        .to_vec();

    assert_eq!(values, vec![1.0, 3.0]);
}

#[test]
fn an_unknown_ndarray_column_is_reported() {
    let array: Array1<f64> = array![1.0];
    let source = SVNdarrayColumn::new("value", array.view());

    assert_eq!(
        source.column("other").err(),
        Some(SVError::ColumnNotFound {
            name: "other".to_owned()
        })
    );
}
