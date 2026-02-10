use std::collections::HashMap;

use crate::field_math::items_to_poly::items_to_poly;
use crate::field_math::pitem::Pitem;
use crate::field_math::poly_to_items::poly_to_items;

pub fn poly_remove_dup(poly: &str) -> String {
    let mut unique_pitem_hash: HashMap<Pitem, bool> = HashMap::new();

    for pitem in poly_to_items(poly) {
        if unique_pitem_hash.contains_key(&pitem) {
            unique_pitem_hash.remove(&pitem);
        } else {
            unique_pitem_hash.entry(pitem).or_insert(true);
        }
    }

    items_to_poly(unique_pitem_hash.into_keys().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly_remove_dup() {
        assert_eq!("x4+1", poly_remove_dup("x4+x+1+x"));
        assert_eq!("x4", poly_remove_dup("x4+x+1+x+1"));
        assert_eq!("x4+x2+x", poly_remove_dup("x4+x2+1+x+1"));
    }
}
