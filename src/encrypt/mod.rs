// Exposing encryption methods.
pub mod base64;
pub mod vigenere;
pub mod xor;

// Importing the encryption methods.
pub use base64::Base64;
pub use vigenere::Vigenere;
pub use xor::Xor;

// Bringing crate's prelude to scope.
use crate::prelude::*;

// Using standard library's traits.
use std::default::Default;

// Other exports.
use std::rc::Rc;

// Definition of the alphabet used on some encryption methods.
pub static ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

// Setting States for the PasswordBuilder
#[derive(Default, Clone, Debug)]
pub struct NoUnique;
#[derive(Clone, Debug)]
pub struct Unique(Rc<str>);
#[derive(Default, Clone, Debug)]
pub struct NoVariable;
#[derive(Clone, Debug)]
pub struct Variable(Rc<str>);

/// Definines the password builder that implements default values and can be cloned.
#[derive(Clone, Default, Debug)]
pub struct PasswordBuilder<U, V> {
    unique: U,
    variable: V,
    repeat: Option<u8>,
}

/// Defines method encryption trait.
pub trait Method: std::fmt::Debug {
    fn encrypt(&self, uw: Rc<str>, vw: Rc<str>) -> Result<String>;
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
            unique: Unique(Rc::from(word.into())),
            variable: self.variable,
            repeat: self.repeat,
        }
    }

    /// Sets the variable word to build the password.
    pub fn variable(self, word: impl Into<String>) -> PasswordBuilder<U, Variable> {
        PasswordBuilder {
            unique: self.unique,
            variable: Variable(Rc::from(word.into())),
            repeat: self.repeat,
        }
    }
}

/// Implementation of PasswordBuilder when "unique" and "variable" fields are set.
impl PasswordBuilder<Unique, Variable> {
    /// Sets a number of repetitions to use on the following specified method.
    pub fn repeat(self, number: impl Into<u8>) -> Self {
        PasswordBuilder {
            repeat: Some(number.into()),
            ..self
        }
    }

    /// Generates a password based on a method. Can be chained with multiple methods.
    pub fn method<T: Method + Default + PartialEq + Clone>(self, method: T) -> Result<Self> {
        let vw = self.variable.0.clone();

        let mut repeat = self.repeat.unwrap_or(1);
        if repeat == 0 {
            repeat = 1_u8;
        }

        let mut pass = self.unique.0.clone();
        for _ in 0..repeat {
            let new_pass = method.encrypt(pass, vw.clone())?;

            pass = Rc::from(new_pass);
        }

        Ok(PasswordBuilder {
            unique: Unique(pass),
            repeat: None,
            ..self
        })
    }

    /// Generates a password based on a method from a pointer. Can be chained with multiple methods.
    pub fn method_ptr(self, method: Box<dyn Method>) -> Result<Self> {
        let vw = self.variable.0.clone();

        let mut repeat = self.repeat.unwrap_or(1);
        if repeat == 0 {
            repeat = 1_u8;
        }

        let mut pass = self.unique.0.clone();
        for _ in 0..repeat {
            let new_pass = method.encrypt(pass, vw.clone())?;

            pass = Rc::from(new_pass);
        }

        Ok(PasswordBuilder {
            unique: Unique(pass),
            repeat: None,
            ..self
        })
    }

    pub fn build(self) -> String {
        self.unique.0.to_string()
    }
}
