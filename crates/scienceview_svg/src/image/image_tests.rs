//! Tests for the encoder, observed the way a browser sees it: a data URI.

use super::base64::sv_base64_encode;
use super::png::sv_encode_png;
use super::sv_encode_png_data_uri;

#[test]
fn base64_encoding_covers_every_padding_case() {
    assert_eq!(sv_base64_encode(b""), "");
    assert_eq!(sv_base64_encode(b"f"), "Zg==");
    assert_eq!(sv_base64_encode(b"fo"), "Zm8=");
    assert_eq!(sv_base64_encode(b"foo"), "Zm9v");
    assert_eq!(sv_base64_encode(b"foob"), "Zm9vYg==");
}

#[test]
fn a_png_starts_with_the_signature_and_ends_with_iend() {
    let pixels = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let png = sv_encode_png(2, 2, &pixels);

    assert_eq!(&png[..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    assert_eq!(&png[png.len() - 8..png.len() - 4], b"IEND");
}

#[test]
fn a_raster_becomes_a_base64_png_data_uri() {
    let pixels = [
        255u8, 0, 0, 255, 0, 0, 255, 255, 0, 255, 0, 255, 255, 255, 0, 255,
    ];
    let uri = sv_encode_png_data_uri(2, 2, &pixels);

    assert!(
        uri.starts_with("data:image/png;base64,iVBORw0KGgo"),
        "{uri}"
    );
    assert!(!uri.contains("http"), "no external reference is allowed");
    assert_eq!(uri, sv_encode_png_data_uri(2, 2, &pixels));
}
