pub mod base64;
pub mod vigenere;
pub mod xor;

use crate::CipherResult;

pub use base64::B64;
pub use vigenere::Vigenere;
pub use xor::Xor;

use std::default::Default;

pub struct PasswordBuilder {
    unique: Option<String>,
    variable: Option<String>,
    repeat: Option<u8>,
    new_pass: Option<CipherResult>,
}

pub trait Method {
    fn encrypt(&self, uw: impl Into<String>, vw: impl Into<String>) -> CipherResult;
}

impl Default for PasswordBuilder {
    fn default() -> Self {
        PasswordBuilder {
            unique: None,
            variable: None,
            repeat: None,
            new_pass: None,
        }
    }
}

impl PasswordBuilder {
    pub fn new() -> PasswordBuilder {
        Default::default()
    }

    pub fn unique(mut self, word: impl Into<String>) -> Self {
        self.unique = Some(word.into());
        self
    }

    pub fn variable(mut self, word: impl Into<String>) -> Self {
        self.variable = Some(word.into());
        self
    }

    pub fn repeat(mut self, number: impl Into<u8>) -> Self {
        self.repeat = Some(number.into());
        self
    }

    pub fn method(mut self, method: &impl Method) -> Self {
        let mut repeat = match self.repeat {
            Some(r) => r,
            None => 1,
        };
        if repeat == 0 {
            repeat = 1 as u8;
        }

        for _ in 0..repeat {
            let uw = self.unique.clone().unwrap();
            let vw = self.variable.clone().unwrap();
            let new_pass: CipherResult = method.encrypt(&uw, &vw);
            if new_pass.is_ok() {
                self.unique = Some(new_pass.unwrap().clone())
            }
        }

        self
    }

    pub fn build(self) -> CipherResult {
        self.new_pass.unwrap()
    }
}
