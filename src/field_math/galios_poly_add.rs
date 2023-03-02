use crate::field_math::pitem::Pitem;

pub fn galios_poly_add(_polys: Vec<&str>) -> String {
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galios_poly_add() {
        assert_eq!("", galios_poly_add(vec!["9" "9"]));
        assert_eq!("x+8", galios_poly_add(vec!["x+8"]));
        assert_eq!("x+8", galios_poly_add(vec!["3x1", "2x1+8"]));
        assert_eq!("3x2+2x+13", galios_poly_add(vec!["3x2+2x+8", "5"]));
        assert_eq!("x2+3x+2", galios_poly_add(vec!["x2+2x", "1x+2"]));
        assert_eq!(
            "x3+7x2+14x+8",
            galios_poly_add(vec!["x3", "4x2", "3x2+12x", "2x+8"])
        );
        assert_eq!(
            "14x3+13x2+12x",
            galios_poly_add(vec!["x4", "x4+14x3", "13x2+12x"])
        );
        assert_eq!(
            "13x2+12x",
            galios_poly_add(vec!["x4", "x4", "x3", "x3", "13x2+12x"])
        );
        assert_eq!(
            "14x3+13x2+12x",
            galios_poly_add(vec!["1x4", "x4+14x3+13x2+12x1"])
        );
        assert_eq!(
            "14x3+13x2+12x",
            galios_poly_add(vec!["x4", "x4+14x3+13x2+12x1"])
        );
        assert_eq!("154x14+220x13+253x12+68x11+10x10+124x9+102x8+201x7+53x6+167x5+140x4+96x3+91x2+50x+66",
                   galios_poly_add(
                       vec!["90x16+66x15+95x14+186x13+120x12+50x11+156x10+158x9+140x8+174x7+108x6+152x5+41x4+88x3+169x2+200x",
                            "90x16+66x15+197x14+102x13+133x12+118x11+150x10+226x9+234x8+103x7+89x6+63x5+165x4+56x3+242x2+250x+66"]));
    }
}
