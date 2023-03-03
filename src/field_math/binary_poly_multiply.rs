pub fn binary_poly_multiply(poly1: &str, poly2: &str) -> String {
    poly_multiply_basic(poly1, poly2, "+",  "*")
}

fn poly_multiply_basic(poly1: &str, poly2: &str, add_op: &str, multiply_op: &str) -> String {
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_poly_multiply() {
        assert_eq!("x6+x5+x4+x1", binary_poly_multiply("x3+x", "x3+x2+1"));
        assert_eq!("x", binary_poly_multiply("1", "x"));
    }
}
