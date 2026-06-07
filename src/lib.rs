//! # compress-bwt-rs
//!
//! A pure-Rust Burrows-Wheeler Transform library with supporting transforms.
//!
//! # Modules
//!
//! - [`bwt_forward`] — Forward BWT using suffix sorting.
//! - [`bwt_inverse`] — Inverse BWT to recover original data.
//! - [`mtf`] — Move-to-front transform for post-BWT entropy coding.
//! - [`rle_post`] — Run-length encoding optimized for post-BWT data.
//! - [`suffix`] — Suffix array construction for BWT.
//!
//! # Quick Start
//!
//! ```
//! use compress_bwt_rs::{bwt_forward, bwt_inverse};
//!
//! let data = b"banana";
//! let (bwt, primary_index) = bwt_forward::transform(data);
//! let recovered = bwt_inverse::inverse(&bwt, primary_index).unwrap();
//! assert_eq!(data.as_slice(), recovered.as_slice());
//! ```

pub mod bwt_forward;
pub mod bwt_inverse;
pub mod mtf;
pub mod rle_post;
pub mod suffix;

#[cfg(test)]
mod tests;
