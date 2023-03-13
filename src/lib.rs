use crate::encrypt::*;

pub mod encrypt;

#[derive(Debug, Clone, PartialEq)]
pub enum CipherError {
    InvalidCharacterError,
    InvalidMethodError,
}

pub type CipherResult = Result<String, CipherError>;

pub fn get_methods() -> Vec<String> {
    let methods: Vec<String> = vec![
        serde_json::to_string(&Vigenere).unwrap(),
        serde_json::to_string(&Base64).unwrap(),
        serde_json::to_string(&Xor).unwrap(),
    ];

    methods
}

#[cfg(test)]
mod tests;
