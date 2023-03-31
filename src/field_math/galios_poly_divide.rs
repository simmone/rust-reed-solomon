use crate::field_math::galios_context::new_gs_from_poly;
use crate::field_math::galios_context::GaliosContext;
use crate::field_math::galios_poly_add::galios_poly_add;
use crate::field_math::galios_poly_divide_align::galios_poly_divide_align;
use crate::field_math::galios_poly_multiply::galios_poly_multiply;
use crate::field_math::items_to_poly::items_to_poly;
use crate::field_math::poly_to_items::poly_to_items;

pub fn galios_poly_divide(
    dividend_poly: &str,
    divisor_poly: &str,
    gs: &GaliosContext,
) -> (String, String) {
    let divisor_pitems = poly_to_items(divisor_poly);

    let divisor_index = divisor_pitems[0].x_index;

    //    println!("divisor_index: {divisor_index}");

    let mut remainder = String::from(dividend_poly);

    let mut quotient = String::from("");

    let mut last_op = String::from("");

    loop {
        //        println!("remainder: {remainder}");

        if remainder != "" {
            let remainder_pitems = poly_to_items(&remainder);

            let remainder_index = remainder_pitems[0].x_index;

            let remainder_coe = remainder_pitems[0].coe;

            //            println!("remainder_index: {remainder_index}, remainder_coe: {remainder_coe}");

            if remainder_coe == 0 {
                remainder = items_to_poly(remainder_pitems[0..].to_vec());

                continue;
            } else {
                if remainder_index >= divisor_index {
                    //                    println!("remainder: {remainder}, divisor: {divisor_poly}");

                    let loop_align_factor = galios_poly_divide_align(&remainder, divisor_poly, gs);
                    //                    println!("loop_align_factor: {loop_align_factor}");

                    let loop_divisor_multiply_factor = galios_poly_multiply(
                        vec![divisor_poly, &loop_align_factor],
                        &gs.field_generator_poly,
                    );
                    //                    println!("loop_divisor_multiply_factor: {loop_divisor_multiply_factor}");

                    let loop_substract =
                        galios_poly_add(vec![&remainder, &loop_divisor_multiply_factor]);
                    //                    println!("loop_substract: {loop_substract}");

                    remainder = loop_substract;

                    quotient = format!("{quotient}{last_op}{loop_align_factor}");

                    last_op = "+".to_string();

                    continue;
                } else {
                    break (quotient, remainder);
                }
            }
        } else {
            break (quotient, remainder);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galios_poly_divide_4() {
        let gs = new_gs_from_poly(4, "x4+x+1");

        let result1 = galios_poly_divide("12x3+4x2+3x+15", "6x2+6x1+4", &gs);
        assert_eq!("2x+13", result1.0);
        assert_eq!("3x+14", result1.1);

        let result2 = galios_poly_divide("7x2+7x+9", "9", &gs);
        assert_eq!("14x2+14x+1", result2.0);
        assert_eq!("", result2.1);

        let result3 = galios_poly_divide("3x1+14", "9", &gs);
        assert_eq!("6x+15", result3.0);
        assert_eq!("", result3.1);

        let result4 = galios_poly_divide("15x9+6", "14", &gs);
        assert_eq!("2x9+10", result4.0);
        assert_eq!("", result4.1);

        let result5 = galios_poly_divide("15x2+6", "14", &gs);
        assert_eq!("2x2+10", result5.0);
        assert_eq!("", result5.1);

        let result6 = galios_poly_divide("6x+15", "14", &gs);
        assert_eq!("10x+2", result6.0);
        assert_eq!("", result6.1);
    }

    #[test]
    fn test_galios_poly_divide_8() {
        let gs = new_gs_from_poly(8, "x8+x4+x3+x2+1");

        let result = galios_poly_divide(
            "11x33+94x32+132x31+202x30+153x29+38x28+98x27+136x26+183x25+101x24+175x23+127x22+122x21+33x20+121x19+118x18+133x17+96x16+6x15+94x14+173x13+232x12+200x11+48x10+3x9+219x8+224x7+239x6+216x5+107x4+66x3+151x2+44x1+6x0",
            "36x32+37x31+13x30+230x29+157x28+251x27+89x26+97x25+221x24+53x23+142x22+10x21+202x20+78x19+105x18+212x17+173x16+81x15+226x14+58x13+142x12+94x11+216x10+37x9+170x8+227x7+216x6+51x5+65x4+104x3+57x2+150x1+46x0",
            &gs);
        assert_eq!("135x+225", result.0);
        assert_eq!("90x31+37x30+110x29+211x28+242x27+150x26+94x25+229x24+231x23+222x22+79x21+189x20+15x18+223x17+148x16+99x15+33x14+35x13+173x12+129x11+106x10+246x9+160x8+174x7+24x6+252x5+83x4+244x3+243x2+107x+80",
                   result.1);
    }
}
