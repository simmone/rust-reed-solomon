use crate::field_math::pitem::Pitem;

pub fn poly_to_items(_poly: &str) -> Vec<Pitem> {
    let polys: Vec<&str> = _poly.split("+").collect();

    let mut polys_iter = polys.iter();

    let mut item_list = Vec::new();

    loop {
        match polys_iter.next() {
            None => break,
            Some(p) => {
                let poly_items: Vec<&str> = p.split("x").collect();

                let poly_iter = poly_items.iter();

                match poly_iter.count() {
                    1 => item_list.push(Pitem {
                        x_index: 0,
                        coe: poly_items[0].parse::<u32>().unwrap(),
                    }),
                    2 => item_list.push(Pitem {
                        x_index: if poly_items[1] == "" {
                            1
                        } else {
                            poly_items[1].parse::<u32>().unwrap()
                        },
                        coe: if poly_items[0] == "" {
                            1
                        } else {
                            poly_items[0].parse::<u32>().unwrap()
                        },
                    }),
                    _ => continue,
                }
            }
        }
    }

    item_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly_to_items() {
        assert_eq!(poly_to_items("1"), vec![Pitem { x_index: 0, coe: 1 }]);
        assert_eq!(poly_to_items("2"), vec![Pitem { x_index: 0, coe: 2 }]);
        assert_eq!(poly_to_items("2x"), vec![Pitem { x_index: 1, coe: 2 }]);
        assert_eq!(poly_to_items("x2"), vec![Pitem { x_index: 2, coe: 1 }]);
    }
}
