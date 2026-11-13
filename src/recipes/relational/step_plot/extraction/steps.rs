//! Expanding ordered samples into explicit step vertices.

use super::super::options::SVStepPosition;

/// Expands ordered samples into the explicit vertices of the requested step.
pub(crate) fn sv_expand_step(
    points: &[(f64, f64)],
    position: SVStepPosition,
) -> (Vec<f64>, Vec<f64>) {
    let mut horizontal = Vec::with_capacity(points.len().saturating_mul(2));
    let mut vertical = Vec::with_capacity(points.len().saturating_mul(2));
    if points.is_empty() {
        return (horizontal, vertical);
    }

    {
        let mut push = |x: f64, y: f64| {
            horizontal.push(x);
            vertical.push(y);
        };
        sv_push_step_vertices(points, position, &mut push);
    }

    (horizontal, vertical)
}

type SVStepVertexSink<'sink> = dyn FnMut(f64, f64) + 'sink;

fn sv_push_step_vertices(
    points: &[(f64, f64)],
    position: SVStepPosition,
    push: &mut SVStepVertexSink<'_>,
) {
    match position {
        SVStepPosition::After => {
            for (index, point) in points.iter().enumerate() {
                push(point.0, point.1);
                if let Some(next) = points.get(index + 1) {
                    push(next.0, point.1);
                }
            }
        }
        SVStepPosition::Before => {
            for (index, point) in points.iter().enumerate() {
                push(point.0, point.1);
                if let Some(next) = points.get(index + 1) {
                    push(point.0, next.1);
                }
            }
        }
        SVStepPosition::Center => {
            push(points[0].0, points[0].1);
            for pair in points.windows(2) {
                let (left, right) = (pair[0], pair[1]);
                let middle = left.0 + (right.0 - left.0) / 2.0;
                push(middle, left.1);
                push(middle, right.1);
            }
            if let Some(last) = points.last() {
                push(last.0, last.1);
            }
        }
    }
}
