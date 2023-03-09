use crate::field_math::galios_context::GaliosContext;

pub fn galios_poly_multiply(polys: Vec<&str>, gs: &GaliosConext) -> String {
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galios_poly_multiply() {
        let gs =
            GaliosContext {
                field_generator_poly: String::from("x4+x+1"),
            };
        
        assert_eq!("x", galios_poly_multiply(vec!["1", "x"], &gs));
        assert_eq!("x6+x5+x4+x", galios_poly_multiply(vec!["x3+x", "x3+x2+1"], &gs));
        assert_eq!("x3+x2+x1+1", galios_poly_multiply(vec!["x2+x1+1", "x"], &gs));
        assert_eq!("x4+x3+x2", galios_poly_multiply(vec!["x2+x1+1", "x2"], &gs));
        assert_eq!("x2+3x+2", galios_poly_multiply(vec!["x+1", "x+2"], &gs));
        assert_eq!("x3+7x2+14x+8", galios_poly_multiply(vec!["x2+3x+2", "x+4"], &gs));
        assert_eq!("x4+15x3+3x2+x+12", galios_poly_multiply(vec!["x3+7x2+14x+8", "x+8"], &gs));
        assert_eq!("x6+x5+x4+x", galios_poly_multiply(vec!["x3+x", "x3+x2+1"], &gs));
        assert_eq!("x8+x7+x6+x3", galios_poly_multiply(vec!["x4+x2+x+1", "x3+x4"], &gs));
        assert_eq!("x4+15x3+3x2+x+12", galios_poly_multiply(vec!["x+1", "x+2", "x+4", "x+8"], &gs));
    }
}
