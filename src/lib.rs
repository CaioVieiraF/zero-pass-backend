use crate::encrypt::{MethodArgs, Methods};
use std::collections::HashMap;

pub mod encrypt;
pub mod login_data;

#[derive(Debug, Clone, PartialEq)]
pub enum CipherError {
    InvalidCharacterError,
    InvalidMethodError,
}

pub type CipherResult = Result<String, CipherError>;

pub fn get_methods<'a>() -> HashMap<String, fn(MethodArgs<'a>) -> Methods<'a>> {
    let mut methods: HashMap<String, fn(MethodArgs<'a>) -> Methods<'a>> = HashMap::new();

    methods.insert(String::from("Vigenere"), Methods::Vigenere);
    methods.insert(String::from("Base64"), Methods::B64);
    methods.insert(String::from("Xor"), Methods::Xor);

    methods
}

#[cfg(test)]
mod tests;
