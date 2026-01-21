use crate::field_math::pitem::Pitem;

pub fn items_to_poly(mut items: Vec<Pitem>) -> String {
    items.sort_by(|a, b| b.x_index.cmp(&a.x_index));

    let mut last_operator = "";

    let mut result = String::from("");

    for item in items.iter() {
        result = format!("{result}{last_operator}{}", item);

        last_operator = "+";
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_items_to_poly() {
        assert_eq!("1", items_to_poly(vec![Pitem { x_index: 0, coe: 1 }]));
        assert_eq!("2", items_to_poly(vec![Pitem { x_index: 0, coe: 2 }]));
        assert_eq!("x", items_to_poly(vec![Pitem { x_index: 1, coe: 1 }]));
        assert_eq!("2x", items_to_poly(vec![Pitem { x_index: 1, coe: 2 }]));
        assert_eq!("x2", items_to_poly(vec![Pitem { x_index: 2, coe: 1 }]));
        assert_eq!("2x3", items_to_poly(vec![Pitem { x_index: 3, coe: 2 }]));
        assert_eq!(
            "x4+x3+x2+x+1",
            items_to_poly(vec![
                Pitem { x_index: 4, coe: 1 },
                Pitem { x_index: 3, coe: 1 },
                Pitem { x_index: 2, coe: 1 },
                Pitem { x_index: 1, coe: 1 },
                Pitem { x_index: 0, coe: 1 }
            ])
        );
        assert_eq!(
            "x4+x3+x2+x+1",
            items_to_poly(vec![
                Pitem { x_index: 4, coe: 1 },
                Pitem { x_index: 3, coe: 1 },
                Pitem { x_index: 1, coe: 1 },
                Pitem { x_index: 2, coe: 1 },
                Pitem { x_index: 0, coe: 1 }
            ])
        );
    }
}
