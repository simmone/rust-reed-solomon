use crate::field_math::galios_context::new_gs_from_value;
use crate::field_math::galios_context::GaliosContext;
use crate::field_math::galios_num_multiply::galios_num_multiply;

pub fn get_syndrome(data_list: Vec<u32>, parity_length: u32, gs: &GaliosContext) -> Vec<u32> {
    let mut loop_parity_index = 0;

    let mut result_list: Vec<u32> = vec![];

    'main: loop {
        println!("result_list: {:?}", result_list);

        let ax = format!("a{loop_parity_index}");

        println!("ax: {ax}");

        let ax_val = gs.galios_index_to_number_hash.get(&ax).unwrap();

        println!("ax_val: {ax_val}");

        if loop_parity_index < parity_length {
            loop_parity_index += 1;
            let mut loop_data_iter = data_list.iter();
            let mut last_result: u32 = 0;
            let mut last_xor_result: u32 = 0;
            'step: loop {
                match loop_data_iter.next() {
                    Some(i) => {
                        last_xor_result = i ^ last_result;

                        let ax_multiply =
                            galios_num_multiply(last_xor_result, *ax_val, &gs.field_generator_poly);

                        println!(
                            "{0: >3} ^ {1: <3} = {2: <3} galios_num_multiply {3: <3} = {4: <4}",
                            last_result, i, last_xor_result, ax_val, ax_multiply
                        );
                        last_result = ax_multiply;
                        continue 'step;
                    }
                    None => break 'step,
                }
            }
            result_list.push(last_xor_result);
            continue 'main;
        } else {
            break 'main;
        }
    }

    result_list.reverse();

    let mut result_iter = result_list.iter();

    let mut strip_prefix_zero_list = Vec::<u32>::new();

    let mut prefix_0 = true;
    loop {
        match result_iter.next() {
            Some(item) => {
                if prefix_0 && (*item == 0) {
                    continue;
                } else {
                    prefix_0 = false;

                    strip_prefix_zero_list.push(*item);
                }
            }
            None => break,
        }
    }

    strip_prefix_zero_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_syndrome4() {
        let gs = new_gs_from_value(4, 19);

        assert_eq!(
            vec![12, 4, 3, 15],
            get_syndrome(
                vec![1, 2, 3, 4, 5, 11, 7, 8, 9, 10, 11, 3, 1, 12, 12],
                4,
                &gs
            )
        );

        assert_eq!(
            vec![7, 14, 2],
            get_syndrome(
                vec![12, 12, 1, 3, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
                4,
                &gs
            )
        );
    }

    #[test]
    fn test_get_syndrome8() {
        let gs = new_gs_from_value(8, 285);

        assert_eq!(
            vec![127, 213, 228, 134, 89, 149, 113, 122, 131, 7],
            get_syndrome(
                vec![
                    32, 91, 10, 121, 209, 114, 220, 77, 67, 64, 236, 16, 235, 17, 236, 17, 196, 35,
                    39, 119, 235, 215, 231, 226, 93, 22
                ],
                10,
                &gs
            )
        );

        assert_eq!(
            vec![238, 78, 236, 177, 145, 43, 66, 173, 243, 171, 61, 129, 94, 102, 22, 92],
            get_syndrome(
                vec![
                    248, 146, 101, 20, 154, 230, 111, 233, 94, 213, 1, 93, 180, 149, 155, 81, 253,
                    215, 246, 143, 121, 234, 121, 19, 172, 146, 19, 15, 170, 25, 3, 93, 89, 58, 63,
                    51, 156, 203, 103, 230, 157, 102, 132, 246, 74, 75, 14, 50, 50, 125, 148, 194,
                    1, 144, 15, 98, 36, 222, 214, 1, 242, 232, 68, 48, 254, 100, 102, 143, 142,
                    194, 199, 92, 140, 18, 93, 43, 230, 28, 206, 110, 194, 76, 135, 0, 21, 105,
                    163, 172, 251, 99, 243, 175, 68, 158, 186, 81, 17, 106, 173, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                ],
                16,
                &gs
            )
        );

        assert_eq!(
            Vec::<u32>::new(),
            get_syndrome(
                vec![
                    32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17, 196, 35,
                    39, 119, 235, 215, 231, 226, 93, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0
                ],
                10,
                &gs
            )
        )
    }
}
