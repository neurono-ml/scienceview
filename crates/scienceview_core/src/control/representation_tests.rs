//! Axis registration consumes dense, sparse and streaming views identically.

use crate::sv_control::{SVControlAxis, SVControlState};
use crate::sv_data::{SVColumnView, SVSparseView, SVStreamingView};
use crate::sv_scale::SVDomain;

#[test]
fn the_three_representations_register_the_same_axis_bounds() {
    let dense_values = [-2.0, 0.0, 3.5];
    let sparse_indices = [0_usize, 2];
    let sparse_values = [-2.0, 3.5];

    let mut dense = SVControlState::new();
    dense
        .register_axis_from_column(
            SVControlAxis::Horizontal,
            SVColumnView::Dense(&dense_values),
        )
        .expect("the dense column has finite values");

    let sparse =
        SVSparseView::new(&sparse_indices, &sparse_values, 3).expect("a valid sparse view");
    let mut from_sparse = SVControlState::new();
    from_sparse
        .register_axis_from_column(SVControlAxis::Horizontal, SVColumnView::Sparse(sparse))
        .expect("the sparse column has finite values");

    let streaming = SVStreamingView::new(&dense_values, 7, 2);
    let mut from_stream = SVControlState::new();
    from_stream
        .register_axis_from_column(
            SVControlAxis::Horizontal,
            SVColumnView::Streaming(streaming),
        )
        .expect("the streaming window has finite values");

    let expected = Some(SVDomain::new(-2.0, 3.5).expect("ordered domain"));
    assert_eq!(dense.axis_domain(SVControlAxis::Horizontal), expected);
    assert_eq!(from_sparse.axis_domain(SVControlAxis::Horizontal), expected);
    assert_eq!(from_stream.axis_domain(SVControlAxis::Horizontal), expected);
}

#[test]
fn a_column_without_finite_values_is_rejected() {
    let values = [f64::NAN, f64::INFINITY];
    let mut state = SVControlState::new();

    assert!(matches!(
        state.register_axis_from_column(SVControlAxis::Vertical, SVColumnView::Dense(&values)),
        Err(crate::sv_error::SVError::InvalidConfiguration { .. })
    ));
}

#[test]
fn a_large_dense_column_registers_once() {
    let values: Vec<f64> = (0..10_000).map(|value| value as f64).collect();
    let mut state = SVControlState::new();

    state
        .register_axis_from_column(SVControlAxis::Horizontal, SVColumnView::Dense(&values))
        .expect("the column is finite");

    assert_eq!(
        state.axis_domain(SVControlAxis::Horizontal),
        Some(SVDomain::new(0.0, 9_999.0).expect("ordered domain"))
    );
}

#[test]
fn a_sparse_column_is_never_densified() {
    let indices = [1_usize];
    let values = [9.0];
    let sparse = SVSparseView::new(&indices, &values, 100).expect("a valid sparse view");

    assert_eq!(sparse.number_of_stored_values(), 1);
    assert_eq!(sparse.length(), 100);
}
