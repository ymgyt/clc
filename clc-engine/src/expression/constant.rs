use crate::errors::InvalidConstantError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Constant {
    Pi,
    E,
}

impl<'a> TryFrom<&'a str> for Constant {
    type Error = InvalidConstantError<'a>;

    fn try_from(c: &'a str) -> Result<Self, Self::Error> {
        use Constant::*;
        let c = match c {
            "pi" => Pi,
            "e" => E,
            raw => return Err(InvalidConstantError::new(raw)),
        };
        Ok(c)
    }
}
