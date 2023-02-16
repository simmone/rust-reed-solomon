pub fn number_to_binary_poly(num: i32) -> String {
    for (index, ch) in format!("{num:b}").chars().enumerate() {
        println!("index: {index}, ch: {ch}");
    }
    
    num.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_binary_poly() {
        assert_eq!("x3+x", number_to_binary_poly(10));
        assert_eq!("x3+x2+1", number_to_binary_poly(13));
        assert_eq!("x3+x2", number_to_binary_poly(12));
        assert_eq!("1", number_to_binary_poly(1));
        assert_eq!("x", number_to_binary_poly(2));
    }
}
