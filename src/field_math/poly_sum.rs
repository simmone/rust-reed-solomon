use crate::field_math::poly_to_items::poly_to_items;

pub fn poly_sum(poly: &str) -> u32 {
    let mut sum = 0;

    for pitem in poly_to_items(poly) {
        sum = sum + (pitem.coe * 2u32.pow(pitem.x_index));
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly_sum() {
        assert_eq!(1, poly_sum("1"));
        assert_eq!(2, poly_sum("x1"));
        assert_eq!(3, poly_sum("x1+1"));
        assert_eq!(4, poly_sum("x2"));
        assert_eq!(5, poly_sum("x2+1"));
        assert_eq!(6, poly_sum("x2+x1"));
        assert_eq!(7, poly_sum("x2+x1+1"));
        assert_eq!(4, poly_sum("4"));
        assert_eq!(4, poly_sum("2x"));
        assert_eq!(9, poly_sum("4x+1"));
        assert_eq!(17, poly_sum("3x2+2x1+1"));
    }
}
