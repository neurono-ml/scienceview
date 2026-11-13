//! A column resolved once, keeping its representation.

use scienceview_core::sv_data::{
    SVColumnSnapshot, SVColumnView, SVRepresentation, SVSparseView, SVStreamingView,
};
use std::collections::BTreeSet;

pub(crate) enum SVColumnAccess<'a> {
    Dense(&'a [f64]),
    Sparse(SVSparseView<'a>),
    Streaming(SVStreamingView<'a>),
}

impl<'a> SVColumnAccess<'a> {
    pub(crate) fn resolve(view: SVColumnView<'a>) -> Self {
        view.dispatch(
            Self::Dense,
            |sparse| Self::Sparse(*sparse),
            |streaming| Self::Streaming(*streaming),
        )
    }

    pub(crate) fn length(&self) -> usize {
        match self {
            Self::Dense(values) => values.len(),
            Self::Sparse(sparse) => sparse.length(),
            Self::Streaming(streaming) => streaming.length(),
        }
    }

    pub(crate) fn value_at(&self, index: usize) -> f64 {
        match self {
            Self::Dense(values) => values.get(index).copied().unwrap_or(0.0),
            Self::Sparse(sparse) => sparse.value_at(index).unwrap_or(0.0),
            Self::Streaming(streaming) => streaming.values().get(index).copied().unwrap_or(0.0),
        }
    }

    pub(crate) fn is_sparse(&self) -> bool {
        matches!(self, Self::Sparse(_))
    }

    /// Adds the logical positions that carry the shape of a sparse column.
    ///
    /// Every stored entry is a vertex. Between two stored entries, only the two
    /// positions that border the stored runs are needed: the implicit zeros in
    /// between are collinear, so they add no pixel and must not become a vertex
    /// per logical slot. The first and last logical positions keep the area's
    /// horizontal domain complete.
    pub(crate) fn collect_geometry_indices(&self, indices: &mut BTreeSet<usize>, length: usize) {
        let Self::Sparse(sparse) = self else {
            return;
        };
        if length == 0 {
            return;
        }
        if length == 1 {
            indices.insert(0);
            return;
        }

        let mut previous: Option<usize> = None;
        for (index, _) in sparse.stored_entries() {
            indices.insert(index);
            match previous {
                None => {
                    indices.insert(0);
                    if index > 0 {
                        indices.insert(index - 1);
                    }
                }
                Some(previous_index) if index > previous_index + 1 => {
                    indices.insert(previous_index + 1);
                    indices.insert(index - 1);
                }
                Some(_) => {}
            }
            previous = Some(index);
        }

        match previous {
            None => {
                indices.insert(0);
                indices.insert(length - 1);
            }
            Some(last) => {
                indices.insert(length - 1);
                if last + 1 < length {
                    indices.insert(last + 1);
                }
            }
        }
    }
}

/// Borrows the current view of a snapshot for the length of the operation.
pub(crate) fn sv_snapshot_view<'snapshot>(
    snapshot: &'snapshot mut SVColumnSnapshot<'_>,
) -> SVColumnView<'snapshot> {
    snapshot.column_view()
}

impl SVColumnAccess<'_> {
    pub(crate) fn representation(&self) -> SVRepresentation {
        match self {
            Self::Sparse(_) => SVRepresentation::Sparse,
            Self::Dense(_) => SVRepresentation::Dense,
            Self::Streaming(_) => SVRepresentation::Streaming,
        }
    }
}
