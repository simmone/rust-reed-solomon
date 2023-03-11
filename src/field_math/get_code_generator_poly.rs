pub fn get_code_generator_poly(parity_length: u32) -> String {
    parity_length.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_code_generator_poly() {
        assert_eq!(1, get_code_generator_poly("1"));
    }
}
