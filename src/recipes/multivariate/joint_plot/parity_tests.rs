//! Dense, sparse and streaming input produce the same figure.

use super::test_support::{SVJointTestSource, sv_build_default};
use crate::sv_data::SVRepresentation;

#[test]
fn dense_sparse_and_streaming_agree() {
    let dense_source = SVJointTestSource::dense();
    let sparse_source = SVJointTestSource::sparse();
    let streaming_source = SVJointTestSource::streaming();

    let dense = sv_build_default(&dense_source);
    let sparse = sv_build_default(&sparse_source);
    let streaming = sv_build_default(&streaming_source);

    let dense_scene = dense.compose().expect("the dense figure composes");
    let sparse_scene = sparse.compose().expect("the sparse figure composes");
    let streaming_scene = streaming.compose().expect("the streaming figure composes");

    assert_eq!(dense.representation(), SVRepresentation::Dense);
    assert_eq!(sparse.representation(), SVRepresentation::Sparse);
    assert_eq!(streaming.representation(), SVRepresentation::Streaming);
    assert_eq!(
        dense_scene.number_of_mark_primitives(),
        sparse_scene.number_of_mark_primitives()
    );
    assert_eq!(
        dense_scene.number_of_mark_primitives(),
        streaming_scene.number_of_mark_primitives()
    );
    assert_eq!(
        dense_scene.decoration_primitives_after_marks(),
        sparse_scene.decoration_primitives_after_marks()
    );
    assert_eq!(
        dense_scene.decoration_primitives_before_marks(),
        streaming_scene.decoration_primitives_before_marks()
    );
}
