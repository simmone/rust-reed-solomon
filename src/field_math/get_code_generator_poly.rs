use crate::field_math::galios_context::GaliosContext;
use crate::field_math::galios_poly_multiply::galios_poly_multiply;

pub fn get_code_generator_poly(parity_length: u32, gs: &GaliosContext) -> String {
    let mut polys: Vec<String> = Vec::new();

    let max_index = parity_length - 1;

    let mut loop_index = 0;

    while loop_index <= max_index {
        let a_index = format!("a{}", loop_index);

        let poly = format!(
            "x+{}",
            gs.galios_index_to_number_hash.get(&a_index).unwrap()
        );

        polys.push(poly);

        loop_index += 1;
    }

    galios_poly_multiply(
        polys.iter().map(|poly| poly.as_str()).collect(),
        &gs.field_generator_poly,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_code_generator_poly_4() {
        let gs = crate::field_math::galios_context::new_gs_from_poly(4, "x4+x+1");

        assert_eq!("x4+15x3+3x2+x+12", get_code_generator_poly(4, &gs));
    }

    #[test]
    fn test_get_code_generator_poly_8() {
        let gs = crate::field_math::galios_context::new_gs_from_poly(8, "x8+x4+x3+x2+1");

        assert_eq!("x16+59x15+13x14+104x13+189x12+68x11+209x10+30x9+8x8+163x7+65x6+41x5+229x4+98x3+50x2+36x+59",
                   get_code_generator_poly(16, &gs));
    }
}
