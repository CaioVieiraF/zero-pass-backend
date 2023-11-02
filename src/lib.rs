pub mod encrypt;
pub mod error;
pub mod prelude;
pub struct Methods;

use crate::encrypt::*;
use prelude::*;

impl Methods {
    pub fn get_methods<'a>() -> Vec<&'a str> {
        let methods = vec!["Vigenere", "Base64", "Xor"];

        methods
    }

    pub fn get_method(text: impl Into<String>) -> Result<Box<dyn Method>> {
        let text = text.into();

        if let Ok(v) = text.parse::<Vigenere>() {
            return Ok(Box::new(v));
        }

        if let Ok(b) = text.parse::<Base64>() {
            return Ok(Box::new(b));
        }

        if let Ok(b) = text.parse::<Xor>() {
            return Ok(Box::new(b));
        }

        Err(Error::InvalidMethodError(text))
    }
}

#[cfg(test)]
mod tests;
