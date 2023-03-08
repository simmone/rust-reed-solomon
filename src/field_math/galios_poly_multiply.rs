use crate::field_math::binary_poly_multiply::poly_multiply_basic;
use crate::field_math::galios_num_multiply::galios_num_multiply;

pub fn galios_poly_multiply(polys: Vec<&str>, field_generator_poly: &str) -> String {
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galios_poly_multiply() {
        let field_generator_poly: &str = "x4+x+1";
        
        assert_eq!("x", galios_poly_multiply(vec!["1", "x"], field_generator_poly));
        assert_eq!("x6+x5+x4+x", galios_poly_multiply(vec!["x3+x", "x3+x2+1"], field_generator_poly));
        assert_eq!("x3+x2+x1+1", galios_poly_multiply(vec!["x2+x1+1", "x"], field_generator_poly));
        assert_eq!("x4+x3+x2", galios_poly_multiply(vec!["x2+x1+1", "x2"], field_generator_poly));
        assert_eq!("x2+3x+2", galios_poly_multiply(vec!["x+1", "x+2"], field_generator_poly));
        assert_eq!("x3+7x2+14x+8", galios_poly_multiply(vec!["x2+3x+2", "x+4"], field_generator_poly));
        assert_eq!("x4+15x3+3x2+x+12", galios_poly_multiply(vec!["x3+7x2+14x+8", "x+8"], field_generator_poly));
        assert_eq!("x6+x5+x4+x", galios_poly_multiply(vec!["x3+x", "x3+x2+1"], field_generator_poly));
        assert_eq!("x8+x7+x6+x3", galios_poly_multiply(vec!["x4+x2+x+1", "x3+x4"], field_generator_poly));
        assert_eq!("x4+15x3+3x2+x+12", galios_poly_multiply(vec!["x+1", "x+2", "x+4", "x+8"], field_generator_poly));
    }
}
