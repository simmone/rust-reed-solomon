use crate::field_math::binary_poly_divide::binary_poly_divide;
use crate::field_math::binary_poly_multiply::binary_poly_multiply;
use crate::field_math::binary_poly_to_binary_string::binary_poly_to_binary_string;
use crate::field_math::galios_context::GaliosContext;
use crate::field_math::number_to_binary_poly::number_to_binary_poly;

pub fn galios_num_multiply(num1: u32, num2: u32, gs: &GaliosContext) -> u32 {
    if num1 == 0 || num2 == 0 {
        0
    } else {
        let num1_binary_poly = number_to_binary_poly(num1);

        let num2_binary_poly = number_to_binary_poly(num2);

        let binary_multiplied = binary_poly_multiply(&num1_binary_poly, &num2_binary_poly);

        let divide_field_generator =
            binary_poly_divide(&binary_multiplied, &gs.field_generator_poly);

        let binary_str = binary_poly_to_binary_string(&divide_field_generator);

        let result = u32::from_str_radix(&binary_str, 2).unwrap();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galios_num_multiply() {
        let gs = GaliosContext {
            field_generator_poly: String::from("x4+x+1"),
        };

        assert_eq!(1, galios_num_multiply(10, 12, &gs));
        assert_eq!(11, galios_num_multiply(10, 13, &gs));
        assert_eq!(11, galios_num_multiply(14, 14, &gs));
        assert_eq!(5, galios_num_multiply(14, 15, &gs));
        assert_eq!(10, galios_num_multiply(3, 6, &gs));
        assert_eq!(2, galios_num_multiply(1, 2, &gs));
        assert_eq!(0, galios_num_multiply(0, 0, &gs));
        assert_eq!(0, galios_num_multiply(0, 15, &gs));
        assert_eq!(0, galios_num_multiply(15, 0, &gs));
    }
}
