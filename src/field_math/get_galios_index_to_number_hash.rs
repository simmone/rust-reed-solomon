use crate::field_math::galios_context::GaliosContext;
use crate::field_math::poly_to_items::poly_to_items;
use crate::field_math::items_to_poly::items_to_poly;
use crate::field_math::galios_poly_multiply::galios_poly_multiply;

use std::collections::HashMap;

pub fn get_galios_index_to_number_hash(bit_width: u32, gs: &GaliosContext) -> HashMap<&str, i32> {
    println!("bit_width: {bit_width}");

    println!("field_generator_poly: {}", gs.field_generator_poly);
    
    let m2_1 = 2_u32.pow(bit_width) - 1;
    
    println!("m2_1: {m2_1}");
    
    let fgp_pitems = poly_to_items(&gs.field_generator_poly);
    
    let first_field_generator_poly = items_to_poly(fgp_pitems[0..1].to_vec());
    
    let rest_field_generator_poly = items_to_poly(fgp_pitems[1..].to_vec());
    
    println!("first_field_generator_poly: {}", first_field_generator_poly);
    
    println!("rest_field_generator_poly: {}", rest_field_generator_poly);

    let mut index_to_number_hash: HashMap<&str, i32> = HashMap::new();
    let mut index_to_poly_hash: HashMap<&str, &str> = HashMap::new();
    let mut poly_index_list: Vec<String> = Vec::new();
    
    index_to_number_hash.insert("0", 0);
    index_to_poly_hash.insert("0", "0");
    poly_index_list.push(String::from("0"));
    poly_index_list.push(String::from("a0"));

    println!("calculating each index's field element\n\n");

    let mut index = 1;
    
    let mut last_val = "1";

    while index < m2_1 {
        let mut a_index = format!("a{index}");
        
        poly_index_list.push(a_index.clone());
        
        println!("a_index: {a_index}");

        let step1 = galios_poly_multiply(vec![last_val, "x"], &gs);
        println!("step1: galios_poly_multiply(vec![{last_val}, \"x\"], &gs): {step1}");

        let step2 = step1.replace(&first_field_generator_poly, &rest_field_generator_poly);
        println!("step2: replace poly item by field_generator_poly:{step2}");
        
        index = index + 1;
    }
    
    index_to_number_hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galios_index_to_number_hash_4() {
        let gs = GaliosContext {
            field_generator_poly: String::from("x4+x+1"),
        };
        
        let index_to_number_hash = get_galios_index_to_number_hash(4, &gs);

        assert_eq!(0, *index_to_number_hash.get("0").unwrap());
        assert_eq!(1, *index_to_number_hash.get("a0").unwrap());
        assert_eq!(2, *index_to_number_hash.get("a1").unwrap());
        assert_eq!(4, *index_to_number_hash.get("a2").unwrap());
        assert_eq!(8, *index_to_number_hash.get("a3").unwrap());
        assert_eq!(3, *index_to_number_hash.get("a4").unwrap());
        assert_eq!(6, *index_to_number_hash.get("a5").unwrap());
        assert_eq!(12, *index_to_number_hash.get("a6").unwrap());
        assert_eq!(11, *index_to_number_hash.get("a7").unwrap());
        assert_eq!(5, *index_to_number_hash.get("a8").unwrap());
        assert_eq!(10, *index_to_number_hash.get("a9").unwrap());
        assert_eq!(7, *index_to_number_hash.get("a10").unwrap());
        assert_eq!(14, *index_to_number_hash.get("a11").unwrap());
        assert_eq!(15, *index_to_number_hash.get("a12").unwrap());
        assert_eq!(13, *index_to_number_hash.get("a13").unwrap());
        assert_eq!(9, *index_to_number_hash.get("a14").unwrap());
    }
}
