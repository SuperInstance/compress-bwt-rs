//! Inverse Burrows-Wheeler Transform.

/// Recover the original data from BWT output and the primary index.
///
/// Uses the standard LF-mapping technique to reconstruct the original string.
///
/// # Arguments
///
/// * `bwt` - The BWT output bytes.
/// * `primary_index` - The primary index from the forward transform.
///
/// # Returns
///
/// `Some(original_data)` if successful, `None` if inputs are invalid.
///
/// # Examples
///
/// ```
/// use compress_bwt_rs::{bwt_forward, bwt_inverse};
///
/// let data = b"banana";
/// let (bwt, idx) = bwt_forward::transform(data);
/// let recovered = bwt_inverse::inverse(&bwt, idx).unwrap();
/// assert_eq!(data.as_slice(), recovered.as_slice());
/// ```
pub fn inverse(bwt: &[u8], primary_index: usize) -> Option<Vec<u8>> {
    if bwt.is_empty() {
        return Some(Vec::new());
    }
    if primary_index >= bwt.len() {
        return None;
    }

    let n = bwt.len();

    // Build the LF mapping
    // First, count occurrences of each byte
    let mut count = vec![0usize; 256];
    for &b in bwt {
        count[b as usize] += 1;
    }

    // Compute cumulative counts (starting position of each byte in F column)
    let mut start = vec![0usize; 256];
    let mut sum = 0;
    for i in 0..256 {
        start[i] = sum;
        sum += count[i];
    }

    // Build LF mapping: for each position i in the L column,
    // LF[i] = start[bwt[i]] + (number of bwt[j] == bwt[i] for j < i)
    let mut lf = vec![0usize; n];
    let mut occ = vec![0usize; 256];
    for i in 0..n {
        let b = bwt[i] as usize;
        lf[i] = start[b] + occ[b];
        occ[b] += 1;
    }

    // Reconstruct by following LF from primary_index
    let mut result = vec![0u8; n];
    let mut idx = primary_index;
    for i in (0..n).rev() {
        result[i] = bwt[idx];
        idx = lf[idx];
    }

    Some(result)
}

/// Verify that a BWT round-trip produces the original data.
///
/// Returns `Ok(())` if the round-trip succeeds, `Err((original, recovered))`
/// otherwise.
pub fn verify_roundtrip(data: &[u8]) -> Result<(), (Vec<u8>, Vec<u8>)> {
    let (bwt, idx) = crate::bwt_forward::transform(data);
    match inverse(&bwt, idx) {
        Some(recovered) if recovered == data => Ok(()),
        Some(recovered) => Err((data.to_vec(), recovered)),
        None => Err((data.to_vec(), Vec::new())),
    }
}
