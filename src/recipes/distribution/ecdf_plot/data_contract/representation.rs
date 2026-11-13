//! The widest representation of the resolved columns.

use scienceview_core::sv_data::{SVColumnView, SVRepresentation};

pub(crate) fn sv_combined_representation(
    value: SVColumnView<'_>,
    series: Option<SVColumnView<'_>>,
) -> SVRepresentation {
    let views = [Some(value), series];
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
