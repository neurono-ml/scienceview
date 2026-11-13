//! Reading the source columns through one representation dispatch.

use super::super::core_implementation::SVStepPlot;
use super::grouping::SVStepSample;
use super::sparse_indices::sv_sample_indices;
use scienceview_core::sv_data::{SVColumnView, SVRepresentation, SVSparseView};
use scienceview_core::sv_error::SVError;

/// A column read through one representation dispatch.
pub(super) enum SVStepCursor<'a> {
    Dense(&'a [f64]),
    Sparse(SVSparseView<'a>),
    Streaming(&'a [f64]),
}

impl<'a> SVStepCursor<'a> {
    pub(super) fn value_at(&self, index: usize) -> Option<f64> {
        match self {
            SVStepCursor::Dense(values) | SVStepCursor::Streaming(values) => {
                values.get(index).copied()
            }
            SVStepCursor::Sparse(sparse) => sparse.value_at(index),
        }
    }

    pub(super) fn sparse(&self) -> Option<&SVSparseView<'a>> {
        match self {
            SVStepCursor::Sparse(sparse) => Some(sparse),
            SVStepCursor::Dense(_) | SVStepCursor::Streaming(_) => None,
        }
    }
}

fn sv_cursor(view: SVColumnView<'_>) -> SVStepCursor<'_> {
    view.dispatch(
        SVStepCursor::Dense,
        |sparse| SVStepCursor::Sparse(*sparse),
        |streaming| SVStepCursor::Streaming(streaming.values()),
    )
}

/// Reads `x`, `y` and the optional `series` column once and resolves the
/// representation the operation reports.
pub(crate) fn sv_collect_samples(
    plot: &SVStepPlot<'_>,
) -> Result<(Vec<SVStepSample>, SVRepresentation), SVError> {
    let mut x_snapshot = plot.data_source.column(&plot.x)?;
    let mut y_snapshot = plot.data_source.column(&plot.y)?;
    let mut series_snapshot = match plot.series.as_deref() {
        Some(name) => Some(plot.data_source.column(name)?),
        None => None,
    };

    let x_view = x_snapshot.column_view();
    let y_view = y_snapshot.column_view();
    let series_view = series_snapshot
        .as_mut()
        .map(|snapshot| snapshot.column_view());

    let length = x_view.length();
    if y_view.length() != length {
        return Err(SVError::MismatchedLengths {
            name: plot.y.clone(),
            expected: length,
            actual: y_view.length(),
        });
    }
    if let Some(series_view) = series_view {
        if series_view.length() != length {
            return Err(SVError::MismatchedLengths {
                name: plot.series.clone().unwrap_or_else(|| "series".to_string()),
                expected: length,
                actual: series_view.length(),
            });
        }
    }

    let representation = sv_combined_representation(x_view, y_view, series_view);
    let x_cursor = sv_cursor(x_view);
    let y_cursor = sv_cursor(y_view);
    let series_cursor = series_view.map(sv_cursor);

    let indices = sv_sample_indices(
        &x_cursor,
        &y_cursor,
        length,
        plot.duplicate_policy,
        plot.aggregate.is_some(),
        plot.series.is_some(),
    );

    let mut samples = Vec::with_capacity(indices.len());
    for index in indices {
        let (Some(x), Some(y)) = (x_cursor.value_at(index), y_cursor.value_at(index)) else {
            continue;
        };
        if !x.is_finite() || !y.is_finite() {
            continue;
        }
        let series = match &series_cursor {
            Some(cursor) => match cursor.value_at(index) {
                Some(value) if value.is_finite() => Some(value),
                _ => continue,
            },
            None => None,
        };
        samples.push(SVStepSample { x, y, series });
    }

    Ok((samples, representation))
}

pub(crate) fn sv_combined_representation(
    x: SVColumnView<'_>,
    y: SVColumnView<'_>,
    series: Option<SVColumnView<'_>>,
) -> SVRepresentation {
    let views = [Some(x), Some(y), series];
    if views
        .iter()
        .flatten()
        .any(|view| view.representation() == SVRepresentation::Streaming)
    {
        SVRepresentation::Streaming
    } else if views
        .iter()
        .flatten()
        .any(|view| view.representation() == SVRepresentation::Sparse)
    {
        SVRepresentation::Sparse
    } else {
        SVRepresentation::Dense
    }
}
