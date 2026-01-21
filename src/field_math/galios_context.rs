use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

use crate::field_math::galios_poly_multiply::galios_poly_multiply;
use crate::field_math::items_to_poly::items_to_poly;
use crate::field_math::poly_remove_dup::poly_remove_dup;
use crate::field_math::poly_sum::poly_sum;
use crate::field_math::poly_to_items::poly_to_items;
use crate::field_math::primitive_poly_table::get_field_generator_poly_by_value;

#[derive(PartialEq, Debug)]
pub struct GaliosContext {
    pub bit_width: u32,
    pub field_generator_poly: String,
    pub galios_index_to_number_hash: HashMap<String, u32>,
    pub galios_number_to_index_hash: HashMap<u32, String>,
}

impl Display for GaliosContext {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "bit_width: {}, field_generator_poly: {}",
            self.bit_width, self.field_generator_poly
        )
    }
}

pub fn new_gs_from_value(bit_width: u32, primitive_poly_value: u64) -> GaliosContext {
    let field_generator_poly = get_field_generator_poly_by_value(primitive_poly_value);

    new_gs_from_poly(bit_width, field_generator_poly)
}

pub fn new_gs_from_poly(bit_width: u32, fgp: &str) -> GaliosContext {
    let _galios_index_to_number_hash = get_galios_index_to_number_hash(bit_width, fgp);

    let _galios_number_to_index_hash = _galios_index_to_number_hash
        .iter()
        .map(|(k, v)| (*v, k.clone()))
        .collect();

    GaliosContext {
        bit_width,
        field_generator_poly: String::from(fgp),
        galios_index_to_number_hash: _galios_index_to_number_hash,
        galios_number_to_index_hash: _galios_number_to_index_hash,
    }
}

pub fn get_galios_index_to_number_hash(
    bit_width: u32,
    field_generator_poly: &str,
) -> HashMap<String, u32> {
    //    println!("bit_width: {}", gs.bit_width);

    //    println!("field_generator_poly: {}", gs.field_generator_poly);

    let m2_1 = 2_u32.pow(bit_width) - 1;

    //    println!("m2_1: {m2_1}");

    let fgp_pitems = poly_to_items(field_generator_poly);

    let first_field_generator_poly = items_to_poly(fgp_pitems[0..1].to_vec());

    let rest_field_generator_poly = items_to_poly(fgp_pitems[1..].to_vec());

    //    println!("first_field_generator_poly: {}", first_field_generator_poly);

    //    println!("rest_field_generator_poly: {}", rest_field_generator_poly);

    let mut index_to_number_hash: HashMap<String, u32> = HashMap::new();
    let mut index_to_poly_hash: HashMap<String, String> = HashMap::new();
    let mut poly_index_list: Vec<String> = Vec::new();

    index_to_number_hash.insert(String::from("0"), 0);
    index_to_poly_hash.insert(String::from("0"), String::from("0"));
    poly_index_list.push(String::from("0"));

    index_to_number_hash.insert(String::from("a0"), 1);
    index_to_poly_hash.insert(String::from("a0"), String::from("1"));
    poly_index_list.push(String::from("a0"));

    //    println!("calculating each index's field element\n\n");

    let mut index = 1;

    let mut last_val: String = String::from("1");

    while index < m2_1 {
        let a_index = format!("a{index}");

        poly_index_list.push(a_index.clone());

        //        println!("a_index: {a_index}");

        let step1 = galios_poly_multiply(vec![&last_val, "x"], field_generator_poly);
        //        println!("step1: galios_poly_multiply(vec![{last_val}, \"x\"], {field_generator_poly}): {step1}");

        let step2 = step1.replace(&first_field_generator_poly, &rest_field_generator_poly);
        //        println!("step2: replace poly item by field_generator_poly: {step2}");

        let step3 = poly_remove_dup(&step2);
        //        println!("step3: poly_remove_dup({}): {}", step2, step3);

        index_to_poly_hash.insert(a_index.clone(), step3.clone());

        index_to_number_hash.insert(a_index.clone(), poly_sum(&step3));

        last_val = step3.clone();

        index += 1;
    }

    index_to_number_hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galios_index_to_number_hash_4() {
        let galios_index_to_number_hash = get_galios_index_to_number_hash(4, "x4+x+1");

        assert_eq!(0, *galios_index_to_number_hash.get("0").unwrap());
        assert_eq!(1, *galios_index_to_number_hash.get("a0").unwrap());
        assert_eq!(2, *galios_index_to_number_hash.get("a1").unwrap());
        assert_eq!(4, *galios_index_to_number_hash.get("a2").unwrap());
        assert_eq!(8, *galios_index_to_number_hash.get("a3").unwrap());
        assert_eq!(3, *galios_index_to_number_hash.get("a4").unwrap());
        assert_eq!(6, *galios_index_to_number_hash.get("a5").unwrap());
        assert_eq!(12, *galios_index_to_number_hash.get("a6").unwrap());
        assert_eq!(11, *galios_index_to_number_hash.get("a7").unwrap());
        assert_eq!(5, *galios_index_to_number_hash.get("a8").unwrap());
        assert_eq!(10, *galios_index_to_number_hash.get("a9").unwrap());
        assert_eq!(7, *galios_index_to_number_hash.get("a10").unwrap());
        assert_eq!(14, *galios_index_to_number_hash.get("a11").unwrap());
        assert_eq!(15, *galios_index_to_number_hash.get("a12").unwrap());
        assert_eq!(13, *galios_index_to_number_hash.get("a13").unwrap());
        assert_eq!(9, *galios_index_to_number_hash.get("a14").unwrap());
    }

    #[test]
    fn test_galios_index_to_number_hash_8() {
        let galios_index_to_number_hash = get_galios_index_to_number_hash(8, "x8+x4+x3+x2+1");

        assert_eq!(0, *galios_index_to_number_hash.get("0").unwrap());
        assert_eq!(1, *galios_index_to_number_hash.get("a0").unwrap());
        assert_eq!(2, *galios_index_to_number_hash.get("a1").unwrap());
        assert_eq!(4, *galios_index_to_number_hash.get("a2").unwrap());
        assert_eq!(8, *galios_index_to_number_hash.get("a3").unwrap());
        assert_eq!(16, *galios_index_to_number_hash.get("a4").unwrap());
        assert_eq!(32, *galios_index_to_number_hash.get("a5").unwrap());
        assert_eq!(64, *galios_index_to_number_hash.get("a6").unwrap());
        assert_eq!(128, *galios_index_to_number_hash.get("a7").unwrap());
        assert_eq!(29, *galios_index_to_number_hash.get("a8").unwrap());
        assert_eq!(58, *galios_index_to_number_hash.get("a9").unwrap());
        assert_eq!(47, *galios_index_to_number_hash.get("a69").unwrap());
        assert_eq!(240, *galios_index_to_number_hash.get("a79").unwrap());
        assert_eq!(221, *galios_index_to_number_hash.get("a204").unwrap());
        assert_eq!(142, *galios_index_to_number_hash.get("a254").unwrap());
    }
}
