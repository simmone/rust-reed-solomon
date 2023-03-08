pub fn number_to_binary_poly(num: u32) -> String {
    let binary_string = format!("{num:b}");

    let b_chars = binary_string.chars();
    let max_index = b_chars.clone().count() - 1;

    let mut result_str = String::new();
    let mut last_op = "";
    for (index, ch) in b_chars.enumerate() {
        if ch == '0' {
            continue;
        } else {
            result_str.push_str(last_op);
            last_op = "+";
            let x_index = max_index - index;
            if x_index == 0 {
                result_str.push('1');
            } else if x_index == 1 {
                result_str.push('x');
            } else {
                result_str.push('x');
                result_str.push_str(&(max_index - index).to_string());
            }
        }
    }

    result_str
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
