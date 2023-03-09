use crate::field_math::galios_context::GaliosContext;

use std::collections::HashMap;

pub fn get_galios_index_to_number_hash(bit_width: u32, gs: &GaliosContext) -> HashMap<&str, u32> {
    let mut index_to_number_hash: HashMap<&str, u32> = HashMap::new();
    
    index_to_number_hash.insert("0", 0);
    
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

        assert_eq!(0, index_to_number_hash.get("0"));
    }
}
