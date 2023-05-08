use reed_solomon_cx::rs_decode::error_locator::error_locator;
use reed_solomon_cx::field_math::galios_context::new_gs_from_value;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_locator4() {
        let gs = new_gs_from_value(4, 19);

        let result = error_locator(vec![12, 4, 3, 15], 2, &gs);
        let (ome, lam) = result.ok().unwrap();
        assert_eq!("6x+15", ome);
        assert_eq!("14x2+14x+1", lam);

        let result = error_locator(vec![5, 5, 1, 11], 2, &gs);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_locator8() {
        let gs = new_gs_from_value(8, 285);

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
