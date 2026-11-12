//! The PNG container: signature, header, one uncompressed image data chunk.

use super::base64::sv_base64_encode;

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
const MAXIMUM_STORED_BLOCK: usize = 65_535;

/// Encodes RGBA pixels as a PNG data URI.
///
/// A raster whose pixel buffer does not hold `width * height * 4` bytes is
/// rejected before a malformed chunk is written. The caller (the image mark)
/// already validates this; doing it again keeps the encoder total.
pub(crate) fn sv_encode_png_data_uri(width: usize, height: usize, pixels: &[u8]) -> String {
    let png = sv_encode_png(width, height, pixels);
    format!("data:image/png;base64,{}", sv_base64_encode(&png))
}

pub(crate) fn sv_encode_png(width: usize, height: usize, pixels: &[u8]) -> Vec<u8> {
    let mut png = Vec::new();
    png.extend_from_slice(&PNG_SIGNATURE);

    let mut header = Vec::with_capacity(13);
    header.extend_from_slice(&(width as u32).to_be_bytes());
    header.extend_from_slice(&(height as u32).to_be_bytes());
    header.extend_from_slice(&[8, 6, 0, 0, 0]);
    sv_push_chunk(&mut png, b"IHDR", &header);

    let mut scanlines = Vec::with_capacity(height.saturating_mul(width.saturating_mul(4) + 1));
    for row in 0..height {
        scanlines.push(0);
        let start = row.saturating_mul(width).saturating_mul(4);
        let end = start
            .saturating_add(width.saturating_mul(4))
            .min(pixels.len());
        scanlines.extend_from_slice(&pixels[start.min(pixels.len())..end]);
    }
    sv_push_chunk(&mut png, b"IDAT", &sv_zlib_stored(&scanlines));
    sv_push_chunk(&mut png, b"IEND", &[]);

    png
}

fn sv_push_chunk(png: &mut Vec<u8>, chunk_type: &[u8; 4], data: &[u8]) {
    png.extend_from_slice(&(data.len() as u32).to_be_bytes());
    png.extend_from_slice(chunk_type);
    png.extend_from_slice(data);

    let mut crc_input = Vec::with_capacity(4 + data.len());
    crc_input.extend_from_slice(chunk_type);
    crc_input.extend_from_slice(data);
    png.extend_from_slice(&sv_crc32(&crc_input).to_be_bytes());
}

fn sv_zlib_stored(data: &[u8]) -> Vec<u8> {
    let mut stream = vec![0x78, 0x01];

    if data.is_empty() {
        stream.extend_from_slice(&[0x01, 0x00, 0x00, 0xff, 0xff]);
    } else {
        let mut remaining = data;
        while !remaining.is_empty() {
            let length = remaining.len().min(MAXIMUM_STORED_BLOCK);
            let is_final = length == remaining.len();
            stream.push(u8::from(is_final));
            let length = length as u16;
            stream.extend_from_slice(&length.to_le_bytes());
            stream.extend_from_slice(&(!length).to_le_bytes());
            stream.extend_from_slice(&remaining[..usize::from(length)]);
            remaining = &remaining[usize::from(length)..];
        }
    }

    stream.extend_from_slice(&sv_adler32(data).to_be_bytes());
    stream
}

fn sv_crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xedb8_8320 & mask);
        }
    }
    !crc
}

fn sv_adler32(bytes: &[u8]) -> u32 {
    let mut first = 1u32;
    let mut second = 0u32;
    for byte in bytes {
        first = (first + u32::from(*byte)) % 65_521;
        second = (second + first) % 65_521;
    }
    (second << 16) | first
}
