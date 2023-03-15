use crate::field_math::pitem::Pitem;

pub fn rs_encode() -> String {
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rs_encode() {
        assert_eq!("", rs_encode());
    }
}
