use crate::field_math::galios_context::GaliosContext;
use crate::field_math::galios_num_multiply::galios_num_multiply;
use crate::field_math::galios_poly_add::galios_poly_add;
use crate::field_math::items_to_poly::items_to_poly;
use crate::field_math::pitem::Pitem;
use crate::field_math::poly_to_items::poly_to_items;

pub fn galios_poly_multiply(polys: Vec<&str>, gs: &GaliosContext) -> String {
    let mut polys_iter = polys.iter();

    let mut poly_multiplicand = String::from(*polys_iter.next().unwrap());

    loop {
        match polys_iter.next() {
            Some(poly_multiplier) => {
                let multiplicand_pitems = poly_to_items(&poly_multiplicand);
                let multiplier_pitems = poly_to_items(poly_multiplier);

                let mut result_polys: Vec<String> = Vec::new();

                for multiplicand_pitem in &multiplicand_pitems {
                    let mut result_pitems = Vec::new();

                    for multiplier_pitem in &multiplier_pitems {
                        result_pitems.push(Pitem {
                            x_index: multiplier_pitem.x_index + multiplicand_pitem.x_index,
                            coe: galios_num_multiply(
                                multiplier_pitem.coe,
                                multiplicand_pitem.coe,
                                &gs,
                            ),
                        });
                    }

                    let poly_multiplied = items_to_poly(result_pitems);

                    result_polys.push(String::from(&poly_multiplied))
                }

                poly_multiplicand =
                    galios_poly_add(result_polys.iter().map(|poly| poly.as_str()).collect());

                continue;
            }
            None => break poly_multiplicand,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galios_poly_multiply() {
        let gs = GaliosContext {
            field_generator_poly: String::from("x4+x+1"),
        };

        assert_eq!("x", galios_poly_multiply(vec!["1", "x"], &gs));
        assert_eq!(
            "x6+x5+x4+x",
            galios_poly_multiply(vec!["x3+x", "x3+x2+1"], &gs)
        );
        assert_eq!("x3+x2+x", galios_poly_multiply(vec!["x2+x1+1", "x"], &gs));
        assert_eq!("x4+x3+x2", galios_poly_multiply(vec!["x2+x1+1", "x2"], &gs));
        assert_eq!("x2+3x+2", galios_poly_multiply(vec!["x+1", "x+2"], &gs));
        assert_eq!(
            "x3+7x2+14x+8",
            galios_poly_multiply(vec!["x2+3x+2", "x+4"], &gs)
        );
        assert_eq!(
            "x4+15x3+3x2+x+12",
            galios_poly_multiply(vec!["x3+7x2+14x+8", "x+8"], &gs)
        );
        assert_eq!(
            "x6+x5+x4+x",
            galios_poly_multiply(vec!["x3+x", "x3+x2+1"], &gs)
        );
        assert_eq!(
            "x8+x7+x6+x3",
            galios_poly_multiply(vec!["x4+x2+x+1", "x3+x4"], &gs)
        );
        assert_eq!(
            "x4+15x3+3x2+x+12",
            galios_poly_multiply(vec!["x+1", "x+2", "x+4", "x+8"], &gs)
        );
    }
}
