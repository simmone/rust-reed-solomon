//! # reed_solomon_cx
//!
//! reed_solomon_cx is a implementation of [Reed Solomon error correction](https://en.wikipedia.org/wiki/Reed%E2%80%93Solomon_error_correction).
//!
//! rs_encode to generate parity length data, then use rs_decode can recover parity_length/2's errors.
mod rs_encode;

mod rs_decode;

mod field_math;
