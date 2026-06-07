//! Forward Burrows-Wheeler Transform.

use crate::suffix;

/// Result of the forward BWT.
///
/// Contains the transformed data and the primary index (position of the
/// original first byte in the sorted rotation list).
#[derive(Debug, Clone)]
pub struct BwtResult {
    /// The BWT output bytes.
    pub bwt: Vec<u8>,
    /// The primary index (position of the original string in the rotation table).
    pub primary_index: usize,
}

/// Compute the forward BWT of the input data.
///
/// The BWT sorts all cyclic rotations of the input and outputs the last column
/// of the sorted rotation matrix, along with the primary index.
///
/// # Examples
///
/// ```
/// use compress_bwt_rs::bwt_forward;
///
/// let (bwt, idx) = bwt_forward::transform(b"banana");
/// assert_eq!(idx, 3); // "banana$" sorts at position 3
/// ```
pub fn transform(data: &[u8]) -> (Vec<u8>, usize) {
    if data.is_empty() {
        return (Vec::new(), 0);
    }

    let sa = suffix::build_suffix_array(data);
    suffix::bwt_from_suffix_array(data, &sa)
}

/// Transform using the BwtResult struct.
pub fn transform_struct(data: &[u8]) -> BwtResult {
    let (bwt, primary_index) = transform(data);
    BwtResult { bwt, primary_index }
}

/// Compute BWT using a naive approach (for verification).
///
/// Generates all rotations, sorts them, and extracts the last column.
/// This is O(n² log n) and should only be used for small inputs or testing.
pub fn transform_naive(data: &[u8]) -> (Vec<u8>, usize) {
    if data.is_empty() {
        return (Vec::new(), 0);
    }

    let n = data.len();
    let mut rotations: Vec<(Vec<u8>, usize)> = (0..n)
        .map(|i| {
            let rot: Vec<u8> = data[i..].iter().chain(data[..i].iter()).copied().collect();
            (rot, i)
        })
        .collect();

    rotations.sort_by(|a, b| a.0.cmp(&b.0));

    let bwt: Vec<u8> = rotations.iter().map(|(rot, _)| rot[n - 1]).collect();
    let primary_index = rotations.iter().position(|(_, idx)| *idx == 0).unwrap();

    (bwt, primary_index)
}
