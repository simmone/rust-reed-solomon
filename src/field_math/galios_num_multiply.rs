use crate::field_math::number_to_binary_poly::number_to_binary_poly;
use crate::field_math::binary_poly_multiply::binary_poly_multiply;
use crate::field_math::binary_poly_divide::binary_poly_divide;

pub fn galios_num_multiply(num1: u32, num2: u32) -> u32 {
    if num1 == 0 || num2 == 0 {
        0
    } else {
        let num1_binary_poly = number_to_binary_poly(num1);

        let num2_binary_poly = number_to_binary_poly(num2);

        let binary_multiplied = binary_poly_multiply(num1_binary_poly, num2_binary_poly);
        
        let divide_field_generator = binary_poly_divide(binary_multiplied, field_generator_poly);
        
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galios_num_multiply() {
        let field_generator_poly = "x4+x+1";
        
        assert_eq!(1, galios_num_multiply(10, 12));
        assert_eq!(11, galios_num_multiply(10, 13));
        assert_eq!(11, galios_num_multiply(14, 14));
        assert_eq!(5, galios_num_multiply(14, 15));
        assert_eq!(10, galios_num_multiply(3, 6));
        assert_eq!(2, galios_num_multiply(1, 2));
        assert_eq!(0, galios_num_multiply(0, 0));
        assert_eq!(0, galios_num_multiply(0, 15));
        assert_eq!(0, galios_num_multiply(15, 0));
    }
}
