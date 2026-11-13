//! Reading a column view into drawable points without changing representation.

use scienceview_core::sv_data::SVColumnView;

/// The points one bare figure draws, plus the logical extent of the column.
///
/// A sparse column contributes only the entries it stores; its logical extent
/// still sets the horizontal domain, so a gap stays a gap. A streaming column
/// contributes the bounded snapshot it already carries, so the render thread
/// never waits for the producer.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SVSeries {
    pub(crate) logical_length: usize,
    pub(crate) points: Vec<(f64, f64)>,
}

/// Resolves the representation exactly once and collects its finite values.
pub(crate) fn sv_collect_series(view: &SVColumnView<'_>) -> SVSeries {
    let logical_length = view.length();
    let points: Vec<(f64, f64)> = match view {
        SVColumnView::Dense(values) => values
            .iter()
            .copied()
            .enumerate()
            .map(|(index, value)| (index as f64, value))
            .collect::<Vec<(f64, f64)>>(),
        SVColumnView::Streaming(streaming) => streaming
            .values()
            .iter()
            .copied()
            .enumerate()
            .map(|(index, value)| (index as f64, value))
            .collect::<Vec<(f64, f64)>>(),
        SVColumnView::Sparse(sparse) => sparse
            .stored_entries()
            .map(|(index, value)| (index as f64, value))
            .collect::<Vec<(f64, f64)>>(),
    }
    .into_iter()
    .filter(|(horizontal, vertical)| horizontal.is_finite() && vertical.is_finite())
    .collect();

    SVSeries {
        logical_length,
        points,
    }
}
