//! Representation parity: the same data through dense, sparse and streaming.

use crate::sv_data::{SVColumnView, SVSparseView, SVStreamingView};
use crate::sv_move::{SVMove, SVMoveOutput, SVMoveRequest, SVMoveSequence};

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

/// Asserts that three outputs carry the same displacement.
fn assert_same_displacement(outputs: &[SVMoveOutput; 3]) {
    for output in &outputs[1..] {
        assert_eq!(output.horizontal_offsets(), outputs[0].horizontal_offsets());
        assert_eq!(output.vertical_offsets(), outputs[0].vertical_offsets());
        assert_eq!(output.values(), outputs[0].values());
        assert_eq!(output.baselines(), outputs[0].baselines());
        assert_eq!(output.widths(), outputs[0].widths());
    }
}

/// Runs one sequence through the three representations and compares them.
fn assert_parity(moves: Vec<SVMove>) {
    let sparse = SVSparseView::new(&SPARSE_INDICES, &SPARSE_VALUES, DENSE_VALUES.len())
        .expect("the sparse view is valid");
    let streaming = SVStreamingView::new(&DENSE_VALUES, 1, 0);
    let requests = sv_requests(&sparse, &streaming);

    let outputs = [
        sv_apply(&requests[0], moves.clone()),
        sv_apply(&requests[1], moves.clone()),
        sv_apply(&requests[2], moves),
    ];
    assert_same_displacement(&outputs);
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
    assert_parity(vec![SVMove::dodge().into()]);
}

#[test]
fn jitter_has_representation_parity() {
    assert_parity(vec![SVMove::jitter().seed(8).into()]);
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
    assert!((tops[2] - 1.0).abs() < 1e-12);
    assert_eq!(
        request
            .values()
            .dispatch(|_| 0_u64, |_| 0, |view| view.sequence()),
        12
    );
}
