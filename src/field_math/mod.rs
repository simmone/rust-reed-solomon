pub fn number_to_binary_poly(num: i32) -> &'static str {
    "this is number_to_binary_poly"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_binary_poly() {
        assert_eq!(number_to_binary_poly(10), "x3+x");
        assert_eq!(number_to_binary_poly(13), "x3+x2+1");
        assert_eq!(number_to_binary_poly(12), "x3+x2");
        assert_eq!(number_to_binary_poly(1), "1");
        assert_eq!(number_to_binary_poly(2), "x");
    }
}
