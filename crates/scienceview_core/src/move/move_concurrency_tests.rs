//! Concurrency: a move sequence is read-only over borrowed data.

use crate::sv_data::SVColumnView;
use crate::sv_move::{SVMove, SVMoveOutput, SVMoveRequest, SVMoveSequence};

/// Applies a shared sequence to one request.
fn sv_apply(sequence: &SVMoveSequence, request: &SVMoveRequest<'_>) -> SVMoveOutput {
    sequence.apply(request).expect("the sequence applies")
}

#[test]
fn one_sequence_applies_concurrently_without_sharing_state() {
    let values: Vec<f64> = (0..256).map(|step| (step % 7) as f64).collect();
    let groups: Vec<u32> = (0..256).map(|step| (step % 3) as u32).collect();
    let sequence = SVMoveSequence::new(vec![
        SVMove::stack().into(),
        SVMove::shift().amount(0.25).into(),
        SVMove::normalize().into(),
    ])
    .expect("the sequence is ordered");

    let request = SVMoveRequest::new(SVColumnView::Dense(&values), &groups);
    let expected = sv_apply(&sequence, &request);

    std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for _ in 0..8 {
            handles.push(scope.spawn(|| sv_apply(&sequence, &request)));
        }

        for handle in handles {
            let output = handle.join().expect("the worker thread finished");
            assert_eq!(output, expected);
        }
    });
}

#[test]
fn different_seeds_jitter_independently_across_threads() {
    let values = [0.0; 64];
    let groups = [0_u32; 64];

    std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for seed in 0..8_u64 {
            handles.push(scope.spawn(move || {
                let request = SVMoveRequest::new(SVColumnView::Dense(&values), &groups);
                let sequence = SVMoveSequence::single(SVMove::jitter().seed(seed).into())
                    .expect("a single jitter is a valid sequence");
                (
                    seed,
                    sv_apply(&sequence, &request)
                        .dense_horizontal_offsets()
                        .expect("a dense displacement")
                        .to_vec(),
                )
            }));
        }

        let results: Vec<(u64, Vec<f64>)> = handles
            .into_iter()
            .map(|handle| handle.join().expect("the worker thread finished"))
            .collect();

        for left in 0..results.len() {
            for right in (left + 1)..results.len() {
                assert_ne!(results[left].1, results[right].1);
            }
        }
    });
}
