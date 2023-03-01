use crate::field_math::poly_to_items::poly_to_items;

pub fn binary_poly_to_binary_string(poly: &str) -> String {
    let indexes: Vec<u32> = poly_to_items(poly)
        .iter()
        .map(|item| item.x_index)
        .collect();

    let mut loop_index = indexes[0];

    let mut result = String::from("");

    loop {
        result = format!(
            "{result}{}",
            if indexes.contains(&loop_index) { 1 } else { 0 }
        );

        if loop_index == 0 {
            break;
        } else {
            loop_index -= 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_poly_to_binary_string() {
        assert_eq!("1110010", binary_poly_to_binary_string("x6+x5+x4+x1"));
        assert_eq!("10011", binary_poly_to_binary_string("x4+x1+1"));
    }
}
