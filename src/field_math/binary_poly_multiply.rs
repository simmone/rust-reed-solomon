use crate::field_math::pitem::Pitem;
use crate::field_math::poly_to_items::poly_to_items;
use crate::field_math::items_to_poly::items_to_poly;
use crate::field_math::galios_poly_add::galios_poly_add;
use crate::field_math::galios_num_multiply::galios_num_multiply;

pub fn binary_poly_multiply(poly1: &str, poly2: &str) -> String {
    poly_multiply_basic(poly1, poly2, "+",  "*")
}

pub fn poly_multiply_basic(poly_multiplicand: &str, poly_multiplier: &str, add_op: &str, multiply_op: &str) -> String {
    let multiplicand_pitems = poly_to_items(poly_multiplicand);
    let multiplier_pitems = poly_to_items(poly_multiplier);
    
    let mut result_polys: Vec<String> = Vec::new();

    for multiplicand_pitem in &multiplicand_pitems {

        let mut result_pitems = Vec::new();

        for multiplier_pitem in &multiplier_pitems {
            result_pitems.push(
                Pitem {
                    x_index: match add_op {
                        "+" => multiplier_pitem.x_index + multiplicand_pitem.x_index,
                        _ => 0,
                    },
                    coe: match multiply_op {
                        "*" => multiplier_pitem.coe * multiplicand_pitem.coe,
                        _ => 0,
                    }
                }
            );
        }

        let poly_multiplied = items_to_poly(result_pitems);
        
        result_polys.push(String::from(&poly_multiplied))
    }
    
    galios_poly_add(result_polys.iter().map(|poly| poly.as_str()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_poly_multiply() {
        assert_eq!("x6+x5+x4+x", binary_poly_multiply("x3+x", "x3+x2+1"));
        assert_eq!("x", binary_poly_multiply("1", "x"));
    }
}
