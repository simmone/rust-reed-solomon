use crate::field_math::galios_context::new_gs;
use crate::field_math::galios_num_multiply::galios_num_multiply;
use crate::field_math::get_code_generator_poly::get_code_generator_poly;
use crate::field_math::poly_to_items::poly_to_items;
use crate::primitive_poly_table::get_field_generator_poly_by_value;

pub fn rs_encode(data_list: Vec<u8>, parity_length: u32) -> Vec<u32> {
    rs_encode_common(data_list.iter().map(|i| u32::from(*i)).collect(), 8, parity_length, 285)
}

pub fn rs_encode_str(data_str: &str, parity_length: u32) -> &str {
    let encoded_vec = rs_encode(data_str.bytes().collect(), parity_length);

    std::str::from_utf8(encoded_vec.iter().map(|i| *i as u8).collect()).unwrap()
}

pub fn rs_encode_common(
    data_list: Vec<u32>,
    bit_width: u32,
    parity_length: u32,
    primitive_poly_value: u64,
) -> Vec<u32> {
        println!("data_list: {:?}", data_list);

        println!("parity_length: {parity_length}");

        println!("bit_width: {bit_width}");

    let field_generator_poly = get_field_generator_poly_by_value(primitive_poly_value);

        println!("field_generator_poly: {field_generator_poly}");

    let gs = new_gs(bit_width, field_generator_poly);

    let code_generator_poly = get_code_generator_poly(parity_length, &gs);

        println!("code_generator_poly: {code_generator_poly}");

    let coefficient_list: Vec<u32> = poly_to_items(&code_generator_poly)
        .iter()
        .map(|item| item.coe)
        .collect();

        println!("coefficient_list: {:?}", coefficient_list);

    let mut loop_remainder_list =
        vec![data_list, vec![0; parity_length.try_into().unwrap()]].concat();

        println!(
            "appended parity_length's 0's to data_list: {:?}",
            loop_remainder_list
        );

        println!("poly divide start");

    let mut _loop_count = 0;

    let mut loop_dividend_list = loop_remainder_list.split_off(parity_length.try_into().unwrap());

    loop {
                println!("loop start[{_loop_count}]");

                println!("loop_dividend_list: {:?}", loop_dividend_list);

                println!("loop_remainder_list: {:?}", loop_remainder_list);

        if loop_dividend_list.len() != 0 {
            let appended_dividend_list =
                vec![loop_remainder_list, loop_dividend_list[0..1].to_vec()].concat();
                        println!(
                            "step1: remainder list + first item of rest dividend list = {:?} as dividend",
                            appended_dividend_list
                        );

            let first_loop_remainder_item = *appended_dividend_list.get(0).unwrap();
            let aligned_code_generator_list: Vec<u32> = coefficient_list
                .iter()
                .map(|&v| {
                    galios_num_multiply(v, first_loop_remainder_item, &gs.field_generator_poly)
                })
                .collect();
                        println!(
                            "step2: coefficient_list * loop_remainder_list's first item = {:?} * {} = {:?}",
                            coefficient_list, first_loop_remainder_item, aligned_code_generator_list
                        );

            let mut remainder_list: Vec<u32> = Vec::new();
            let mut appended_dividend_list_iter = appended_dividend_list.iter();
            let mut aligned_code_generator_list_iter = aligned_code_generator_list.iter();
            loop {
                match appended_dividend_list_iter.next() {
                    Some(item) => {
                        remainder_list
                            .push(item ^ aligned_code_generator_list_iter.next().unwrap());
                    }
                    None => break,
                }
            }

                        println!(
                            "step3: appended_dividend_list bitwise-xor aligned_code_generator = {:?}",
                            remainder_list
                        );

            _loop_count += 1;

            loop_dividend_list = loop_dividend_list[1..].to_vec();

            loop_remainder_list = remainder_list[1..].to_vec();

                        println!("loop end");
        } else {
            break loop_remainder_list;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rs_encode_common() {
        assert_eq!(
            vec![3, 3, 12, 12],
            rs_encode_common(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 4, 4, 19)
        );

        assert_eq!(
            vec![15, 11, 11, 0, 15],
            rs_encode_common(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 4, 5, 19)
        );
    }
    
    #[test]
    fn test_rs_encode() {
        assert_eq!(
            vec![196, 35, 39, 119, 235, 215, 231, 226, 93, 23],
            rs_encode(vec![32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17], 10)
        );
        
        assert_eq!(
            vec![168, 72, 22, 82, 217, 54, 156, 0, 46, 15, 180, 122, 16],
            rs_encode(vec![32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236], 13)
        );

        assert_eq!(
            vec![0, 154, 220, 253, 68, 10, 124, 102, 201, 53, 167, 140, 96, 91, 50, 66],
            rs_encode(vec![35, 37, 245, 131, 35, 83, 116, 84, 83], 16)
        );
    }
    
    #[test]
    fn test_rs_encode_str() {
        assert_eq!(
            "ss",
            rs_encode_str("Chen Xiao is just a programmer.", 10)
        );
    }
}
