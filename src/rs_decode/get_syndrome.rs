use crate::field_math::galios_context::GaliosContext;
use crate::field_math::galios_num_multiply::galios_num_multiply;

pub fn get_syndrome(data_list: Vec<u32>, parity_length: u32, gs: &GaliosContext) -> Vec<u32> {
    let mut loop_parity_index = 0;

    let mut result_list: Vec<u32> = vec![];

    'main: loop {
        //println!("result_list: {:?}", result_list);

        let ax = format!("a{loop_parity_index}");

        //println!("ax: {ax}");

        let ax_val = gs.galios_index_to_number_hash.get(&ax).unwrap();

        //println!("ax_val: {ax_val}");

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

                        //println!(
                        //    "{0: >3} ^ {1: <3} = {2: <3} galios_num_multiply {3: <3} = {4: <4}",
                        //    last_result, i, last_xor_result, ax_val, ax_multiply
                        //);
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
