use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Debug)]
pub struct GaliosContext {
    pub field_generator_poly: String,
}

impl Display for GaliosContext {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "field_generator_poly: {}", self.field_generator_poly)
    }
}
