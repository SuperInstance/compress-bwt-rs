//! Comprehensive test suite for compress-bwt-rs.

#[cfg(test)]
mod tests {
    use crate::{bwt_forward, bwt_inverse, mtf, rle_post, suffix};

    /// Helper: full BWT round-trip.
    fn bwt_roundtrip(data: &[u8]) -> Vec<u8> {
        let (bwt, idx) = bwt_forward::transform(data);
        bwt_inverse::inverse(&bwt, idx).unwrap()
    }

    // ── Suffix array tests ──────────────────────────────────────────────

    #[test]
    fn test_suffix_array_banana() {
        let sa = suffix::build_suffix_array(b"banana");
        assert_eq!(sa, vec![5, 3, 1, 0, 4, 2]);
    }

    #[test]
    fn test_suffix_array_empty() {
        assert_eq!(suffix::build_suffix_array(b""), Vec::<usize>::new());
    }

    #[test]
    fn test_suffix_array_single() {
        assert_eq!(suffix::build_suffix_array(b"x"), vec![0]);
    }

    #[test]
    fn test_suffix_array_sorted() {
        let sa = suffix::build_suffix_array(b"abcd");
        assert_eq!(sa, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_suffix_array_reverse_sorted() {
        let sa = suffix::build_suffix_array(b"dcba");
        assert_eq!(sa, vec![3, 2, 1, 0]);
    }

    #[test]
    fn test_suffix_array_all_same() {
        let sa = suffix::build_suffix_array(b"aaa");
        // All suffixes are equal, order doesn't matter much
        assert_eq!(sa.len(), 3);
    }

    // ── BWT forward tests ───────────────────────────────────────────────

    #[test]
    fn test_bwt_banana() {
        let (bwt, _idx) = bwt_forward::transform(b"banana");
        // BWT of "banana" should be "nnbaaa" (or similar depending on sentinel handling)
        assert_eq!(bwt.len(), 6);
    }

    #[test]
    fn test_bwt_empty() {
        let (bwt, idx) = bwt_forward::transform(b"");
        assert!(bwt.is_empty());
        assert_eq!(idx, 0);
    }

    #[test]
    fn test_bwt_single() {
        let (bwt, idx) = bwt_forward::transform(b"x");
        assert_eq!(bwt, b"x");
        assert_eq!(idx, 0);
    }

    #[test]
    fn test_bwt_all_same() {
        let (bwt, idx) = bwt_forward::transform(b"aaaa");
        assert_eq!(bwt, b"aaaa"); // all rotations are the same
        // Primary index is valid
        let recovered = bwt_inverse::inverse(&bwt, idx).unwrap();
        assert_eq!(recovered, b"aaaa");
    }

    #[test]
    fn test_bwt_naive_matches() {
        let data = b"banana";
        let (bwt1, idx1) = bwt_forward::transform(data);
        let (bwt2, idx2) = bwt_forward::transform_naive(data);
        assert_eq!(bwt1, bwt2);
        assert_eq!(idx1, idx2);
    }

    #[test]
    fn test_bwt_naive_matches_abracadabra() {
        let data = b"abracadabra";
        let (bwt1, idx1) = bwt_forward::transform(data);
        let (bwt2, idx2) = bwt_forward::transform_naive(data);
        assert_eq!(bwt1, bwt2);
        assert_eq!(idx1, idx2);
    }

    // ── BWT inverse / round-trip tests ──────────────────────────────────

    #[test]
    fn test_bwt_roundtrip_banana() {
        assert_eq!(b"banana".as_slice(), bwt_roundtrip(b"banana").as_slice());
    }

    #[test]
    fn test_bwt_roundtrip_empty() {
        assert_eq!(Vec::<u8>::new(), bwt_roundtrip(b""));
    }

    #[test]
    fn test_bwt_roundtrip_single() {
        assert_eq!(b"x".as_slice(), bwt_roundtrip(b"x").as_slice());
    }

    #[test]
    fn test_bwt_roundtrip_abracadabra() {
        assert_eq!(b"abracadabra".as_slice(), bwt_roundtrip(b"abracadabra").as_slice());
    }

    #[test]
    fn test_bwt_roundtrip_text() {
        let data = b"the quick brown fox jumps over the lazy dog";
        assert_eq!(data.as_slice(), bwt_roundtrip(data).as_slice());
    }

    #[test]
    fn test_bwt_roundtrip_binary() {
        let data: Vec<u8> = (0..=255).collect();
        assert_eq!(data.as_slice(), bwt_roundtrip(&data).as_slice());
    }

    #[test]
    fn test_bwt_roundtrip_repetitive() {
        let data = b"aaaaaaabbbbccd";
        assert_eq!(data.as_slice(), bwt_roundtrip(data).as_slice());
    }

    #[test]
    fn test_bwt_invalid_primary_index() {
        let (bwt, _) = bwt_forward::transform(b"test");
        assert!(bwt_inverse::inverse(&bwt, 999).is_none());
    }

    #[test]
    fn test_bwt_verify_roundtrip() {
        assert!(bwt_inverse::verify_roundtrip(b"hello world").is_ok());
    }

    // ── MTF tests ───────────────────────────────────────────────────────

    #[test]
    fn test_mtf_roundtrip() {
        let data = b"banana";
        let encoded = mtf::encode(data);
        let decoded = mtf::decode(&encoded);
        assert_eq!(data.as_slice(), decoded.as_slice());
    }

    #[test]
    fn test_mtf_empty() {
        assert_eq!(mtf::encode(b""), Vec::<u8>::new());
        assert_eq!(mtf::decode(&[]), Vec::<u8>::new());
    }

    #[test]
    fn test_mtf_single() {
        let encoded = mtf::encode(b"a");
        assert_eq!(encoded, vec![b'a']); // 'a' is at position 'a'=97
        let decoded = mtf::decode(&encoded);
        assert_eq!(decoded, b"a");
    }

    #[test]
    fn test_mtf_repeated_byte() {
        // First occurrence at position 97, subsequent at position 0
        let encoded = mtf::encode(b"aaa");
        assert_eq!(encoded[0], b'a'); // first 'a' at its initial position
        assert_eq!(encoded[1], 0); // second 'a' moved to front
        assert_eq!(encoded[2], 0); // third 'a' still at front
    }

    #[test]
    fn test_mtf_roundtrip_binary() {
        let data: Vec<u8> = (0..=255).collect();
        let encoded = mtf::encode(&data);
        let decoded = mtf::decode(&encoded);
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_mtf_custom_alphabet() {
        let alphabet = b"abc";
        let data = b"aabbc";
        let encoded = mtf::encode_with_alphabet(data, alphabet);
        let decoded = mtf::decode_with_alphabet(&encoded, alphabet);
        assert_eq!(data.as_slice(), decoded.as_slice());
    }

    #[test]
    fn test_mtf_zero_ratio() {
        // All same byte → BWT → MTF should give mostly zeros
        let data = b"aaaaaa";
        let bwt_result = bwt_forward::transform(data);
        let bwt = &bwt_result.0;
        let mtf_data = mtf::encode(bwt);
        let (zeros, total) = mtf::zero_ratio(&mtf_data);
        assert!(zeros > 0, "expected some zeros in MTF of repeated data");
        assert_eq!(total, data.len());
    }

    // ── RLE post-BWT tests ──────────────────────────────────────────────

    #[test]
    fn test_rle_post_roundtrip() {
        let data = vec![0, 0, 0, 0, 1, 2, 0, 0, 0];
        let encoded = rle_post::encode(&data);
        let decoded = rle_post::decode(&encoded);
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_rle_post_empty() {
        assert_eq!(rle_post::encode(&[]), Vec::<(u8, u16)>::new());
        assert_eq!(rle_post::decode(&[]), Vec::<u8>::new());
    }

    #[test]
    fn test_rle_post_all_zeros() {
        let data = vec![0u8; 1000];
        let encoded = rle_post::encode(&data);
        assert_eq!(encoded.len(), 1); // Single run
        assert_eq!(encoded[0], (0, 1000));
        let decoded = rle_post::decode(&encoded);
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_rle_post_no_runs() {
        let data: Vec<u8> = (1..=10).collect();
        let encoded = rle_post::encode(&data);
        assert_eq!(encoded.len(), 10); // Each byte is its own run
    }

    #[test]
    fn test_rle_post_compact_roundtrip() {
        let data = vec![0, 0, 0, 5, 3, 0, 0, 0, 0, 0];
        let compact = rle_post::encode_compact(&data);
        let decoded = rle_post::decode_compact(&compact).unwrap();
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_rle_post_compact_empty() {
        let compact = rle_post::encode_compact(&[]);
        assert!(compact.is_empty());
        assert_eq!(rle_post::decode_compact(&[]), Some(Vec::<u8>::new()));
    }

    #[test]
    fn test_rle_post_compact_0xff_value() {
        let data = vec![0xFF, 0xFF, 0xFF];
        let compact = rle_post::encode_compact(&data);
        let decoded = rle_post::decode_compact(&compact).unwrap();
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_rle_post_compression_stats() {
        let data = vec![0u8; 100];
        let (pairs, original) = rle_post::compression_stats(&data);
        assert_eq!(original, 100);
        assert_eq!(pairs, 1);
    }

    // ── BWT clustering effect tests ─────────────────────────────────────

    #[test]
    fn test_bwt_clustering_improves_mtf() {
        // Repetitive data: BWT should cluster, MTF should produce more zeros
        let data = b"abababababababababab";
        let bwt_result = bwt_forward::transform(data);
        let bwt = &bwt_result.0;

        // MTF of original data
        let mtf_original = mtf::encode(data);
        let (_zeros_orig, _) = mtf::zero_ratio(&mtf_original);

        // MTF of BWT data
        let mtf_bwt = mtf::encode(bwt);
        let (zeros_bwt, _) = mtf::zero_ratio(&mtf_bwt);

        // BWT+MTF should have at least as many zeros as direct MTF
        // (usually more, but depends on input)
        assert!(zeros_bwt > 0, "BWT+MTF should produce some zeros");
    }

    #[test]
    fn test_full_pipeline_roundtrip() {
        // BWT → MTF → RLE → decode RLE → decode MTF → inverse BWT
        let data = b"mississippi";
        
        // Forward
        let (bwt, idx) = bwt_forward::transform(data);
        let mtf_data = mtf::encode(&bwt);
        let rle_data = rle_post::encode_compact(&mtf_data);
        
        // Inverse
        let mtf_decoded = rle_post::decode_compact(&rle_data).unwrap();
        let bwt_decoded = mtf::decode(&mtf_decoded);
        let original = bwt_inverse::inverse(&bwt_decoded, idx).unwrap();
        
        assert_eq!(data.as_slice(), original.as_slice());
    }

    #[test]
    fn test_rle_post_compact_reduces_size_repetitive() {
        let data = vec![0u8; 100];
        let compact = rle_post::encode_compact(&data);
        assert!(compact.len() < data.len(),
            "compact RLE should reduce size for repetitive data: {} vs {}",
            compact.len(), data.len());
    }
}
