use crate::field_math::pitem::Pitem;
use crate::field_math::poly_to_items::poly_to_items;

pub fn binary_poly_multiply(poly1: &str, poly2: &str) -> String {
    poly_multiply_basic(poly1, poly2, "+",  "*")
}

fn poly_multiply_basic(poly_multiplicand: &str, poly_multiplier: &str, add_op: &str, multiply_op: &str) -> String {
    let multiplicand_pitems = poly_to_items(poly_multiplicand);
    let multiplier_pitems = poly_to_items(poly_multiplier);
    
    let mut result_pitems: Vec<Pitem> = Vec::new();

    println!("{:?}, {:?}", multiplicand_pitems, multiplier_pitems);

    for multiplicand_pitem in &multiplicand_pitems {
        for multiplier_pitem in &multiplier_pitems {
            result_pitems.push(
                Pitem {
                    x_index: match add_op {
                        "+" => multiplier_pitem.x_index + multiplicand_pitem.x_index,
                        _ => 0,
                    },
                    coe: match multiply_op {
                        "*" => multiplier_pitem.x_index * multiplicand_pitem.x_index,
                        _ => 0,
                    }
                }
            );
        }
    }

    println!("{:?}", result_pitems);

    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_poly_multiply() {
        assert_eq!("x6+x5+x4+x1", binary_poly_multiply("x3+x", "x3+x2+1"));
        assert_eq!("x", binary_poly_multiply("1", "x"));
    }
}
