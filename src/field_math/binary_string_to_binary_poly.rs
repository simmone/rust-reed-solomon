use std::fmt::Write;

pub fn binary_string_to_binary_poly(binary_str: &str) -> String {
    let mut result = String::from("");
    
    let mut binary_chars = binary_str.chars();
    
    let mut loop_index = binary_chars.clone().count();
    
    let mut last_op = "";
    
    loop {
        loop_index -= 1;

        match binary_chars.next() {
            None => break,
            Some('1') => {
                match loop_index {
                    1 => write!(result, "{}x", last_op).unwrap(),
                    0 => write!(result, "{}1", last_op).unwrap(),
                    _ => write!(result, "{}x{}", last_op, loop_index).unwrap(),
                }
            },
            _ => continue,
        };
        
        last_op = "+";
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_string_to_binary_poly() {
        assert_eq!("x6+x5+x4+x1", binary_string_to_binary_poly("1110010"));
        assert_eq!("x4+x3+x2+x+1", binary_string_to_binary_poly("10011"));
        assert_eq!("x", binary_string_to_binary_poly("10"));
    }
}
