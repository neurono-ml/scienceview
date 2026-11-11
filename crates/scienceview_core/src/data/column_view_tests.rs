//! Companion tests for the three representations and their borrowing rules.

use std::cell::Cell;

use crate::sv_data::{SVColumnView, SVDataError, SVRepresentation, SVSparseView, SVStreamingView};

const DENSE_VALUES: [f64; 5] = [1.0, 0.0, 3.0, 0.0, 5.0];
const SPARSE_INDICES: [usize; 3] = [0, 2, 4];
const SPARSE_VALUES: [f64; 3] = [1.0, 3.0, 5.0];

fn sparse_view<'a>() -> SVSparseView<'a> {
    SVSparseView::new(&SPARSE_INDICES, &SPARSE_VALUES, DENSE_VALUES.len())
        .expect("the fixture is a valid sparse view")
}

#[test]
fn dense_view_borrows_the_slice_without_copying() {
    let values = [2.0_f64, 4.0, 8.0];
    let view = SVColumnView::Dense(&values);

    assert_eq!(view.representation(), SVRepresentation::Dense);
    assert_eq!(view.length(), 3);
    assert_eq!(view.dense_values().expect("dense is contiguous"), &values);
    assert!(std::ptr::eq(
        view.dense_values().expect("dense is contiguous").as_ptr(),
        values.as_ptr()
    ));
}

#[test]
fn sparse_view_borrows_indices_and_values() {
    let indices = [1_usize, 3];
    let values = [10.0_f64, 30.0];
    let sparse = SVSparseView::new(&indices, &values, 4).expect("valid sparse view");
    let view = SVColumnView::Sparse(sparse);

    assert_eq!(view.representation(), SVRepresentation::Sparse);
    assert_eq!(view.length(), 4);
    assert!(std::ptr::eq(
        view.sparse_view().expect("sparse view").indices().as_ptr(),
        indices.as_ptr()
    ));
}

#[test]
fn streaming_view_borrows_the_window_snapshot() {
    let snapshot = [1.0_f64, 2.0, 3.0];
    let streaming = SVStreamingView::new(&snapshot, 7, 4);
    let view = SVColumnView::Streaming(streaming);

    assert_eq!(view.representation(), SVRepresentation::Streaming);
    assert_eq!(view.length(), 3);
    assert_eq!(streaming.sequence(), 7);
    assert_eq!(streaming.dropped_samples(), 4);
    assert!(std::ptr::eq(streaming.values().as_ptr(), snapshot.as_ptr()));
}

#[test]
fn dense_values_rejects_a_sparse_view_instead_of_densifying() {
    let view = SVColumnView::Sparse(sparse_view());

    match view.dense_values() {
        Err(SVDataError::UnsupportedRepresentation {
            operation,
            representation,
            alternative,
        }) => {
            assert_eq!(operation, "dense_values");
            assert_eq!(representation, SVRepresentation::Sparse);
            assert!(alternative.contains("stored_entries"));
        }
        other => panic!("a sparse view must not be densified, received {other:?}"),
    }
}

#[test]
fn dense_and_streaming_views_share_the_same_values() {
    let streamed = SVStreamingView::new(&DENSE_VALUES, 5, 0);

    assert_eq!(
        SVColumnView::Dense(&DENSE_VALUES).dense_values().unwrap(),
        SVColumnView::Streaming(streamed).dense_values().unwrap()
    );
}

#[test]
fn sparse_value_at_answers_implicit_zeros_and_out_of_range_positions() {
    let sparse = sparse_view();

    assert_eq!(sparse.value_at(1), Some(0.0));
    assert_eq!(sparse.value_at(2), Some(3.0));
    assert_eq!(sparse.value_at(5), None);
    assert_eq!(sparse.number_of_stored_values(), 3);
}

#[test]
fn sparse_view_rejects_an_out_of_range_index() {
    let indices = [0_usize, 9];
    let values = [1.0_f64, 2.0];

    assert_eq!(
        SVSparseView::new(&indices, &values, 5),
        Err(SVDataError::SparseIndexOutOfRange {
            index: 9,
            length: 5
        })
    );
}

#[test]
fn sparse_view_rejects_indices_that_do_not_increase() {
    let indices = [0_usize, 1, 1];
    let values = [1.0_f64, 2.0, 3.0];

    assert_eq!(
        SVSparseView::new(&indices, &values, 4),
        Err(SVDataError::SparseIndicesNotIncreasing { position: 2 })
    );
}

#[test]
fn sparse_view_rejects_mismatched_lengths() {
    let indices = [0_usize, 2];
    let values = [1.0_f64];

    assert_eq!(
        SVSparseView::new(&indices, &values, 4),
        Err(SVDataError::MismatchedSparseLengths {
            indices: 2,
            values: 1
        })
    );
}

#[test]
fn dispatch_runs_exactly_one_branch_once() {
    let dense_calls = Cell::new(0);
    let sparse_calls = Cell::new(0);
    let streaming_calls = Cell::new(0);

    let view = SVColumnView::Sparse(sparse_view());
    let length = view.dispatch(
        |values| {
            dense_calls.set(dense_calls.get() + 1);
            values.len()
        },
        |sparse| {
            sparse_calls.set(sparse_calls.get() + 1);
            sparse.length()
        },
        |streaming| {
            streaming_calls.set(streaming_calls.get() + 1);
            streaming.length()
        },
    );

    assert_eq!(length, 5);
    assert_eq!(dense_calls.get(), 0);
    assert_eq!(sparse_calls.get(), 1);
    assert_eq!(streaming_calls.get(), 0);
}

#[test]
fn representations_are_equivalent_through_value_at() {
    let dense = SVColumnView::Dense(&DENSE_VALUES);
    let sparse = SVColumnView::Sparse(sparse_view());
    let streaming = SVColumnView::Streaming(SVStreamingView::new(&DENSE_VALUES, 5, 0));

    for index in 0..DENSE_VALUES.len() {
        let expected = dense.value_at(index);
        assert_eq!(sparse.value_at(index), expected);
        assert_eq!(streaming.value_at(index), expected);
    }
}
