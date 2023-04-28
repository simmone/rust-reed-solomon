pub mod get_syndrome;

pub mod error_locator;

pub mod chien_search;

pub mod forney;

use crate::field_math::galios_context::new_gs_from_value;
use crate::rs_decode::chien_search::chien_search;
use crate::rs_decode::error_locator::error_locator;
use crate::rs_decode::forney::forney;
use crate::rs_decode::get_syndrome::get_syndrome;

pub fn rs_decode_common(
    data_list: Vec<u32>,
    parity_length: u32,
    bit_width: u32,
    primitive_poly_value: u64,
) -> Vec<u32> {
    let gs = new_gs_from_value(bit_width, primitive_poly_value);

    let t_length: u32 = ((parity_length / 2) as f32).floor() as u32;
    //println!("t_length: {t_length}");

    let m2_1 = 2u32.pow(gs.bit_width) - 1;
    //println!("m2_1: {m2_1}");

    let mut appended_data_list = data_list.clone();

    if m2_1 > data_list.len() as u32 {
        let need_appended_count = m2_1 - data_list.len() as u32 + 1;
        //println!("need_appended_count: {need_appended_count}");

        let mut appended_zeros = vec![0; need_appended_count as usize];

        appended_data_list.append(&mut appended_zeros);
    }

    //println!("         data_list: {:?}", data_list);

    //println!("appended_data_list: {:?}", appended_data_list);

    let syndromes = get_syndrome(appended_data_list.clone(), t_length * 2, &gs);
    //println!(
    //        "syndromes: get_syndrome(appended_data_list.clone(), {}, &gs = {:?}",
    //        t_length * 2,
    //        syndromes
    //);

    if syndromes.len() == 0 {
        data_list
    } else {
        let result = error_locator(syndromes, t_length, &gs);
        if result.is_err() {
            data_list
        } else {
            let (ome_poly, lam_poly) = result.ok().unwrap();
            //println!("error_locator: ome_poly: {ome_poly}, lam_poly: {lam_poly}");

            let err_places = chien_search(&lam_poly, &gs);
            //println!("chien_search: err_places: {:?}", err_places);

            if err_places.len() == 0 {
                data_list
            } else {
                let err_correct_pairs = forney(&lam_poly, &ome_poly, &err_places, &gs);
                //println!(
                //    "err_correct_pairs = forney({lam_poly}, {ome_poly}, {:?}, &gs = {:?}",
                //    err_places, err_correct_pairs
                //);

                appended_data_list.reverse();

                for correct_pair in err_correct_pairs {
                    if let Some(err_data) = appended_data_list.get_mut(correct_pair.0 as usize) {
                        print!(
                            "{:?} ^ appended_data_list.get({:?}): {err_data}",
                            correct_pair.1, correct_pair.0
                        );

                        *err_data = correct_pair.1 ^ *err_data;

                        //println!(" = {err_data}");
                    }
                }

                appended_data_list.reverse();

                appended_data_list[0..data_list.len()].to_vec()
            }
        }
    }
}

pub fn rs_decode(data_list: Vec<u32>, parity_length: u32) -> Vec<u32> {
    rs_decode_common(data_list, parity_length, 8, 285)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rs_decode_common() {
        // 2 errors
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 3, 3, 12, 12],
            rs_decode_common(
                vec![1, 2, 3, 4, 5, 11, 7, 8, 9, 10, 11, 3, 1, 12, 12],
                4,
                4,
                19
            )
        );

        // 1 error
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 3, 3, 12, 12],
            rs_decode_common(
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 3, 1, 12, 12],
                4,
                4,
                19
            )
        );

        // no error
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 3, 3, 12, 12],
            rs_decode_common(
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 3, 3, 12, 12],
                4,
                4,
                19
            )
        );
        
        // 3 errors
        assert_eq!(
            vec![5, 2, 3, 4, 5, 11, 7, 8, 9, 10, 11, 3, 1, 12, 12],
            rs_decode_common(
                vec![5, 2, 3, 4, 5, 11, 7, 8, 9, 10, 11, 3, 1, 12, 12],
                4,
                4,
                19
            )
        );

        // 4 errors
        assert_eq!(
            vec![12, 12, 1, 3, 11, 10, 9, 0, 7, 12, 5, 4, 3, 2, 1],
            rs_decode_common(
                vec![12, 12, 1, 3, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
                4,
                4,
                19
            )
        );
    }

    #[test]
    fn test_rs_decode() {
        // 5 errors
        assert_eq!(
            vec![
                32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35, 39,
                119, 235, 215, 231, 226, 93, 23
            ],
            rs_decode(
                vec![
                    32, 91, 10, 121, 209, 114, 220, 77, 67, 64, 236, 16, 235, 17, 236, 17, 196, 35,
                    39, 119, 235, 215, 231, 226, 93, 22
                ],
                10
            )
        );
        
        // no errors
        assert_eq!(
            vec![
                32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35, 39,
                119, 235, 215, 231, 226, 93, 23
            ],
            rs_decode(
                vec![
                    32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35,
                    39, 119, 235, 215, 231, 226, 93, 23
                ],
                10
            )
        );
        
        // 1 error
        assert_eq!(
            vec![32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35, 39, 119, 235, 215, 231, 226, 93, 23],
            rs_decode(
                   vec![32, 91, 10, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35, 39, 119, 235, 215, 231, 226, 93, 23],
                   10)
        );

        // 2 errors
        assert_eq!(
            vec![32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35, 39, 119, 235, 215, 231, 226, 93, 23],
            rs_decode(
                vec![32, 91, 10, 120, 208, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35, 39, 119, 235, 215, 231, 226, 93, 23],
                10)
        );
        
        // 3 errors
        assert_eq!(
            vec![32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35, 39, 119, 235, 215, 231, 226, 93, 23],
            rs_decode(
                vec![32, 91, 10, 120, 209, 115, 220, 77, 67, 64, 236, 17, 236, 17, 235, 17, 196, 35, 39, 119, 235, 215, 231, 226, 93, 23],
                10)
        );

        // 4 errors
        assert_eq!(
            vec![32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35, 39, 119, 235, 215, 231, 226, 93, 23],
            rs_decode(
                vec![32, 91, 10, 121, 209, 114, 220, 77, 67, 64, 236, 16, 235, 17, 236, 17, 196, 35, 39, 119, 235, 215, 231, 226, 93, 23],
                10)
        );


        // can't fix 6 errors
        assert_ne!(
            vec![32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35, 39, 119, 235, 215, 231, 226, 93, 23],
            rs_decode(
                vec![32, 91, 10, 121, 209, 114, 220, 77, 67, 64, 236, 16, 235, 17, 236, 17, 196, 35, 39, 119, 235, 215, 231, 225, 93, 22],
                10)
        );
    }
}
