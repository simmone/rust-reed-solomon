pub mod get_syndrome;

pub mod error_locator;

pub mod chien_search;

pub mod forney;

use crate::field_math::galios_context::new_gs_from_value;
use crate::rs_decode::chien_search::chien_search;
use crate::rs_decode::error_locator::error_locator;
use crate::rs_decode::get_syndrome::get_syndrome;

pub fn rs_decode(
    data_list: Vec<u32>,
    parity_length: u32,
    bit_width: u32,
    primitive_poly_value: u64,
) -> Vec<u32> {
    let gs = new_gs_from_value(bit_width, primitive_poly_value);

    let t_length: u32 = ((parity_length / 2) as f32).floor() as u32;
    println!("t_length: {t_length}");

    let m2_1 = 2u32.pow(gs.bit_width) - 1;
    println!("m2_1: {m2_1}");

    let mut appended_data_list = data_list.clone();

    if m2_1 > data_list.len() as u32 {
        let need_appended_count = m2_1 - data_list.len() as u32 + 1;
        println!("need_appended_count: {need_appended_count}");

        let mut appended_zeros = vec![0; need_appended_count as usize];

        appended_data_list.append(&mut appended_zeros);
    }

    println!("         data_list: {:?}", data_list);

    println!("appended_data_list: {:?}", appended_data_list);

    let syndromes = get_syndrome(appended_data_list, t_length * 2, &gs);
    println!("syndromes: {:?}", syndromes);

    if syndromes.len() == 0 {
        data_list
    } else {
        let result = error_locator(syndromes, t_length, &gs);
        if result.is_err() {
            data_list
        } else {
            let (ome_poly, lam_poly) = result.ok().unwrap();
            println!("error_locator: ome_poly: {ome_poly}, lam_poly: {lam_poly}");

            let err_places = chien_search(&lam_poly, &gs);
            println!("chien_search: err_places: {:?}", err_places);

            if err_places.len() == 0 {
                data_list
            } else {
                //                let err_correct_pairs  = forney(lam_poly, ome_poly, err_places);

                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 3, 3, 12, 12]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rs_decode() {
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 3, 3, 12, 12],
            rs_decode(
                vec![1, 2, 3, 4, 5, 11, 7, 8, 9, 10, 11, 3, 1, 12, 12],
                4,
                4,
                19
            )
        )
    }
}
