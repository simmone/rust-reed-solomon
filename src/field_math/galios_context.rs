use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Debug)]
pub struct GaliosContext {
    pub bit_width: u32,
    pub field_generator_poly: String,
    pub galios_index_to_number_hash: HashMap<String, u32>,
    pub galios_number_to_index_hash: HashMap<u32, String>,
}

impl Display for GaliosContext {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "bit_width: {}, field_generator_poly: {}",
            self.bit_width, self.field_generator_poly
        )
    }
}

pub fn new_gs(bit_width: u32, fgp: &str) -> GaliosContext {
    GaliosContext {
        bit_width,
        field_generator_poly: String::from(fgp),
        galios_index_to_number_hash: HashMap::new(),
        galios_number_to_index_hash: HashMap::new(),
    }
}
