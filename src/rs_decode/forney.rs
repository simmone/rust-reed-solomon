use crate::field_math::galios_context::GaliosContext;
use crate::field_math::galios_poly_divide::galios_poly_divide;
use crate::field_math::items_to_poly::items_to_poly;
use crate::field_math::poly_to_items::poly_to_items;
use std::str::FromStr;

fn calculate_factor(ome_poly: &str, factor: u32, gs: &GaliosContext) -> u32 {
    //println!("calculate_factor: ome_poly: {ome_poly}, factor: {factor}");

    let ome_items = poly_to_items(ome_poly);

    let m2_1 = 2u32.pow(gs.bit_width) - 1;

    let index_list: Vec<u32> = ome_items
        .iter()
        .map(|pitem| {
            let coe_to_a =
                u32::from_str(&gs.galios_number_to_index_hash.get(&pitem.coe).unwrap()[1..])
                    .unwrap();
            //println!("coe_to_a: {coe_to_a}");

            let index_multiply_factor = factor * pitem.x_index;
            //println!("index_multiply_factor: {index_multiply_factor}");

            let coe_add_last_result = coe_to_a + index_multiply_factor;
            //println!("coe_add_last_result: {coe_add_last_result}");

            let modulo_last_result = coe_add_last_result % m2_1;
            //println!("modulo_last_result: {modulo_last_result}");

            let index_number = gs
                .galios_index_to_number_hash
                .get(&format!("a{modulo_last_result}"))
                .unwrap();
            //println!("index_number: {index_number}");

            *index_number
        })
        .collect();

    //println!("index_list: {:?}", index_list);

    let bitwise_xor_result = index_list.into_iter().reduce(|acc, x| acc ^ x).unwrap();
    //println!("bitwise_xor_result: {:?}", bitwise_xor_result);

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
    //println!("calculate_factor result: {:?}", result);

    result
}

pub fn forney(
    lam_poly: &str,
    ome_poly: &str,
    err_places: &[u32],
    gs: &GaliosContext,
) -> Vec<(u32, u32)> {
    //println!("forney");
    //println!("lam_poly: {lam_poly}");
    //println!("ome_poly: {ome_poly}");
    //println!("err_places: {:?}", err_places);

    let lam_items = poly_to_items(lam_poly);
    let only_odd_poly = items_to_poly(
        lam_items
            .into_iter()
            .filter(|item| item.x_index % 2 != 0)
            .collect(),
    );
    //println!("remove lam_poly's even index part: {only_odd_poly}");

    let (derivative_lam, _) = galios_poly_divide(&only_odd_poly, "x", gs);
    //println!("galios_poly_divide({only_odd_poly}, \"x\")'s quotient = {derivative_lam}");

    err_places
        .iter()
        .map(|error_index| {
            //println!("error_index: {error_index}");

            let m2_1 = 2u32.pow(gs.bit_width) - 1;
            let factor = m2_1 - error_index;
            //println!("m2_1 - {error_index} = {factor}");

            let ome_a = calculate_factor(ome_poly, factor, gs);
            //println!("ome_a = calculate_factor({ome_poly}, {factor}) = {ome_a}");

            let delam_a = calculate_factor(&derivative_lam, factor, gs);
            //println!("delam_a = calculate_factor({derivative_lam}, {factor}) = {delam_a}");

            let cal_a: i32 = *error_index as i32 + ome_a as i32 - delam_a as i32;
            //println!("cal_a = {error_index} + {ome_a} - {delam_a} = {cal_a}");

            let positive_a = (m2_1 as i32 + cal_a) as u32;
            //println!("positive_a = {m2_1} + {cal_a} = {positive_a}");

            let modulo_a = positive_a % m2_1;
            //println!("modulo_a = {positive_a} % {m2_1} = {modulo_a}");

            let result_n = gs
                .galios_index_to_number_hash
                .get(&format!("a{modulo_a}"))
                .unwrap();
            //println!("result_n: {result_n}");

            (*error_index, *result_n)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forney4() {
        let gs = crate::field_math::galios_context::new_gs_from_value(4, 19);

        assert_eq!(0, calculate_factor("6x+15", 6, &gs));

        assert_eq!(4, calculate_factor("10x+2", 6, &gs));

        assert_eq!(14, calculate_factor("10x+2", 13, &gs));

        assert_eq!(11, calculate_factor("14", 6, &gs));

        assert_eq!(
            vec![(9, 13), (2, 2)],
            forney("14x2+14x+1", "6x+15", &vec![9, 2], &gs)
        );
    }

    #[test]
    fn test_forney8() {
        let gs = crate::field_math::galios_context::new_gs_from_value(8, 285);

        assert_eq!(
            vec![(116, 172), (102, 22)],
            forney(
                "62x8+237x7+88x6+121x5+63x4+218x3+249x2+179x+1",
                "152x7+230x6+254x5+208x4+156x3+118x2+152x+134",
                &vec![116, 102],
                &gs
            )
        )
    }
}
