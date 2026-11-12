//! Representation parity: the same data through dense, sparse and streaming.

use crate::sv_data::{SVColumnView, SVSparseView, SVStreamingView};
use crate::sv_move::{SVMove, SVMoveColumn, SVMoveOutput, SVMoveRequest, SVMoveSequence};

/// The logical data every representation carries: one implicit zero in the
/// middle, which a sparse view does not store.
const DENSE_VALUES: [f64; 5] = [2.0, 0.0, 3.0, -1.0, 4.0];
const SPARSE_INDICES: [usize; 4] = [0, 2, 3, 4];
const SPARSE_VALUES: [f64; 4] = [2.0, 3.0, -1.0, 4.0];
const GROUP_KEYS: [u32; 5] = [0, 0, 0, 0, 0];
const CATEGORY_KEYS: [u32; 5] = [0, 0, 0, 0, 0];

/// Applies a sequence to one request.
fn sv_apply(request: &SVMoveRequest<'_>, moves: Vec<SVMove>) -> SVMoveOutput {
    SVMoveSequence::new(moves)
        .expect("the sequence is ordered")
        .apply(request)
        .expect("the sequence applies")
}

/// Builds the three requests for the same logical column.
fn sv_requests<'a>(
    sparse: &'a SVSparseView<'a>,
    streaming: &'a SVStreamingView<'a>,
) -> [SVMoveRequest<'a>; 3] {
    [
        SVMoveRequest::new(SVColumnView::Dense(&DENSE_VALUES), &GROUP_KEYS)
            .with_category_keys(&CATEGORY_KEYS),
        SVMoveRequest::new(SVColumnView::Sparse(*sparse), &GROUP_KEYS)
            .with_category_keys(&CATEGORY_KEYS),
        SVMoveRequest::new(SVColumnView::Streaming(*streaming), &GROUP_KEYS)
            .with_category_keys(&CATEGORY_KEYS),
    ]
}

/// Asserts that two columns carry the same logical values, position by
/// position. A dense and a sparse column describe the same data even though
/// their storage differs, so the comparison is logical, never structural.
fn assert_same_column(actual: &SVMoveColumn, expected: &SVMoveColumn) {
    assert_eq!(actual.length(), expected.length());
    for index in 0..expected.length() {
        assert_eq!(
            actual.value_at(index),
            expected.value_at(index),
            "position {index} differs"
        );
    }
}

/// Asserts that three outputs carry the same displacement.
fn assert_same_displacement(outputs: &[SVMoveOutput; 3]) {
    for output in &outputs[1..] {
        assert_same_column(output.horizontal_offsets(), outputs[0].horizontal_offsets());
        assert_same_column(output.vertical_offsets(), outputs[0].vertical_offsets());
        match (output.values(), outputs[0].values()) {
            (Some(actual), Some(expected)) => assert_same_column(actual, expected),
            (None, None) => {}
            _ => panic!("only one representation produced values"),
        }
        match (output.baselines(), outputs[0].baselines()) {
            (Some(actual), Some(expected)) => assert_same_column(actual, expected),
            (None, None) => {}
            _ => panic!("only one representation produced baselines"),
        }
        match (output.widths(), outputs[0].widths()) {
            (Some(actual), Some(expected)) => assert_same_column(actual, expected),
            (None, None) => {}
            _ => panic!("only one representation produced widths"),
        }
    }
}

/// Asserts that two columns carry the same logical values at `positions`.
fn assert_same_column_at(actual: &SVMoveColumn, expected: &SVMoveColumn, positions: &[usize]) {
    for index in positions.iter().copied() {
        assert_eq!(
            actual.value_at(index),
            expected.value_at(index),
            "position {index} differs"
        );
    }
}

/// Asserts that three outputs carry the same displacement at the positions the
/// sparse view actually stores.
///
/// A per-mark move — jitter and dodge — displaces the marks a sparse column
/// stores; an unstored position is not a mark of that column, so it keeps the
/// implicit zero instead of receiving a displacement of its own. Comparing the
/// stored positions is therefore the honest parity claim for those moves, and
/// a dense column that stores a zero there is a different logical column.
fn assert_same_displacement_at_stored_positions(outputs: &[SVMoveOutput; 3]) {
    for output in &outputs[1..] {
        assert_same_column_at(
            output.horizontal_offsets(),
            outputs[0].horizontal_offsets(),
            &SPARSE_INDICES,
        );
        assert_same_column_at(
            output.vertical_offsets(),
            outputs[0].vertical_offsets(),
            &SPARSE_INDICES,
        );
        match (output.values(), outputs[0].values()) {
            (Some(actual), Some(expected)) => {
                assert_same_column_at(actual, expected, &SPARSE_INDICES)
            }
            (None, None) => {}
            _ => panic!("only one representation produced values"),
        }
        match (output.baselines(), outputs[0].baselines()) {
            (Some(actual), Some(expected)) => {
                assert_same_column_at(actual, expected, &SPARSE_INDICES)
            }
            (None, None) => {}
            _ => panic!("only one representation produced baselines"),
        }
        match (output.widths(), outputs[0].widths()) {
            (Some(actual), Some(expected)) => {
                assert_same_column_at(actual, expected, &SPARSE_INDICES)
            }
            (None, None) => {}
            _ => panic!("only one representation produced widths"),
        }
    }
}

/// Runs one sequence through the three representations and compares them.
fn assert_parity(moves: Vec<SVMove>) {
    let outputs = sv_outputs(moves);
    assert_same_displacement(&outputs);
    assert_representations(&outputs);
}

