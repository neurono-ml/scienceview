//! Embedding the figure data without changing its representation.
//!
//! The data lands in the component as a plain JSON object so the host can show
//! the current point count, feed a live indicator or hand the values to its own
//! update path. A sparse column keeps only its stored entries and its logical
//! length; it is never expanded into a dense array.

use scienceview_core::sv_data::SVColumnView;
use scienceview_svg::sv_format_number;

/// Serializes a column view to the deterministic data object embedded in the
/// component source.
pub(super) fn sv_component_data_json(view: &SVColumnView<'_>) -> String {
    let representation = match view {
        SVColumnView::Dense(_) => "dense",
        SVColumnView::Sparse(_) => "sparse",
        SVColumnView::Streaming(_) => "streaming",
    };

    let entries: Vec<(usize, f64)> = match view {
        SVColumnView::Dense(values) => values.iter().copied().enumerate().collect(),
        SVColumnView::Sparse(sparse) => sparse.stored_entries().collect(),
        SVColumnView::Streaming(streaming) => {
            streaming.values().iter().copied().enumerate().collect()
        }
    };

    let points = entries
        .iter()
        .filter(|(_, value)| value.is_finite())
        .map(|(index, value)| format!("[{index},{}]", sv_format_number(*value)))
        .collect::<Vec<String>>()
        .join(",");

    format!(
        "{{\"logicalLength\":{},\"representation\":\"{representation}\",\"points\":[{points}]}}",
        view.length(),
    )
}
