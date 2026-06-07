//! Run-length encoding optimized for post-BWT data.
//!
//! After BWT + MTF, the data consists mostly of small values (0s and 1s)
//! with long runs. This module provides RLE variants that efficiently
//! encode such data.

/// Run-length encode post-BWT/MTF data using a simple scheme.
///
/// Each run is stored as `(value, count)` where count is a `u16`.
/// This is effective for the long runs of zeros typical after BWT+MTF.
///
/// # Examples
///
/// ```
/// use compress_bwt_rs::rle_post;
///
/// let data = vec![0, 0, 0, 0, 1, 2, 0, 0, 0];
/// let encoded = rle_post::encode(&data);
/// let decoded = rle_post::decode(&encoded);
/// assert_eq!(data, decoded);
/// ```
pub fn encode(data: &[u8]) -> Vec<(u8, u16)> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut current = data[0];
    let mut count = 1u16;

    for &byte in &data[1..] {
        if byte == current && count < 65535 {
            count += 1;
        } else {
            result.push((current, count));
            current = byte;
            count = 1;
        }
    }
    result.push((current, count));
    result
}

/// Decode post-BWT run-length encoded data.
pub fn decode(pairs: &[(u8, u16)]) -> Vec<u8> {
    let total: usize = pairs.iter().map(|&(_, c)| c as usize).sum();
    let mut result = Vec::with_capacity(total);
    for &(byte, count) in pairs {
        for _ in 0..count {
            result.push(byte);
        }
    }
    result
}

/// Encode into a compact byte format suitable for post-BWT data.
///
/// Uses a bzip2-style RUNA/RUNB encoding for zeros, and direct byte
/// values for non-zero entries. Simplified version:
///
/// - If value is 0 and count > 0: `[0x00, count_u8]` (up to 255 zeros)
/// - Otherwise: `[value]` for single occurrences, or `[0xFF, value, count_u8]`
///   for repeated non-zero values.
pub fn encode_compact(data: &[u8]) -> Vec<u8> {
    let pairs = encode(data);
    let mut out = Vec::new();

    for (value, mut count) in pairs {
        if value == 0 {
            while count > 0 {
                let chunk = count.min(255) as u8;
                out.push(0x00);
                out.push(chunk);
                count -= chunk as u16;
            }
        } else if count == 1 {
            // Send as-is (escape 0x00 and 0xFF)
            if value == 0xFF {
                out.push(0xFF);
                out.push(0xFF);
                out.push(1);
            } else {
                out.push(value);
            }
        } else {
            while count > 0 {
                let chunk = count.min(255) as u8;
                out.push(0xFF);
                out.push(value);
                out.push(chunk);
                count -= chunk as u16;
            }
        }
    }

    out
}

/// Decode from compact byte format.
pub fn decode_compact(data: &[u8]) -> Option<Vec<u8>> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < data.len() {
        match data[i] {
            0x00 => {
                i += 1;
                if i >= data.len() {
                    return None;
                }
                let count = data[i] as usize;
                result.extend(std::iter::repeat_n(0, count));
                i += 1;
            }
            0xFF => {
                i += 1;
                if i + 1 >= data.len() {
                    return None;
                }
                let value = data[i];
                let count = data[i + 1] as usize;
                for _ in 0..count {
                    result.push(value);
                }
                i += 2;
            }
            value => {
                result.push(value);
                i += 1;
            }
        }
    }

    Some(result)
}

/// Compute compression statistics.
///
/// Returns `(encoded_pair_count, original_byte_count)`.
pub fn compression_stats(data: &[u8]) -> (usize, usize) {
    let pairs = encode(data);
    (pairs.len(), data.len())
}

/// Estimate the compact encoded size in bytes.
pub fn compact_size(data: &[u8]) -> usize {
    encode_compact(data).len()
}
