use crate::field_math::galios_context::new_gs_from_value;
use crate::field_math::galios_context::GaliosContext;
use crate::field_math::galios_num_multiply::galios_num_multiply;
use crate::field_math::items_to_poly::items_to_poly;
use crate::field_math::pitem::Pitem;
use crate::field_math::galios_poly_divide::galios_poly_divide;
use crate::field_math::galios_poly_multiply::galios_poly_multiply;
use crate::field_math::galios_poly_add::galios_poly_add;

pub fn error_locator(
    syndromes: Vec<u32>,
    error_length: u32,
    gs: &GaliosContext,
) -> Result<(String, String), bool> {
    println!("syndromes: {:?}, error_length: {error_length}", syndromes);

    let syndromes_items: Vec<Pitem> = syndromes
        .iter()
        .enumerate()
        .map(|(index, item)| Pitem {
            x_index: (syndromes.len() - index - 1usize) as u32,
            coe: *item,
        })
        .collect();

    let syndrome_poly = items_to_poly(syndromes_items.clone());

    println!(
        "syndromes_items: {:?}, syndrome_poly: {syndrome_poly}",
        syndromes_items
    );
    
    let mut loop_dividend = format!("x{}", error_length * 2);
    
    let mut loop_divisor = syndrome_poly;
    
    let mut loop_add_factor = String::from("0");
    
    let mut loop_multiply_factor = String::from("1");
    
    let mut loop_result = String::from("");
    
    loop {
        println!("loop_dividend: {loop_dividend}");
        println!("loop_divisor: {loop_divisor}");
        println!("loop_add_factor: {loop_add_factor}");
        println!("loop_multiply_factor: {loop_multiply_factor}");
        
        let (quotient, remainder) = galios_poly_divide(&loop_dividend, &loop_divisor, gs);
        println!("galios_poly_divide({loop_dividend}, {loop_divisor}) = ({quotient} {remainder})");
        
        let multiply_result = galios_poly_multiply(vec![&quotient, &loop_multiply_factor], &gs.field_generator_poly);
        println!("galios_poly_multiply({quotient}, loop_multiply_factor) = {multiply_result}");

        loop_result = galios_poly_add(vec![&loop_add_factor, &multiply_result]);
        println!("loop_result = galios_poly_add({loop_add_factor}, {multiply_result}) = {loop_result}");

        break;
    }

    Ok(("".to_string(), "".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_locator() {
        let gs = new_gs_from_value(4, 19);

        let result = error_locator(vec![12, 4, 3, 15], 2, &gs);
        let (ome, lam) = result.ok().unwrap();
        assert_eq!("6x+15", ome);
        assert_eq!("14x2+14x+1", lam);
    }
}
