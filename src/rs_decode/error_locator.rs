use crate::field_math::galios_context::GaliosContext;
use crate::field_math::galios_poly_add::galios_poly_add;
use crate::field_math::galios_poly_divide::galios_poly_divide;
use crate::field_math::galios_poly_multiply::galios_poly_multiply;
use crate::field_math::items_to_poly::items_to_poly;
use crate::field_math::pitem::Pitem;
use crate::field_math::poly_to_items::poly_to_items;

pub fn error_locator(
    syndromes: Vec<u32>,
    error_length: u32,
    gs: &GaliosContext,
) -> Result<(String, String), bool> {
    // println!("syndromes: {:?}, error_length: {error_length}", syndromes);

    let syndromes_items: Vec<Pitem> = syndromes
        .iter()
        .enumerate()
        .map(|(index, item)| Pitem {
            x_index: (syndromes.len() - index - 1usize) as u32,
            coe: *item,
        })
        .collect();

    let syndrome_poly = items_to_poly(syndromes_items.clone());

    // println!(
    //    "syndromes_items: {:?}, syndrome_poly: {syndrome_poly}",
    //    syndromes_items
    //);

    let mut loop_dividend = format!("x{}", error_length * 2);

    let mut loop_divisor = syndrome_poly;

    let mut loop_add_factor = String::from("0");

    let mut loop_multiply_factor = String::from("1");

    let mut loop_result;

    loop {
        // println!("loop_dividend: {loop_dividend}");
        // println!("loop_divisor: {loop_divisor}");
        // println!("loop_add_factor: {loop_add_factor}");
        // println!("loop_multiply_factor: {loop_multiply_factor}");

        let (quotient, remainder) = galios_poly_divide(&loop_dividend, &loop_divisor, gs);
        // println!("galios_poly_divide({loop_dividend}, {loop_divisor}) = ({quotient} {remainder})");

        let multiply_result = galios_poly_multiply(
            vec![&quotient, &loop_multiply_factor],
            &gs.field_generator_poly,
        );
        // println!("galios_poly_multiply({quotient}, loop_multiply_factor) = {multiply_result}");

        loop_result = galios_poly_add(vec![&loop_add_factor, &multiply_result]);
        // println!(
        // "loop_result = galios_poly_add({loop_add_factor}, {multiply_result}) = {loop_result}"
        // );

        let remainder_items = poly_to_items(&remainder);

        let remainder_first_item = remainder_items.first().unwrap();

        if remainder_first_item.x_index >= error_length {
            loop_dividend = loop_divisor;
            loop_divisor = remainder;
            loop_add_factor = loop_multiply_factor;
            loop_multiply_factor = loop_result;

            continue;
        } else {
            let result_items = poly_to_items(&loop_result);
            let result_items_last_item = result_items.last().unwrap();
            let last_coe = result_items_last_item.coe;
            let last_index = result_items_last_item.x_index;

            if last_index != 0 {
                break Err(false);
            } else {
                // println!("last_coe: {last_coe}, last_index: {last_index}");

                let (ome_quotient, _ome_remainder) =
                    galios_poly_divide(&remainder, &last_coe.to_string(), gs);
                // println!("galios_poly_divide({remainder}, {last_coe}) = (ome_quotient: {ome_quotient}, ome_remainder: {_ome_remainder})");

                let (lam_quotient, _lam_remainder) =
                    galios_poly_divide(&loop_result, &last_coe.to_string(), gs);
                // println!("galios_poly_divide({loop_result}, {last_coe}) = (lam_quotient: {lam_quotient}, lam_remainder: {_lam_remainder})");

                // println!("result: (ome_quotient, lam_quotient) = ({ome_quotient}, {lam_quotient})");

                break Ok((ome_quotient, lam_quotient));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_locator4() {
        let gs = crate::field_math::galios_context::new_gs_from_value(4, 19);

        let result = error_locator(vec![12, 4, 3, 15], 2, &gs);
        let (ome, lam) = result.ok().unwrap();
        assert_eq!("6x+15", ome);
        assert_eq!("14x2+14x+1", lam);

        let result = error_locator(vec![5, 5, 1, 11], 2, &gs);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_locator8() {
        let gs = crate::field_math::galios_context::new_gs_from_value(8, 285);

        let result = error_locator(
            vec![
                17, 0, 135, 209, 16, 239, 171, 210, 53, 216, 13, 247, 89, 197, 161, 90,
            ],
            16,
            &gs,
        );
        let (ome, lam) = result.ok().unwrap();
        assert_eq!(
            "175x14+194x13+164x12+53x11+102x10+105x9+36x8+54x7+69x6+28x5+230x4+251x3+137x2+83x+90",
            ome
        );
        assert_eq!("59x17+39x15+49x14+183x13+235x12+137x11+112x10+114x9+101x8+221x7+11x6+218x5+122x4+x3+166x2+50x+1", lam);
    }
}
