//! Pairwise observation readers over the three representations.
//!
//! The density contour needs the `(x, y)` pair of every logical observation.
//! Both columns dispatch on their representation exactly once. A sparse column
//! is never materialised: its stored entries are merged with the other
//! column's, and the implicit-zero run between two stored entries is reported
//! once with its multiplicity.
//!
//! The ungrouped, unweighted path uses the efficient multiplicity walker; a
//! series or weight column needs a per-observation index, so the indexed walker
//! visits every logical position without ever building a dense value buffer.

use crate::sv_data::{SVColumnView, SVSparseView};

enum SVAxis<'a> {
    Slice(&'a [f64]),
    Sparse(SVSparseView<'a>),
}

impl<'a> SVAxis<'a> {
    fn from_view(view: SVColumnView<'a>) -> Self {
        match view {
            SVColumnView::Dense(values) => SVAxis::Slice(values),
            SVColumnView::Sparse(sparse) => SVAxis::Sparse(sparse),
            SVColumnView::Streaming(streaming) => SVAxis::Slice(streaming.values()),
        }
    }

    fn value_at(&self, index: usize) -> f64 {
        match self {
            SVAxis::Slice(values) => values.get(index).copied().unwrap_or(0.0),
            SVAxis::Sparse(sparse) => sparse.value_at(index).unwrap_or(0.0),
        }
    }
}

/// Visits every observation pair once, with the multiplicity of the run.
///
/// A dense or streaming pair is visited once per index. A sparse pair is
/// visited per stored index of either column, plus one visit for the run of
/// positions implicit in both columns; the callback's `multiplicity` is the
/// number of observations that run represents.
pub(crate) fn sv_visit_pairs(
    x: SVColumnView<'_>,
    y: SVColumnView<'_>,
    mut visit: impl FnMut(f64, f64, usize),
) {
    match (SVAxis::from_view(x), SVAxis::from_view(y)) {
        (SVAxis::Slice(horizontal), SVAxis::Slice(vertical)) => {
            for (x_value, y_value) in horizontal.iter().zip(vertical.iter()) {
                visit(*x_value, *y_value, 1);
            }
        }
        (SVAxis::Slice(horizontal), SVAxis::Sparse(vertical)) => {
            let mut next = 0;
            for (index, x_value) in horizontal.iter().enumerate() {
                let y_value = sv_sparse_value(&vertical, index, &mut next);
                visit(*x_value, y_value, 1);
            }
        }
        (SVAxis::Sparse(horizontal), SVAxis::Slice(vertical)) => {
            let mut next = 0;
            for (index, y_value) in vertical.iter().enumerate() {
                let x_value = sv_sparse_value(&horizontal, index, &mut next);
                visit(x_value, *y_value, 1);
            }
        }
        (SVAxis::Sparse(horizontal), SVAxis::Sparse(vertical)) => {
            sv_visit_sparse_pairs(&horizontal, &vertical, visit);
        }
    }
}

/// Visits every logical observation with its index.
///
/// The indexed walker is used when a series or a weight column must be read per
/// observation; it never densifies a sparse column.
pub(crate) fn sv_visit_indexed_pairs(
    x: SVColumnView<'_>,
    y: SVColumnView<'_>,
    mut visit: impl FnMut(usize, f64, f64),
) {
    let horizontal = SVAxis::from_view(x);
    let vertical = SVAxis::from_view(y);
    for index in 0..x.length() {
        visit(index, horizontal.value_at(index), vertical.value_at(index));
    }
}

fn sv_sparse_value(sparse: &SVSparseView<'_>, index: usize, next: &mut usize) -> f64 {
    match sparse.indices().get(*next).copied() {
        Some(stored) if stored == index => {
            *next += 1;
            sparse.values().get(*next - 1).copied().unwrap_or(0.0)
        }
        _ => 0.0,
    }
}

fn sv_visit_sparse_pairs(
    horizontal: &SVSparseView<'_>,
    vertical: &SVSparseView<'_>,
    mut visit: impl FnMut(f64, f64, usize),
) {
    let mut horizontal_position = 0;
    let mut vertical_position = 0;
    let mut covered = 0;
    loop {
        let horizontal_index = horizontal
            .indices()
            .get(horizontal_position)
            .copied()
            .unwrap_or(usize::MAX);
        let vertical_index = vertical
            .indices()
            .get(vertical_position)
            .copied()
            .unwrap_or(usize::MAX);
        if horizontal_index == usize::MAX && vertical_index == usize::MAX {
            break;
        }
        let index = horizontal_index.min(vertical_index);
        let x_value = if horizontal_index == index {
            horizontal_position += 1;
            horizontal.values()[horizontal_position - 1]
        } else {
            0.0
        };
        let y_value = if vertical_index == index {
            vertical_position += 1;
            vertical.values()[vertical_position - 1]
        } else {
            0.0
        };
        covered += 1;
        visit(x_value, y_value, 1);
    }

    let implicit = horizontal.length().saturating_sub(covered);
    if implicit > 0 {
        visit(0.0, 0.0, implicit);
    }
}
