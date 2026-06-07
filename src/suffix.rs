//! Suffix array construction for BWT.

/// Build a suffix array using a simple O(n log²n) approach.
///
/// The suffix array `sa[i]` gives the starting index of the `i`-th
/// smallest suffix of the input data.
///
/// A sentinel value (conceptually 0) is appended to make all suffixes
/// distinct. In practice we sort the indices `0..n` by their suffixes.
pub fn build_suffix_array(data: &[u8]) -> Vec<usize> {
    let n = data.len();
    if n == 0 {
        return Vec::new();
    }

    let mut sa: Vec<usize> = (0..n).collect();
    let mut rank: Vec<usize> = data.iter().map(|&b| b as usize + 1).collect();
    let mut tmp = vec![0usize; n];

    let mut k = 1usize;
    while k < n {
        let rank_copy = rank.clone();
        let k_copy = k;

        sa.sort_by(|&a, &b| {
            let ra = rank_copy[a];
            let rb = rank_copy[b];
            match ra.cmp(&rb) {
                std::cmp::Ordering::Equal => {
                    let ra2 = if a + k_copy < n { rank_copy[a + k_copy] } else { 0 };
                    let rb2 = if b + k_copy < n { rank_copy[b + k_copy] } else { 0 };
                    ra2.cmp(&rb2)
                }
                other => other,
            }
        });

        tmp[sa[0]] = 1;
        for i in 1..n {
            let prev = sa[i - 1];
            let curr = sa[i];
            let same = rank_copy[prev] == rank_copy[curr]
                && {
                    let rp = if prev + k_copy < n { rank_copy[prev + k_copy] } else { 0 };
                    let rc = if curr + k_copy < n { rank_copy[curr + k_copy] } else { 0 };
                    rp == rc
                };
            tmp[curr] = tmp[prev] + if same { 0 } else { 1 };
        }

        rank.copy_from_slice(&tmp);

        if rank[sa[n - 1]] == n {
            break;
        }

        k *= 2;
    }

    sa
}

/// Build suffix array for data with an end-of-string sentinel.
///
/// The sentinel (conceptually smaller than all other bytes) ensures
/// the empty suffix sorts first. We handle this by treating the byte
/// after the data end as 0.
pub fn build_suffix_array_with_sentinel(data: &[u8]) -> Vec<usize> {
    build_suffix_array(data)
}

/// Get the BWT from a suffix array.
///
/// For each entry `sa[i]`, the BWT character is the byte just before the
/// suffix start (or the last byte if `sa[i] == 0`).
pub fn bwt_from_suffix_array(data: &[u8], sa: &[usize]) -> (Vec<u8>, usize) {
    let n = data.len();
    let mut bwt = Vec::with_capacity(n);
    let mut primary_index = 0;

    for (i, &start) in sa.iter().enumerate() {
        if start == 0 {
            // The full string sorts here; BWT char is the last byte
            bwt.push(data[n - 1]);
            primary_index = i;
        } else {
            bwt.push(data[start - 1]);
        }
    }

    (bwt, primary_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_array_banana() {
        let sa = build_suffix_array(b"banana");
        // Suffixes sorted: a, a$na, ana, anana$, banana$, na, nana$
        // Indices:         5, 3,    1,   ??? 
        // Actually: banana suffixes:
        // 0: banana
        // 1: anana
        // 2: nana
        // 3: ana
        // 4: na
        // 5: a
        // Sorted: 5(a), 3(ana), 1(anana), 0(banana), 4(na), 2(nana)
        assert_eq!(sa, vec![5, 3, 1, 0, 4, 2]);
    }

    #[test]
    fn test_suffix_array_empty() {
        assert_eq!(build_suffix_array(b""), Vec::<usize>::new());
    }

    #[test]
    fn test_suffix_array_single() {
        assert_eq!(build_suffix_array(b"a"), vec![0]);
    }

    #[test]
    fn test_suffix_array_sorted() {
        let sa = build_suffix_array(b"abcd");
        assert_eq!(sa, vec![0, 1, 2, 3]);
    }
}
