//! # reed_solomon_cx
//!
//! reed_solomon_cx is a implementation of [Reed Solomon error correction](https://en.wikipedia.org/wiki/Reed%E2%80%93Solomon_error_correction).
//!
//! rs_encode to generate parity length data, then use rs_decode can recover parity_length/2's errors.
//!
//! # Examples
//!
//! ```
//! // use rs_encode to generate 10 parity data
//!        assert_eq!(
//!            vec![196, 35, 39, 119, 235, 215, 231, 226, 93, 23],
//!            reed_solomon_cx::rs_encode::rs_encode(
//!                vec![32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17],
//!                10
//!            )
//!        );
//!
//! // now get origin data + parity data =
//! // [32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35, 39, 119, 235, 215, 231, 226, 93, 23]
//! // it has 10 parity length, so i can fix 5 errors at most, so make 5 errors:
//! // [32, 91, 12, 120, 209, 114, 221, 77, 67, 64, 235, 17, 236, 17, 236, 17, 197, 35, 39, 119, 235, 215, 232, 226, 93, 23]
//!
//! // use rs_decode to recover right data:
//!        assert_eq!(
//!            vec![32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35, 39, 119, 235, 215, 231, 226, 93, 23],
//!          reed_solomon_cx::rs_decode::rs_decode(
//!            vec![32, 91, 12, 120, 209, 114, 221, 77, 67, 64, 235, 17, 236, 17, 236, 17, 197, 35, 39, 119, 235, 215, 232, 226, 93, 23],
//!            10
//!          )
//!        );
//!
//! ```
pub mod rs_encode;

pub mod rs_decode;

mod field_math;
