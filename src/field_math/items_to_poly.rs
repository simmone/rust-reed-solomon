use crate::field_math::pitem::Pitem;

pub fn items_to_poly(items: Vec<Pitem>) -> String {
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_items_to_poly() {
        assert_eq!("1", items_to_poly(vec![Pitem { x_index: 0, coe: 1 }]));
        assert_eq!("2", items_to_poly(vec![Pitem { x_index: 0, coe: 2 }]));
        assert_eq!("x", items_to_poly(vec![Pitem { x_index: 1, coe: 1 }]));
        assert_eq!(items_to_poly(vec![Pitem { x_index: 1, coe: 2 }), "2x");
        assert_eq!(items_to_poly(vec![Pitem { x_index: 2, coe: 1 }), "x2");
        assert_eq!(
            items_to_poly(vec![
                Pitem { x_index: 4, coe: 1 },
                Pitem { x_index: 3, coe: 1 },
                Pitem { x_index: 2, coe: 1 },
                Pitem { x_index: 1, coe: 1 },
                Pitem { x_index: 0, coe: 1 }
            ],
            poly_to_items("x4+x3+x2+x+1")
        );
        assert_eq!(
            items_to_poly(vec![
                Pitem { x_index: 4, coe: 1 },
                Pitem { x_index: 3, coe: 1 },
                Pitem { x_index: 2, coe: 1 },
                Pitem { x_index: 1, coe: 1 },
                Pitem { x_index: 0, coe: 1 }
            ],
            poly_to_items(" x4 + x3+x2+x+1")
        );
        assert_eq!(
            items_to_poly(vec![
                Pitem { x_index: 4, coe: 1 },
                Pitem { x_index: 3, coe: 1 },
                Pitem { x_index: 2, coe: 1 },
                Pitem { x_index: 1, coe: 1 },
                Pitem { x_index: 0, coe: 1 }
            ],
            poly_to_items("x4+x3+x1+x2+1")
        );
    }
}
