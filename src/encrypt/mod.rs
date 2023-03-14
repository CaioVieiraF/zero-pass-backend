pub mod base64;
pub mod vigenere;
pub mod xor;

use crate::{CipherError, CipherResult};

pub use base64::Base64;
pub use vigenere::Vigenere;
pub use xor::Xor;

use std::default::Default;

#[derive(Default, Clone)]
pub struct NoUnique;
#[derive(Default, Clone)]
pub struct Unique(String);
#[derive(Default, Clone)]
pub struct NoVariable;
#[derive(Default, Clone)]
pub struct Variable(String);

#[derive(Clone, Default)]
pub struct PasswordBuilder<U, V> {
    unique: U,
    variable: V,
    repeat: Option<u8>,
}

pub trait Method {
    fn encrypt(&self, uw: impl Into<String>, vw: impl Into<String>) -> CipherResult;
}

impl PasswordBuilder<NoUnique, NoVariable> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<U, V> PasswordBuilder<U, V> {
    pub fn unique(self, word: impl Into<String>) -> PasswordBuilder<Unique, V> {
        PasswordBuilder {
            unique: Unique(word.into()),
            variable: self.variable,
            repeat: self.repeat
        }
    }

    pub fn variable(self, word: impl Into<String>) -> PasswordBuilder<U, Variable> {
        PasswordBuilder {
            unique: self.unique,
            variable: Variable(word.into()),
            repeat: self.repeat,
        }
    }
}

impl PasswordBuilder<Unique, Variable> {
    pub fn repeat(mut self, number: impl Into<u8>) -> Self {
        self.repeat = Some(number.into());

        self
    }

    pub fn method(mut self, method: impl Method) -> Result<Self, CipherError> {
        let vw = self.variable.0.clone();

        let mut repeat = match self.repeat {
            Some(r) => r,
            None => 1,
        };
        if repeat == 0 {
            repeat = 1 as u8;
        }

        for _ in 0..repeat {
            let uw = self.unique.0;
            let new_pass = method.encrypt(&uw, &vw)?;

            self.unique = Unique(new_pass);
        }

        self.repeat = None;

        Ok(self)
    }

    pub fn build(self) -> String {
        self.unique.0
    }

}
