//! # decode `Vec<u32>` to fix possible errors

mod chien_search;
mod error_locator;
mod forney;
mod get_syndrome;

use crate::field_math::galios_context::new_gs_from_value;
use crate::rs_decode::chien_search::chien_search;
use crate::rs_decode::error_locator::error_locator;
use crate::rs_decode::forney::forney;
use crate::rs_decode::get_syndrome::get_syndrome;

/// encode `Vec<u32>` data to generate parity data, use bit_width: 8, primitive_poly_value: 285
/// # Examples
/// ```
/// // original data: vec![32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17,
/// //                     196, 35, 39, 119, 235, 215, 231, 226, 93, 23]
/// // make 5 errors: vec![33, 91, 11, 120, 219, 144, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17,
/// //                     196, 35, 39, 119, 251, 215, 231, 226, 93, 92]
/// assert_eq!(
///   vec![32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17,
///        196, 35, 39, 119, 235, 215, 231, 226, 93, 23],
///   reed_solomon_cx::rs_decode::rs_decode(
///     vec![33, 91, 11, 120, 219, 144, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17,
///          196, 35, 39, 119, 251, 215, 231, 226, 93, 92],
///     10
///   )
/// );
/// ```
pub fn rs_decode(data_list: Vec<u32>, parity_length: u32) -> Vec<u32> {
    rs_decode_common(data_list, parity_length, 8, 285)
}

/// decode `Vec<u32>` to fix possible errors, use specific bit_width and primitive_poly_value
/// # Examples
///
/// ```
/// assert_eq!(
///   vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 3, 3, 12, 12],
///   reed_solomon_cx::rs_decode::rs_decode_common(
///     vec![1, 2, 3, 4, 5, 11, 7, 8, 9, 10, 11, 3, 1, 12, 12],
///     4,
///     4,
///     19
///   )
/// )
/// ```
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
                        //print!(
                        //    "{:?} ^ appended_data_list.get({:?}): {err_data}",
                        //    correct_pair.1, correct_pair.0
                        //);

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

        // odd parity length
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 15, 11, 11, 0, 15],
            rs_decode_common(
                vec![1, 2, 3, 4, 5, 11, 7, 8, 9, 0, 11, 15, 11, 11, 0, 15],
                5,
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
            vec![
                32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35, 39,
                119, 235, 215, 231, 226, 93, 23
            ],
            rs_decode(
                vec![
                    32, 91, 10, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35,
                    39, 119, 235, 215, 231, 226, 93, 23
                ],
                10
            )
        );

        // 2 errors
        assert_eq!(
            vec![
                32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35, 39,
                119, 235, 215, 231, 226, 93, 23
            ],
            rs_decode(
                vec![
                    32, 91, 10, 120, 208, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35,
                    39, 119, 235, 215, 231, 226, 93, 23
                ],
                10
            )
        );

        // 3 errors
        assert_eq!(
            vec![
                32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35, 39,
                119, 235, 215, 231, 226, 93, 23
            ],
            rs_decode(
                vec![
                    32, 91, 10, 120, 209, 115, 220, 77, 67, 64, 236, 17, 236, 17, 235, 17, 196, 35,
                    39, 119, 235, 215, 231, 226, 93, 23
                ],
                10
            )
        );

        // 4 errors
        assert_eq!(
            vec![
                32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35, 39,
                119, 235, 215, 231, 226, 93, 23
            ],
            rs_decode(
                vec![
                    32, 91, 10, 121, 209, 114, 220, 77, 67, 64, 236, 16, 235, 17, 236, 17, 196, 35,
                    39, 119, 235, 215, 231, 226, 93, 23
                ],
                10
            )
        );

        // can't fix 6 errors
        assert_ne!(
            vec![
                32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35, 39,
                119, 235, 215, 231, 226, 93, 23
            ],
            rs_decode(
                vec![
                    32, 91, 10, 121, 209, 114, 220, 77, 67, 64, 236, 16, 235, 17, 236, 17, 196, 35,
                    39, 119, 235, 215, 231, 225, 93, 22
                ],
                10
            )
        );
    }

    #[test]
    fn test_rs_decode_string() {
        let mut err_str_bytes: Vec<u32> = "Chen Xiao is just a progr54321."
            .bytes()
            .map(|item| item as u32)
            .collect();
        let mut parity_bytes: Vec<u32> = vec![250, 189, 109, 169, 189, 181, 76, 72, 94, 173];
        err_str_bytes.append(&mut parity_bytes);
        assert_eq!(
            b"Chen Xiao is just a programmer."
                .iter()
                .map(|item| *item as u32)
                .collect::<Vec<u32>>(),
            rs_decode(err_str_bytes, 10)[..31]
        );
    }

    #[test]
    fn test_rs_decode_string_6_errors_canot_recover() {
        let mut err_str_bytes: Vec<u32> = "Chen Xiao is just a prog654321."
            .bytes()
            .map(|item| item as u32)
            .collect();
        let mut parity_bytes: Vec<u32> = vec![250, 189, 109, 169, 189, 181, 76, 72, 94, 173];
        err_str_bytes.append(&mut parity_bytes);
        assert_ne!(
            b"Chen Xiao is just a programmer."
                .iter()
                .map(|item| *item as u32)
                .collect::<Vec<u32>>(),
            rs_decode(err_str_bytes, 10)[..31]
        );
    }

    #[test]
    fn test_rs_decode_recover_17_errors() {
        let mut err_str_bytes: Vec<u32> = "Chen Xiao is a fabulous artist."
            .bytes()
            .map(|item| item as u32)
            .collect();
        let mut parity_bytes: Vec<u32> = vec![
            201, 232, 253, 243, 90, 249, 138, 230, 111, 33, 73, 65, 232, 242, 136, 181, 174, 184,
            191, 159, 231, 30, 32, 155, 76, 22, 129, 29, 204, 46, 200, 46, 101, 46,
        ];
        err_str_bytes.append(&mut parity_bytes);
        assert_eq!(
            b"Chen Xiao is just a programmer."
                .iter()
                .map(|item| *item as u32)
                .collect::<Vec<u32>>(),
            rs_decode(err_str_bytes, 34)[..31]
        );
    }
}
