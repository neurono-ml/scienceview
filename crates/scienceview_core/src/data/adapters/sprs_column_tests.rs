//! Companion tests for the `sprs` sparse adapter.

use sprs::CsVec;

use crate::data::adapters::SVSparseColumn;
use crate::data::{SVColumnRole, SVDataError, SVDataSource, SVRepresentation};

#[test]
fn a_sparse_vector_is_presented_without_being_densified() {
    let vector: CsVec<f64> = CsVec::new(6, vec![0, 3], vec![1.0, 4.0]);
    let source = SVSparseColumn::new("value", vector.view());

    let mut snapshot = source.column("value").expect("the column exists");
    assert_eq!(snapshot.representation(), SVRepresentation::Sparse);

    let view = snapshot.column_view();
    let sparse = view.sparse_view().expect("the view is sparse");
    assert_eq!(sparse.length(), 6);
    assert_eq!(sparse.number_of_stored_values(), 2);
    assert_eq!(sparse.value_at(1), Some(0.0));
    assert_eq!(sparse.value_at(3), Some(4.0));
}

#[test]
fn dense_values_rejects_the_sparse_vector_instead_of_expanding_it() {
    let vector: CsVec<f64> = CsVec::new(6, vec![1], vec![2.0]);
    let source = SVSparseColumn::new("value", vector.view());
    let mut snapshot = source.column("value").expect("the column exists");

    let error = snapshot
        .column_view()
        .dense_values()
        .expect_err("a sparse view is never densified");

    assert!(matches!(
        error,
        SVDataError::UnsupportedRepresentation {
            representation: SVRepresentation::Sparse,
            ..
        }
    ));
}

#[test]
fn a_sparse_column_declares_its_role() {
    let vector: CsVec<f64> = CsVec::new(3, vec![2], vec![7.0]);
    let source = SVSparseColumn::new("weight", vector.view()).with_role(SVColumnRole::Weight);

    assert_eq!(
        source.schema().column("weight").map(|value| value.role()),
        Some(SVColumnRole::Weight)
    );
}
#[test]
fn an_unknown_sparse_column_is_reported() {
    let vector: CsVec<f64> = CsVec::new(3, vec![2], vec![7.0]);
    let source = SVSparseColumn::new("weight", vector.view());

    assert_eq!(
        source.column("value").err(),
        Some(SVDataError::ColumnNotFound {
            name: "value".to_owned()
        })
    );
}
