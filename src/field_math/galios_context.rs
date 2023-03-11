use std::fmt::{self, Display, Formatter};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct GaliosContext {
    pub field_generator_poly: String,
    pub galios_index_to_number_hash: HashMap<String, u32>,
}

impl Display for GaliosContext {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "field_generator_poly: {},", self.field_generator_poly)
    }
}

pub fn new_gs(fgp: &str) -> GaliosContext {
    GaliosContext {
        field_generator_poly: String::from(fgp),
        galios_index_to_number_hash: HashMap::new(),
    }
}

