use crate::field_math::galios_context::new_gs;
use crate::field_math::galios_context::GaliosContext;
use crate::field_math::galios_poly_multiply::galios_poly_multiply;
use crate::field_math::items_to_poly::items_to_poly;
use crate::field_math::poly_remove_dup::poly_remove_dup;
use crate::field_math::poly_sum::poly_sum;
use crate::field_math::poly_to_items::poly_to_items;

use std::collections::HashMap;

pub fn get_galios_index_to_number_hash(gs: &GaliosContext) -> HashMap<String, u32> {
    //    println!("bit_width: {}", gs.bit_width);

    //    println!("field_generator_poly: {}", gs.field_generator_poly);

    let m2_1 = 2_u32.pow(gs.bit_width) - 1;

    //    println!("m2_1: {m2_1}");

    let fgp_pitems = poly_to_items(&gs.field_generator_poly);

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

        let step1 = galios_poly_multiply(vec![&last_val, "x"], &gs);
        //        println!("step1: galios_poly_multiply(vec![{last_val}, \"x\"], &gs): {step1}");

        let step2 = step1.replace(&first_field_generator_poly, &rest_field_generator_poly);
        //        println!("step2: replace poly item by field_generator_poly: {step2}");

        let step3 = poly_remove_dup(&step2);
        //        println!("step3: poly_remove_dup({}): {}", step2, step3);

        index_to_poly_hash.insert(a_index.clone(), step3.clone());

        index_to_number_hash.insert(a_index.clone(), poly_sum(&step3));

        last_val = step3.clone();

        index = index + 1;
    }

    index_to_number_hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galios_index_to_number_hash_4() {
        let mut gs = new_gs(4, "x4+x+1");
        gs.galios_index_to_number_hash = get_galios_index_to_number_hash(&gs);

        assert_eq!(0, *gs.galios_index_to_number_hash.get("0").unwrap());
        assert_eq!(1, *gs.galios_index_to_number_hash.get("a0").unwrap());
        assert_eq!(2, *gs.galios_index_to_number_hash.get("a1").unwrap());
        assert_eq!(4, *gs.galios_index_to_number_hash.get("a2").unwrap());
        assert_eq!(8, *gs.galios_index_to_number_hash.get("a3").unwrap());
        assert_eq!(3, *gs.galios_index_to_number_hash.get("a4").unwrap());
        assert_eq!(6, *gs.galios_index_to_number_hash.get("a5").unwrap());
        assert_eq!(12, *gs.galios_index_to_number_hash.get("a6").unwrap());
        assert_eq!(11, *gs.galios_index_to_number_hash.get("a7").unwrap());
        assert_eq!(5, *gs.galios_index_to_number_hash.get("a8").unwrap());
        assert_eq!(10, *gs.galios_index_to_number_hash.get("a9").unwrap());
        assert_eq!(7, *gs.galios_index_to_number_hash.get("a10").unwrap());
        assert_eq!(14, *gs.galios_index_to_number_hash.get("a11").unwrap());
        assert_eq!(15, *gs.galios_index_to_number_hash.get("a12").unwrap());
        assert_eq!(13, *gs.galios_index_to_number_hash.get("a13").unwrap());
        assert_eq!(9, *gs.galios_index_to_number_hash.get("a14").unwrap());
    }

    #[test]
    fn test_galios_index_to_number_hash_8() {
        let mut gs = new_gs(8, "x8+x4+x3+x2+1");
        gs.galios_index_to_number_hash = get_galios_index_to_number_hash(&gs);

        assert_eq!(0, *gs.galios_index_to_number_hash.get("0").unwrap());
        assert_eq!(1, *gs.galios_index_to_number_hash.get("a0").unwrap());
        assert_eq!(2, *gs.galios_index_to_number_hash.get("a1").unwrap());
        assert_eq!(4, *gs.galios_index_to_number_hash.get("a2").unwrap());
        assert_eq!(8, *gs.galios_index_to_number_hash.get("a3").unwrap());
        assert_eq!(16, *gs.galios_index_to_number_hash.get("a4").unwrap());
        assert_eq!(32, *gs.galios_index_to_number_hash.get("a5").unwrap());
        assert_eq!(64, *gs.galios_index_to_number_hash.get("a6").unwrap());
        assert_eq!(128, *gs.galios_index_to_number_hash.get("a7").unwrap());
        assert_eq!(29, *gs.galios_index_to_number_hash.get("a8").unwrap());
        assert_eq!(58, *gs.galios_index_to_number_hash.get("a9").unwrap());
        assert_eq!(47, *gs.galios_index_to_number_hash.get("a69").unwrap());
        assert_eq!(240, *gs.galios_index_to_number_hash.get("a79").unwrap());
        assert_eq!(221, *gs.galios_index_to_number_hash.get("a204").unwrap());
        assert_eq!(142, *gs.galios_index_to_number_hash.get("a254").unwrap());
    }
}
