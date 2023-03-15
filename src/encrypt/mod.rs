// Exposing encryption methods.
pub mod base64;
pub mod vigenere;
pub mod xor;
pub use self::base64::Base64;
pub use vigenere::Vigenere;
pub use xor::Xor;

// Bringing errors to scope.
use crate::{CipherError, CipherResult};

// Using standard library's traits.
use std::default::Default;
use std::str::FromStr;

// Setting States for the PasswordBuilder
#[derive(Default, Clone)]
pub struct NoUnique;
#[derive(Default, Clone)]
pub struct Unique(String);
#[derive(Default, Clone)]
pub struct NoVariable;
#[derive(Default, Clone)]
pub struct Variable(&'static str);

/// Definines the password builder that implements default values and can be cloned.
#[derive(Clone, Default)]
pub struct PasswordBuilder<U, V> {
    unique: U,
    variable: V,
    repeat: Option<u8>,
}

/// Defines method encryption trait.
pub trait Method {
    fn encrypt(&self, uw: String, vw: &'static str) -> CipherResult;
}

/// Implementation of PasswordBuilder when nothing is set yet.
impl PasswordBuilder<NoUnique, NoVariable> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Implementation of PasswordBuilder when it is already instantiated.
impl<U, V> PasswordBuilder<U, V> {
    /// Sets the unique word to build the passoword.
    pub fn unique(self, word: impl Into<String>) -> PasswordBuilder<Unique, V> {
        PasswordBuilder {
            unique: Unique(word.into()),
            variable: self.variable,
            repeat: self.repeat,
        }
    }

    /// Sets the variable word to build the password.
    pub fn variable(self, word: impl Into<&'static str>) -> PasswordBuilder<U, Variable> {
        PasswordBuilder {
            unique: self.unique,
            variable: Variable(word.into()),
            repeat: self.repeat,
        }
    }
}

/// Implementation of PasswordBuilder when "unique" and "variable" fields are set.
impl PasswordBuilder<Unique, Variable> {
    /// Sets a number of repetitions to use on the following specified method.
    pub fn repeat(mut self, number: impl Into<u8>) -> Self {
        self.repeat = Some(number.into());

        self
    }

    /// Generates a password based on a method. Can be chained with multiple methods.
    pub fn method(mut self, method: impl Method + FromStr) -> Result<Self, CipherError> {
        let vw = self.variable.0;

        let mut repeat = self.repeat.unwrap_or(1);
        if repeat == 0 {
            repeat = 1_u8;
        }

        for _ in 0..repeat {
            let uw = self.unique.0;
            let new_pass = method.encrypt(uw, vw)?;

            self.unique = Unique(new_pass);
        }

        self.repeat = None;

        self.variable.0 = vw;
        Ok(self)
    }

    pub fn build(self) -> String {
        self.unique.0
    }
}
