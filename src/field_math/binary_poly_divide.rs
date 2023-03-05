pub fn binary_poly_divide(dividend_poly: &str, divisor_poly: &str) -> String {
    String::from("")
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
