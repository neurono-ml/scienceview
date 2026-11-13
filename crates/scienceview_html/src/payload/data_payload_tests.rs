//! Tests for the payload shape and representation parity.

use super::SVHtmlDataPayload;
use scienceview_core::sv_data::{SVColumnView, SVRepresentation, SVSparseView, SVStreamingView};

const DENSE_VALUES: [f64; 3] = [1.0, 2.0, 3.0];

#[test]
fn a_dense_column_serializes_with_a_fixed_key_order() {
    let payload = SVHtmlDataPayload::new(SVColumnView::Dense(&DENSE_VALUES), "m");

    assert_eq!(
        payload.to_json(),
        "{\"version\":1,\"representation\":\"Dense\",\"data\":{\"values\":[1,2,3]},\
\"points\":3,\"revision\":0,\"appended_points\":0,\
\"clock\":{\"position\":0,\"tick_milliseconds\":16,\"reduced_motion\":false},\"markup\":\"m\"}"
    );
}

#[test]
fn a_sparse_column_stays_sparse_in_the_payload() {
    let indices = [0_usize, 2];
    let values = [1.0, 3.0];
    let sparse = SVSparseView::new(&indices, &values, 5).expect("valid sparse view");
    let payload = SVHtmlDataPayload::new(SVColumnView::Sparse(sparse), "m");
    let json = payload.to_json();

    assert_eq!(payload.representation(), SVRepresentation::Sparse);
    assert_eq!(
        json,
        "{\"version\":1,\"representation\":\"Sparse\",\
\"data\":{\"indices\":[0,2],\"values\":[1,3],\"logical_length\":5},\
\"points\":5,\"revision\":0,\"appended_points\":0,\
\"clock\":{\"position\":0,\"tick_milliseconds\":16,\"reduced_motion\":false},\"markup\":\"m\"}"
    );
    assert!(
        !json.contains("2,0,3"),
        "the sparse gap is not densified: {json}"
    );
}

#[test]
fn a_streaming_snapshot_keeps_its_clock_metadata() {
    let snapshot = [4.0, 5.0];
    let streaming = SVStreamingView::new(&snapshot, 9, 2);
    let payload = SVHtmlDataPayload::new(SVColumnView::Streaming(streaming), "m");
    let json = payload.to_json();

    assert_eq!(payload.representation(), SVRepresentation::Streaming);
    assert!(json.contains("\"data\":{\"values\":[4,5],\"sequence\":9,\"dropped_samples\":2}"));
}

#[test]
fn dense_parse_and_streaming_columns_embed_the_same_points() {
    let snapshot = [1.0, 2.0, 3.0];
    let dense = SVHtmlDataPayload::new(SVColumnView::Dense(&DENSE_VALUES), "m");
    let streaming = SVHtmlDataPayload::new(
        SVColumnView::Streaming(SVStreamingView::new(&snapshot, 0, 0)),
        "m",
    );

    assert_eq!(dense.points(), 3);
    assert_eq!(streaming.points(), 3);
    assert!(dense.to_json().contains("\"values\":[1,2,3]"));
    assert!(streaming.to_json().contains("\"values\":[1,2,3]"));
}

#[test]
fn appended_points_advance_the_model_clock() {
    let payload =
        SVHtmlDataPayload::new(SVColumnView::Dense(&DENSE_VALUES), "m").with_appended_points(2);

    assert_eq!(payload.revision(), 2);
    assert!(payload.to_json().contains("\"revision\":2"));
    assert!(payload.to_json().contains("\"appended_points\":2"));
    assert!(payload.to_json().contains("\"clock\":{\"position\":2"));
}

#[test]
fn a_title_that_looks_like_a_script_tag_is_escaped_in_the_markup_field() {
    let payload = SVHtmlDataPayload::new(
        SVColumnView::Dense(&DENSE_VALUES),
        "</script><script>alert(1)</script>",
    );
    let json = payload.to_json();

    assert!(!json.contains("</script>"), "{json}");
    assert!(json.contains("\\u003c/script\\u003e"), "{json}");
}