/// Runs one sequence through the three representations and compares the
/// positions the sparse view stores.
fn assert_stored_parity(moves: Vec<SVMove>) {
    let outputs = sv_outputs(moves);
    assert_same_displacement_at_stored_positions(&outputs);
    assert_representations(&outputs);
    assert_eq!(
        outputs[1].horizontal_offset(1),
        Some(0.0),
        "an unstored position is not a mark and keeps the implicit zero offset"
    );
}

/// Runs one sequence through the three representations.
fn sv_outputs(moves: Vec<SVMove>) -> [SVMoveOutput; 3] {
    let sparse = SVSparseView::new(&SPARSE_INDICES, &SPARSE_VALUES, DENSE_VALUES.len())
        .expect("the sparse view is valid");
    let streaming = SVStreamingView::new(&DENSE_VALUES, 1, 0);
    let requests = sv_requests(&sparse, &streaming);

    [
        sv_apply(&requests[0], moves.clone()),
        sv_apply(&requests[1], moves.clone()),
        sv_apply(&requests[2], moves),
    ]
}

/// Asserts the three outputs report the expected representation.
fn assert_representations(outputs: &[SVMoveOutput; 3]) {
    assert_eq!(
        outputs[0].representation(),
        crate::sv_data::SVRepresentation::Dense
    );
    assert_eq!(
        outputs[1].representation(),
        crate::sv_data::SVRepresentation::Sparse
    );
    assert_eq!(
        outputs[2].representation(),
        crate::sv_data::SVRepresentation::Streaming
    );
}

#[test]
fn stack_has_representation_parity() {
    assert_parity(vec![SVMove::stack().into()]);
}

#[test]
fn dodge_has_representation_parity() {
    assert_stored_parity(vec![SVMove::dodge().into()]);
}

#[test]
fn jitter_has_representation_parity() {
    assert_stored_parity(vec![SVMove::jitter().seed(8).into()]);
}

#[test]
fn shift_has_representation_parity() {
    assert_parity(vec![SVMove::shift().amount(0.5).into()]);
}

#[test]
fn swarm_has_representation_parity() {
    assert_parity(vec![SVMove::swarm().marker_extent(0.5).into()]);
}

#[test]
fn a_near_zero_swarm_cluster_has_representation_parity() {
    // The zero is implicit in the sparse view, but the dense twin lays it out
    // at the middle of the cluster: it occupies the zero slot and pushes the
    // stored marks aside. The sparse layout must reserve that same slot.
    let dense_values = [0.4, 0.0, 0.6];
    let sparse_indices = [0_usize, 2];
    let sparse_values = [0.4, 0.6];
    let group_keys = [0_u32; 3];

    let dense_request = SVMoveRequest::new(SVColumnView::Dense(&dense_values), &group_keys);
    let sparse = SVSparseView::new(&sparse_indices, &sparse_values, dense_values.len())
        .expect("the sparse view is valid");
    let sparse_request = SVMoveRequest::new(SVColumnView::Sparse(sparse), &group_keys);
    let sequence = SVMoveSequence::single(SVMove::swarm().marker_extent(1.0).into())
        .expect("a valid sequence");

    let dense = sequence
        .apply(&dense_request)
        .expect("the dense swarm applies");
    let sparse = sequence
        .apply(&sparse_request)
        .expect("the sparse swarm applies");

    assert_eq!(
        dense.horizontal_offset(0),
        Some(1.0),
        "the 0.4 mark moves off the zero slot"
    );
    assert_eq!(
        dense.horizontal_offset(1),
        Some(0.0),
        "the zero keeps slot 0"
    );
    assert_eq!(
        dense.horizontal_offset(2),
        Some(-1.0),
        "the 0.6 mark takes the other side"
    );
    for index in 0..dense_values.len() {
        assert_eq!(
            sparse.horizontal_offset(index),
            dense.horizontal_offset(index),
            "the sparse layout differs from the dense twin at position {index}"
        );
    }
}

#[test]
fn normalize_has_representation_parity() {
    assert_parity(vec![SVMove::normalize().into()]);
}

#[test]
fn the_full_pipeline_has_representation_parity() {
    assert_parity(vec![
        SVMove::stack().into(),
        SVMove::shift().amount(0.1).into(),
        SVMove::normalize().into(),
    ]);
}

#[test]
fn a_sparse_stack_preserves_the_implicit_zero() {
    let sparse = SVSparseView::new(&SPARSE_INDICES, &SPARSE_VALUES, DENSE_VALUES.len())
        .expect("the sparse view is valid");
    let request = SVMoveRequest::new(SVColumnView::Sparse(sparse), &GROUP_KEYS);
    let moved = sv_apply(&request, vec![SVMove::stack().into()]);

    assert_eq!(moved.value_at(1), Some(0.0));
    assert_eq!(moved.baseline_at(1), 0.0);
    assert_eq!(moved.number_of_points(), DENSE_VALUES.len());
}

#[test]
fn a_streaming_stack_uses_the_window_total() {
    // A window snapshot is the data a live figure can show, so the sum of the
    // window is the total of the stack.
    let window = [1.0, 2.0, 3.0];
    let groups = [0, 0, 0];
    let streaming = SVStreamingView::new(&window, 12, 7);
    let request = SVMoveRequest::new(SVColumnView::Streaming(streaming), &groups);
    let moved = sv_apply(
        &request,
        vec![SVMove::stack().into(), SVMove::normalize().into()],
    );

    let tops = moved.values().expect("the pipeline writes values");
    assert!((tops.value_at(2).expect("the third point") - 1.0).abs() < 1e-12);
    assert_eq!(
        request
            .values()
            .dispatch(|_| 0_u64, |_| 0, |view| view.sequence()),
        12
    );
}
