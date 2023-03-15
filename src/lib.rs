use crate::encrypt::*;

pub mod encrypt;

#[derive(Debug, Clone, PartialEq)]
pub enum CipherError {
    InvalidCharacterError,
    InvalidMethodError,
}

pub struct Methods;

pub type CipherResult = Result<String, CipherError>;

impl Methods {
    pub fn get_methods() -> Vec<&'static str> {
        let methods = vec![
            "Vigenere",
            "Base64",
            "Xor",
        ];

        methods
    }

    pub fn get_method(text: impl Into<String>) -> Result<Box<dyn Method>, CipherError> {
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

        Err(CipherError::InvalidMethodError)

    }
}

#[cfg(test)]
mod tests;
