use crate::field_math::galios_context::new_gs_from_value;
use crate::field_math::galios_context::GaliosContext;
use crate::field_math::galios_num_multiply::galios_num_multiply;
use crate::field_math::items_to_poly::items_to_poly;
use crate::field_math::pitem::Pitem;

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

    let syndromes_poly = items_to_poly(syndromes_items.clone());

    println!(
        "syndromes_items: {:?}, syndromes_poly: {syndromes_poly}",
        syndromes_items
    );

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
