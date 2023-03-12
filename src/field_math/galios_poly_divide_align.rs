pub fn galios_poly_divide_align(dividend_poly: &str, divisor_poly: &str) -> String {
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galios_poly_divide_align_4() {
        let mut gs = new_gs(4, "x4+x+1");
        gs.galios_index_to_number_hash = get_galios_index_to_number_hash(&gs);
        gs.galios_number_to_index_hash = gs.galios_index_to_number_hash.iter().map(|(k, v)| (v, k)).collect(); 


    (parameterize*
     ([*bit_width* 4]
      [*field_generator_poly* "x4+x+1"]
      [*galios_index->number_map* (get-galios-index->number_map)]
      [*galios_number->index_map* (make-hash (hash-map (*galios_index->number_map*) (lambda (a n) (cons n a))))])

     (check-equal? (galios-poly-divide-align "x4" "12x3") "10x")

     (check-equal? (galios-poly-divide-align "14x3" "12x3") "6")

     (check-equal? (galios-poly-divide-align "12x3" "6x2") "2x")
    
     (check-equal? (galios-poly-divide-align "8x2" "6x2") "13")

     (check-equal? (galios-poly-divide-align "14x2" "7x2") "2"))

    (parameterize*
     ([*bit_width* 8]
      [*field_generator_poly* "x8+x4+x3+x2+1"]
      [*galios_index->number_map* (get-galios-index->number_map)]
      [*galios_number->index_map* (make-hash (hash-map (*galios_index->number_map*) (lambda (a n) (cons n a))))])

     (check-equal?
      (galios-poly-divide-align
       "x16"
       "49x14+195x13+228x12+166x11+225x10+133x9+24x8+105x7+4x6+9x5+222x4+119x3+138x2+193x1+87x0")
      "137x2"))
    )

}
