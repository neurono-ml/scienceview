//! Executable specification that a key resolution reads one representation and
//! never densifies it.

use crate::sv_data::{SVColumnView, SVSparseView, SVStreamingView};
use crate::sv_transition::{SVMatchKey, sv_resolve_match_keys};

#[test]
fn a_sparse_key_source_is_read_over_its_stored_observations_only() {
    // The sparse view declares seven logical positions but stores two
    // observations. Reading it must yield two keys, not seven: an implicit zero
    // is not an observation, so densifying the column would invent five marks.
    let indices = [1, 5];
    let values = [3.0, 5.0];
    let sparse = SVSparseView::new(&indices, &values, 7).expect("the sparse view is valid");

    let keys = sv_resolve_match_keys(SVColumnView::Sparse(sparse), SVMatchKey::Category)
        .expect("stored codes are finite");

    assert_eq!(keys.len(), 2, "an implicit zero carries no key");
}

#[test]
fn a_streaming_key_source_reads_its_window_not_its_logical_length() {
    // The streamed column declares twelve logical positions and carries a
    // two-value window snapshot. Reading it must yield the two keys of the
    // snapshot, without waiting for the rest of the stream.
    let window = [4.0, 6.0];
    let streaming = SVStreamingView::new(&window, 12, 3);

    let keys = sv_resolve_match_keys(SVColumnView::Streaming(streaming), SVMatchKey::Identity)
        .expect("the window codes are finite");

    assert_eq!(keys.len(), 2, "the bounded window is the read unit");
}
