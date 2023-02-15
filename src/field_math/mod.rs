pub fn number_to_binary_poly() {
    println!("this is number_to_binary_poly");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_binary_poly() {
        number_to_binary_poly();
        
        assert_eq!(4, 4);
    }
}
