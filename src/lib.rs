pub mod encrypt;
pub mod error;
pub mod prelude;

use crate::encrypt::*;
use prelude::*;
use zero_pass_backend_derive::Method;
use std::str::FromStr;

#[derive(Method, Debug, Clone)]
pub enum Methods {
    Vigenere,
    Base64,
    Xor,
}

impl TryFrom<&str> for Methods {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Methods::try_from(value.to_string())
    }
}

impl FromStr for Methods {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        Methods::try_from(value.to_string())
    }
}

#[cfg(test)]
mod tests;
