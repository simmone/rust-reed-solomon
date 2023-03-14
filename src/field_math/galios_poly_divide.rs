use crate::field_math::galios_context::new_gs;
use crate::field_math::galios_context::GaliosContext;

pub fn galios_poly_divide(
    dividend_poly: &str,
    divisor_poly: &str,
    gs: &GaliosContext,
) -> (String, String) {
    (String::from(""), String::from(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galios_poly_divide_4() {
        let gs = new_gs(4, "x4+x+1");
        
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
        let gs = new_gs(8, "x8+x4+x3+x2+1");

        let result = galios_poly_divide(
            "11x33+94x32+132x31+202x30+153x29+38x28+98x27+136x26+183x25+101x24+175x23+127x22+122x21+33x20+121x19+118x18+133x17+96x16+6x15+94x14+173x13+232x12+200x11+48x10+3x9+219x8+224x7+239x6+216x5+107x4+66x3+151x2+44x1+6x0",
            "36x32+37x31+13x30+230x29+157x28+251x27+89x26+97x25+221x24+53x23+142x22+10x21+202x20+78x19+105x18+212x17+173x16+81x15+226x14+58x13+142x12+94x11+216x10+37x9+170x8+227x7+216x6+51x5+65x4+104x3+57x2+150x1+46x0",
            &gs);
        assert_eq!("135x+225", result.0);
        assert_eq!("90x31+37x30+110x29+211x28+242x27+150x26+94x25+229x24+231x23+222x22+79x21+189x20+15x18+223x17+148x16+99x15+33x14+35x13+173x12+129x11+106x10+246x9+160x8+174x7+24x6+252x5+83x4+244x3+243x2+107x+80",
                   result.1);
    }
}
