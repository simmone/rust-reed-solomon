use crate::field_math::binary_poly_to_binary_string::binary_poly_to_binary_string;
use crate::field_math::binary_string_to_binary_poly::binary_string_to_binary_poly;

pub fn binary_poly_divide(dividend_poly: &str, divisor_poly: &str) -> String {
    let dividend_poly_bits = binary_poly_to_binary_string(dividend_poly);
    let divisor_poly_bits = binary_poly_to_binary_string(divisor_poly);
    let divisor_bits_length = divisor_poly_bits.len();

    let mut loop_bits: String = String::from(dividend_poly_bits);

    let divisor_poly_bits_to_u32 = u32::from_str_radix(&divisor_poly_bits, 2).unwrap();

    let result = loop {
        if loop_bits.len() >= divisor_bits_length {
            let head_loop_bits = &loop_bits[0..divisor_bits_length];

            let head_loop_bits_to_u32 = u32::from_str_radix(head_loop_bits, 2).unwrap();

            let bitwise_result = head_loop_bits_to_u32 ^ divisor_poly_bits_to_u32;

            let bitwise_result_binary = format!("{bitwise_result:b}");

            loop_bits = format!(
                "{}{}",
                bitwise_result_binary,
                &loop_bits[divisor_bits_length..]
            );

            continue;
        } else {
            break binary_string_to_binary_poly(&loop_bits);
        }
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_poly_divide() {
        assert_eq!("x3+x+1", binary_poly_divide("x6+x5+x4+x", "x4+x+1"));
        assert_eq!("x", binary_poly_divide("x", "x4+x+1"));
    }
}
