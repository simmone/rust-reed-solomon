use crate::field_math::galios_context::new_gs_from_value;
use crate::field_math::galios_context::GaliosContext;

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
                
            }
            continue 'main;
        } else {
            break 'main;
        }
    } 

    vec![0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_syndrome() {
        let gs = new_gs_from_value(4, 19);

        assert_eq!(
            vec![12, 4, 3, 15],
            get_syndrome(
                vec![1, 2, 3, 4, 5, 11, 7, 8, 9, 10, 11, 3, 1, 12, 12],
                4,
                &gs
            )
        )
    }
}
