use crate::field_math::galios_context::new_gs;
use crate::primitive_poly_table::get_field_generator_poly_by_value;
use crate::field_math::get_code_generator_poly::get_code_generator_poly;
use crate::field_math::poly_to_items::poly_to_items;
use crate::field_math::galios_num_multiply::galios_num_multiply;

pub fn rs_encode(
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
    
    let code_generator_poly = get_code_generator_poly(bit_width, &gs);
    
    println!("code_generator_poly: {code_generator_poly}");
    
    let coefficient_list: Vec<u32> = poly_to_items(&code_generator_poly).iter().map(|item| item.coe).collect();
    
    println!("coefficient_list: {:?}", coefficient_list);

    let mut loop_remainder_list = vec![data_list, vec![0; parity_length.try_into().unwrap()]].concat();

    println!("appended parity_length's 0's to data_list: {:?}", loop_remainder_list);

    println!("poly divide start");
    
    let mut loop_count = 1;

    let mut loop_dividend_list = loop_remainder_list.split_off(parity_length.try_into().unwrap());
    
    loop {
        println!("loop start[{loop_count}]");
        
        println!("loop_dividend_list: {:?}", loop_dividend_list);

        println!("loop_remainder_list: {:?}", loop_remainder_list);
        
        if loop_dividend_list.len() != 0 {
            let appended_dividend_list = vec![loop_remainder_list, loop_dividend_list[0..1].to_vec()].concat();
            println!("step1: remainder list + first item of rest dividend list = {:?} as dividend",
                     appended_dividend_list);

            let first_loop_remainder_item = *appended_dividend_list.get(0).unwrap();
            let aligned_code_generator_list: Vec<u32> =
                coefficient_list.iter().map(|&v| galios_num_multiply(v, first_loop_remainder_item, &gs.field_generator_poly)).collect();
            println!("step2: coefficient_list * loop_remainder_list's first item = {:?} * {} = {:?}",
                     coefficient_list,
                     first_loop_remainder_item,
                     aligned_code_generator_list);
            
            let mut remainder_list: Vec<u32> = Vec::new();
            let mut appended_dividend_list_iter = appended_dividend_list.iter();
            let mut aligned_code_generator_list_iter = aligned_code_generator_list.iter();
            loop {
                match appended_dividend_list_iter.next() {
                    Some(item) => {
                        remainder_list.push(item ^ aligned_code_generator_list_iter.next().unwrap());
                    },
                    None => break,
                }
            }

            println!("step3: appended_dividend_list bitwise-xor aligned_code_generator = {:?}", remainder_list);
            
            loop_count += 1;
            
            loop_dividend_list = loop_dividend_list[1..].to_vec();
            
            loop_remainder_list = remainder_list[1..].to_vec();

            println!("loop end");
        } else {
            break loop_remainder_list
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rs_encode() {
        assert_eq!(
            vec![3, 3, 12, 12],
            rs_encode(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 4, 4, 19)
        );
    }
}
