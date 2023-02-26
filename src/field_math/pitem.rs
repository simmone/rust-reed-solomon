use std::fmt::{self, Formatter, Display};

#[derive(PartialEq, Debug)]
pub struct Pitem {
    pub x_index: u32,
    pub coe: u32,
}

impl Display for Pitem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.x_index == 0 {
            write!(f, "{}", self.coe)
        } else {
            write!(f, "{}x{}",
                   if self.coe == 1 {
                       String::from("")
                   } else {
                       self.coe.to_string()
                   },
                   if self.x_index == 1 {
                       String::from("")
                   } else {
                       self.x_index.to_string()
                   })
        }
    }
}
