use crate::field_math::pitem::Pitem;

pub fn poly_to_items(_poly: &str) -> Vec<Pitem> {
    let polys: Vec<&str> = _poly.split("+").collect();

    let mut polys_iter = polys.iter();

    let mut items = Vec::new();

    loop {
        match polys_iter.next() {
            None => break,
            Some(p) => {
                let poly_items: Vec<&str> = p.trim().split("x").collect();

                match poly_items.len() {
                    1 => items.push(Pitem {
                        x_index: 0,
                        coe: poly_items[0].parse::<u32>().unwrap(),
                    }),
                    2 => items.push(Pitem {
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
    
    items.sort_by(|a, b| b.x_index.cmp(&a.x_index));

    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly_to_items() {
        assert_eq!(vec![Pitem { x_index: 0, coe: 1 }], poly_to_items("1"));
        assert_eq!(vec![Pitem { x_index: 0, coe: 2 }], poly_to_items("2"));
        assert_eq!(vec![Pitem { x_index: 1, coe: 1 }], poly_to_items("x"));
        assert_eq!(vec![Pitem { x_index: 1, coe: 2 }], poly_to_items("2x"));
        assert_eq!(vec![Pitem { x_index: 2, coe: 1 }], poly_to_items("x2"));
        assert_eq!(vec![Pitem { x_index: 3, coe: 2 }], poly_to_items("2x3"));
        assert_eq!(
            vec![
                Pitem { x_index: 4, coe: 1 },
                Pitem { x_index: 3, coe: 1 },
                Pitem { x_index: 2, coe: 1 },
                Pitem { x_index: 1, coe: 1 },
                Pitem { x_index: 0, coe: 1 }
            ],
            poly_to_items("x4+x3+x2+x+1")
        );
        assert_eq!(
            vec![
                Pitem { x_index: 4, coe: 1 },
                Pitem { x_index: 3, coe: 1 },
                Pitem { x_index: 2, coe: 1 },
                Pitem { x_index: 1, coe: 1 },
                Pitem { x_index: 0, coe: 1 }
            ],
            poly_to_items(" x4 + x3+x2+x+1")
        );
        assert_eq!(
            vec![
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
