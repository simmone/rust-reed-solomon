use crate::field_math::galios_context::new_gs;
use crate::field_math::galios_context::GaliosContext;
use crate::field_math::poly_to_items::poly_to_items;

use std::str::FromStr;

pub fn galios_poly_divide_align(dividend_poly: &str, divisor_poly: &str, gs: &GaliosContext) -> String {
    let dividend_pitems = poly_to_items(dividend_poly);
    
    let divisor_pitems = poly_to_items(divisor_poly);

    let src_coe_n = divisor_pitems[0].coe;
    
    let src_coe_a = gs.galios_number_to_index_hash.get(&src_coe_n).unwrap();
    
    let src_coe_a_n = u32::from_str(&src_coe_a[1..]).unwrap();
    
    println!("src_coe_a: {src_coe_a}, src_coe_a_n: {src_coe_a_n}");

    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galios_poly_divide_align_4() {
        let gs = new_gs(4, "x4+x+1");

        assert_eq!("10x", galios_poly_divide_align("x4", "12x3", &gs));
        assert_eq!("6", galios_poly_divide_align("14x3", "12x3", &gs));
        assert_eq!("2x", galios_poly_divide_align("12x3", "6x2", &gs));
        assert_eq!("13", galios_poly_divide_align("8x2", "6x2", &gs));
        assert_eq!("2", galios_poly_divide_align("14x2", "7x2", &gs));
    }

    #[test]
    fn test_galios_poly_divide_align_8() {
        let gs = new_gs(8, "x8+x4+x3+x2+1");

        assert_eq!(
            "137x2",
            galios_poly_divide_align(
             "x16",
             "49x14+195x13+228x12+166x11+225x10+133x9+24x8+105x7+4x6+9x5+222x4+119x3+138x2+193x1+87x0",
            &gs))
    }
}
