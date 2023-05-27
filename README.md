# reed_solomon_cx
# _a [Reed-Solomon](https://en.wikipedia.org/wiki/Reed%E2%80%93Solomon_error_correction) error correction algorithm implementation for Rust lang._

### basic usage: 
### 1. use rs_encode to generate parity length data.
### 2. use rs_decode recover parity_length/2’s errors(at most).

```rust
// use rs_encode to generate 10 parity data
       assert_eq!(
           vec![196, 35, 39, 119, 235, 215, 231, 226, 93, 23],
           reed_solomon_cx::rs_encode::rs_encode(
               vec![32, 91, 11, 120, 209, 114, 220, 77, 67, 64,
                    236,17, 236, 17, 236, 17],
               10
           )
       );

// now get origin data + parity data =
// [32, 91, 11, 120, 209, 114, 220, 77, 67, 64,
//  236, 17, 236, 17, 236, 17, 196, 35, 39, 119,
//  235, 215, 231, 226, 93, 23]
// it has 10 parity length, so i can fix 5 errors at most, so make 5 errors:
// [32, 91, 12, 120, 209, 114, 221, 77, 67, 64,
//  235, 17, 236, 17, 236, 17, 197, 35, 39, 119,
//  235, 215, 232, 226, 93, 23]

// use rs_decode to recover right data:
       assert_eq!(
           vec![32, 91, 11, 120, 209, 114, 220, 77, 67, 64,
                236, 17, 236, 17, 236, 17, 196, 35, 39, 119,
                235, 215, 231, 226, 93, 23],
         reed_solomon_cx::rs_decode::rs_decode(
           vec![32, 91, 12, 120, 209, 114, 221, 77, 67, 64,
                235, 17, 236, 17, 236, 17, 197, 35, 39, 119,
                235, 215, 232, 226, 93, 23],
           10
         )
       );
```
