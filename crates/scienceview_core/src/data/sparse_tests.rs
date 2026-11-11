//! Companion tests for borrowed sparse storage.

use crate::sv_data::SVSparseView;

#[test]
fn a_sparse_view_borrows_its_storage() {
    let indices = [0_usize, 3];
    let values = [1.5_f64, 4.5];
    let sparse = SVSparseView::new(&indices, &values, 6).expect("valid sparse view");

    assert_eq!(sparse.indices(), &indices);
    assert_eq!(sparse.values(), &values);
    assert_eq!(sparse.length(), 6);
    assert_eq!(sparse.number_of_stored_values(), 2);
    assert!(!sparse.is_empty());
    assert!(std::ptr::eq(sparse.values().as_ptr(), values.as_ptr()));
}

#[test]
fn stored_entries_yield_index_value_pairs() {
    let indices = [0_usize, 3, 5];
    let values = [1.0_f64, 4.0, 6.0];
    let sparse = SVSparseView::new(&indices, &values, 6).expect("valid sparse view");

    let entries: Vec<(usize, f64)> = sparse.stored_entries().collect();
    assert_eq!(entries, vec![(0, 1.0), (3, 4.0), (5, 6.0)]);
}

#[test]
fn an_empty_sparse_view_is_valid() {
    let sparse = SVSparseView::new(&[], &[], 0).expect("an empty view is valid");

    assert!(sparse.is_empty());
    assert_eq!(sparse.value_at(0), None);
}
#[test]
fn a_sparse_view_never_materialises_implicit_zeros() {
    let indices = [2_usize];
    let values = [9.0_f64];
    let sparse = SVSparseView::new(&indices, &values, 1_000_000).expect("valid sparse view");

    assert_eq!(sparse.length(), 1_000_000);
    assert_eq!(sparse.number_of_stored_values(), 1);
    assert_eq!(sparse.value_at(999_999), Some(0.0));
}
