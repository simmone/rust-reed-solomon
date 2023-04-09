use crate::field_math::galios_context::new_gs_from_value;
use crate::field_math::galios_context::GaliosContext;
use crate::field_math::poly_to_items::poly_to_items;
use std::str::FromStr;

fn calculate_factor(ome_poly: &str, factor: u32, gs: &GaliosContext) -> u32 {
    println!("ome_poly: {ome_poly}, factor: {factor}");

    let ome_items = poly_to_items(ome_poly);

    let m2_1 = 2u32.pow(gs.bit_width) - 1;

    let index_list: Vec<u32> = ome_items
        .iter()
        .map(|pitem| {
            let coe_to_a =
                u32::from_str(&gs.galios_number_to_index_hash.get(&pitem.coe).unwrap()[1..])
                    .unwrap();
            println!("coe_to_a: {coe_to_a}");

            let index_multiply_factor = factor * pitem.x_index;
            println!("index_multiply_factor: {index_multiply_factor}");

            let coe_add_last_result = coe_to_a + index_multiply_factor;
            println!("coe_add_last_result: {coe_add_last_result}");

            let modulo_last_result = coe_add_last_result % m2_1;
            println!("modulo_last_result: {modulo_last_result}");

            let index_number = gs
                .galios_index_to_number_hash
                .get(&format!("a{modulo_last_result}"))
                .unwrap();
            println!("index_number: {index_number}");

            *index_number
        })
        .collect();

    println!("index_list: {:?}", index_list);

    let bitwise_xor_result = index_list.into_iter().reduce(|acc, x| acc ^ x).unwrap();
    println!("bitwise_xor_result: {:?}", bitwise_xor_result);

    let result = if bitwise_xor_result == 0 {
        0
    } else {
        u32::from_str(
            &gs.galios_number_to_index_hash
                .get(&bitwise_xor_result)
                .unwrap()[1..],
        )
        .unwrap()
    };
    println!("result: {:?}", result);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forney4() {
        let gs = new_gs_from_value(4, 19);

        assert_eq!(0, calculate_factor("6x+15", 6, &gs));
    }
}
