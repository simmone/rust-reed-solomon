use crate::field_math::galios_context::GaliosContext;
use crate::field_math::items_to_poly::items_to_poly;
use crate::field_math::pitem::Pitem;
use crate::field_math::poly_to_items::poly_to_items;

use std::str::FromStr;

pub fn galios_poly_divide_align(
    dividend_poly: &str,
    divisor_poly: &str,
    gs: &GaliosContext,
) -> String {
    let dividend_pitems = poly_to_items(dividend_poly);

    let divisor_pitems = poly_to_items(divisor_poly);

    let src_coe_n = divisor_pitems[0].coe;

    let src_index_n = divisor_pitems[0].x_index;

    let src_coe_a = gs.galios_number_to_index_hash.get(&src_coe_n).unwrap();

    let src_coe_a_n = u32::from_str(&src_coe_a[1..]).unwrap();

    //    println!("src_coe_a: [{src_coe_a}][{src_coe_a_n}]");

    let dst_coe_n = dividend_pitems[0].coe;

    let dst_index_n = dividend_pitems[0].x_index;

    let dst_coe_a = gs.galios_number_to_index_hash.get(&dst_coe_n).unwrap();

    let dst_coe_a_n = u32::from_str(&dst_coe_a[1..]).unwrap();

    //    println!("dst_coe_a: [{dst_coe_a}][{dst_coe_a_n}]");

    let m2_1 = 2_u32.pow(gs.bit_width) - 1;

    let step1 = m2_1 + dst_coe_a_n - src_coe_a_n;

    //    println!("m2_1 + dst_coe_a_n - src_coe_a_n = {step1}");

    let step2 = step1 % m2_1;

    //    println!("step1 % {m2_1} = {step2}");

    let step2_a = format!("a{step2}");

    let step3 = gs.galios_index_to_number_hash.get(&step2_a).unwrap();

    //    println!("gs.galios_index_to_number_hash.get(&{step2_a}) = {step3}");

    let pitem = Pitem {
        x_index: dst_index_n - src_index_n,
        coe: *step3,
    };

    items_to_poly(vec![pitem])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galios_poly_divide_align_4() {
        let gs = crate::field_math::galios_context::new_gs_from_poly(4, "x4+x+1");

        assert_eq!("10x", galios_poly_divide_align("x4", "12x3", &gs));
        assert_eq!("6", galios_poly_divide_align("14x3", "12x3", &gs));
        assert_eq!("2x", galios_poly_divide_align("12x3", "6x2", &gs));
        assert_eq!("13", galios_poly_divide_align("8x2", "6x2", &gs));
        assert_eq!("2", galios_poly_divide_align("14x2", "7x2", &gs));
    }

    #[test]
    fn test_galios_poly_divide_align_8() {
        let gs = crate::field_math::galios_context::new_gs_from_poly(8, "x8+x4+x3+x2+1");

        assert_eq!(
            "137x2",
            galios_poly_divide_align(
             "x16",
             "49x14+195x13+228x12+166x11+225x10+133x9+24x8+105x7+4x6+9x5+222x4+119x3+138x2+193x1+87x0",
            &gs))
    }
}
