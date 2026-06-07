//! Move-to-front transform for post-BWT entropy coding.

/// Apply the move-to-front (MTF) transform.
///
/// The MTF maintains a list of byte values (0–255). For each input byte,
/// it outputs the position of that byte in the list, then moves it to the
/// front (position 0). After BWT, this converts clustered bytes into small
/// integers, which compress well with entropy coders.
///
/// # Examples
///
/// ```
/// use compress_bwt_rs::mtf;
///
/// let data = b"banana";
/// let encoded = mtf::encode(data);
/// let decoded = mtf::decode(&encoded);
/// assert_eq!(data.as_slice(), decoded.as_slice());
/// ```
pub fn encode(data: &[u8]) -> Vec<u8> {
    let mut list: Vec<u8> = (0..=255).collect();
    let mut result = Vec::with_capacity(data.len());

    for &byte in data {
        let pos = list.iter().position(|&b| b == byte).unwrap_or(0);
        result.push(pos as u8);
        list.remove(pos);
        list.insert(0, byte);
    }

    result
}

/// Apply the inverse move-to-front transform.
///
/// Reconstructs the original bytes from MTF-encoded data.
pub fn decode(data: &[u8]) -> Vec<u8> {
    let mut list: Vec<u8> = (0..=255).collect();
    let mut result = Vec::with_capacity(data.len());

    for &pos in data {
        let byte = list[pos as usize];
        result.push(byte);
        list.remove(pos as usize);
        list.insert(0, byte);
    }

    result
}

/// Apply MTF with a custom alphabet (for testing/specialized use).
///
/// The alphabet must contain all byte values present in the input.
pub fn encode_with_alphabet(data: &[u8], alphabet: &[u8]) -> Vec<u8> {
    let mut list = alphabet.to_vec();
    let mut result = Vec::with_capacity(data.len());

    for &byte in data {
        let pos = list.iter().position(|&b| b == byte).unwrap_or(0);
        result.push(pos as u8);
        list.remove(pos);
        list.insert(0, byte);
    }

    result
}

/// Decode MTF with a custom alphabet.
pub fn decode_with_alphabet(data: &[u8], alphabet: &[u8]) -> Vec<u8> {
    let mut list = alphabet.to_vec();
    let mut result = Vec::with_capacity(data.len());

    for &pos in data {
        if pos as usize >= list.len() {
            result.push(0);
            continue;
        }
        let byte = list[pos as usize];
        result.push(byte);
        list.remove(pos as usize);
        list.insert(0, byte);
    }

    result
}

/// Compute the zero-run statistics of MTF output.
///
/// Returns `(zero_count, total_count)`. A higher zero ratio indicates
/// better BWT clustering.
pub fn zero_ratio(mtf_data: &[u8]) -> (usize, usize) {
    let zeros = mtf_data.iter().filter(|&&b| b == 0).count();
    (zeros, mtf_data.len())
}
