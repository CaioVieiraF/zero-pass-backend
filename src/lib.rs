use crate::encrypt::*;

pub mod encrypt;

#[derive(Debug, Clone, PartialEq)]
pub enum CipherError {
    InvalidCharacterError,
    InvalidMethodError,
}

pub enum Methods {
    Vigenere(Vigenere),
    Base64(Base64),
    Xor(Xor),
}

pub type CipherResult = Result<String, CipherError>;

impl Methods {
    pub fn get_methods() -> Vec<String> {
        let methods: Vec<String> = vec![
            serde_json::to_string(&Vigenere).unwrap(),
            serde_json::to_string(&Base64).unwrap(),
            serde_json::to_string(&Xor).unwrap(),
        ];

        methods
    }

    pub fn get_method(self, text: impl Into<String>) -> Result<Box<dyn Method>, CipherError> {
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
