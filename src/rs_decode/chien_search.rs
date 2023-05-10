use crate::field_math::galios_context::GaliosContext;
use crate::field_math::poly_to_items::poly_to_items;
use std::str::FromStr;

fn chien_value(lam_poly: &str, seq: u32, gs: &GaliosContext) -> u32 {
    //println!("lam_poly: {lam_poly}, seq: {seq}");

    let m2_1 = 2u32.pow(gs.bit_width) - 1;
    let m2_1_seq = m2_1 - seq;
    //println!("m2_1: {m2_1}, m2_1_seq: {m2_1_seq}");

    let lam_items = poly_to_items(lam_poly);

    let result_list: Vec<u32> = lam_items
        .iter()
        .map(|pitem| {
            let last_multiply_index = m2_1_seq * pitem.x_index;
            //println!("1. 2^m_1_seq({m2_1_seq}) * x_index({}) = {last_multiply_index}", pitem.x_index);

            let convert_coe_to_index =
                u32::from_str(&gs.galios_number_to_index_hash.get(&pitem.coe).unwrap()[1..])
                    .unwrap();
            //println!("2. convert_coe_to_index {} = {}", pitem.coe, convert_coe_to_index);

            let add_and_modulo = (convert_coe_to_index + last_multiply_index) % m2_1;
            //println!("3. add and modulo = ({convert_coe_to_index} + {last_multiply_index}) % {m2_1} = {add_and_modulo}");

            let convert_index_to_coe = gs
                .galios_index_to_number_hash
                .get(&format!("a{add_and_modulo}"))
                .unwrap();
            //println!("convert_index_to_coe = from {add_and_modulo} to {convert_index_to_coe}\n");

            *convert_index_to_coe
        })
        .collect();

    //println!("result_list: {:?}", result_list);

    result_list.into_iter().reduce(|acc, x| acc ^ x).unwrap()
}

pub fn chien_search(lam_poly: &str, gs: &GaliosContext) -> Vec<u32> {
    let mut loop_index = 2i32.pow(gs.bit_width) - 1;
    let mut search_result: Vec<u32> = vec![];

    loop {
        if loop_index >= 0 {
            let chien_value_result = chien_value(lam_poly, loop_index as u32, gs);

            if chien_value_result == 0 {
                search_result.push(loop_index as u32);
            }

            loop_index -= 1;

            continue;
        } else {
            break;
        }
    }

    search_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chien_search4() {
        let gs = crate::field_math::galios_context::new_gs_from_value(4, 19);

        assert_eq!(3, chien_value("14x2+14x1+1", 14, &gs));
        assert_eq!(13, chien_value("14x2+14x1+1", 13, &gs));
        assert_eq!(12, chien_value("14x2+14x1+1", 12, &gs));
        assert_eq!(3, chien_value("14x2+14x1+1", 11, &gs));
        assert_eq!(15, chien_value("14x2+14x1+1", 10, &gs));
        assert_eq!(0, chien_value("14x2+14x1+1", 9, &gs));
        assert_eq!(14, chien_value("14x2+14x1+1", 8, &gs));
        assert_eq!(13, chien_value("14x2+14x1+1", 7, &gs));
        assert_eq!(14, chien_value("14x2+14x1+1", 6, &gs));
        assert_eq!(15, chien_value("14x2+14x1+1", 5, &gs));
        assert_eq!(2, chien_value("14x2+14x1+1", 4, &gs));
        assert_eq!(2, chien_value("14x2+14x1+1", 3, &gs));
        assert_eq!(0, chien_value("14x2+14x1+1", 2, &gs));
        assert_eq!(12, chien_value("14x2+14x1+1", 1, &gs));
        assert_eq!(1, chien_value("14x2+14x1+1", 0, &gs));

        assert_eq!(vec![9, 2], chien_search("14x2+14x+1", &gs));
    }

    #[test]
    fn test_chien_search8() {
        let gs = crate::field_math::galios_context::new_gs_from_value(8, 285);

        assert_eq!(
            Vec::<u32>::new(),
            chien_search("148x8+38x7+153x6+74x5+43x4+7x3+226x2+102x1+1x0", &gs)
        );
    }
}
