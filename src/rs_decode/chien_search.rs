use crate::field_math::galios_context::new_gs_from_value;
use crate::field_math::galios_context::GaliosContext;
use crate::field_math::poly_to_items::poly_to_items;
use crate::field_math::pitem::Pitem;

fn chien_value(lam_poly: &str, seq: u32, gs: &GaliosContext) -> u32 {
    println!("lam_poly: {lam_poly}, seq: {seq}");

    let m2_1 = 2u32.pow(gs.bit_width) - 1;
    let m2_1_seq = m2_1 - seq;
    println!("m2_1: {m2_1}, m2_1_seq: {m2_1_seq}");
    
    let lam_items = poly_to_items(lam_poly);
    
    let result_list: Vec<u32> = lam_items.iter().map(
        |pitem| {
            println!("1. 2^m_1_seq({m2_1_seq}) * index({}) = {}", pitem.x_index, pitem.coe);
            4
        })
        .collect();
    
    println!("result_list: {:?}", result_list);

    0u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chien_search() {
        let gs = new_gs_from_value(4, 19);

        assert_eq!(3, chien_value("14x2+14x1+1x0", 14, &gs));
    }
}
