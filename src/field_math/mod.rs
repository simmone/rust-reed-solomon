pub fn number_to_binary_poly(num: i32) -> String {
    let binary_string = format!("{num:b}");
    println!("binary_string: {binary_string}");

    let b_chars = binary_string.chars();
    let start_index = b_chars.clone().count();
    println!("start_index: {start_index}");

    for ch in b_chars {
        println!("ch: {ch}");
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
