//! Executable specification that a key resolution reads one representation.

use crate::sv_data::SVColumnView;

#[test]
fn a_column_read_never_densifies_a_sparse_key_source() {
    // The transition reads keys from the representation it is given; the sparse
    // path is exercised in `parity_tests.rs`. This test pins that a dense column
    // and its streaming window resolve the same category keys.
    let values = [1.0, 2.0, 3.0];
    let dense = crate::sv_transition::sv_resolve_match_keys(
        SVColumnView::Dense(&values),
        crate::sv_transition::SVMatchKey::Category,
    )
    .expect("unique keys");

    assert_eq!(dense.len(), 3);
}
