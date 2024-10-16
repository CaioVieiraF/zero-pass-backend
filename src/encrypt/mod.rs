// Using standard library's traits.
use std::default::Default;

use base64ct::{Base64, Encoding};
// Other exports.
use digest::DynDigest;

// Importing the encryption methods.
pub use vigenere::Vigenere;
pub use xor::Xor;

// Bringing crate's prelude to scope.
use crate::prelude::*;

// Exposing encryption methods.
pub mod vigenere;
pub mod xor;

// Definition of the alphabet used on some encryption methods.
pub static ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

// Setting States for the PasswordBuilder
#[derive(Default, Clone, Debug)]
pub struct NoContext;
#[derive(Clone, Debug)]
pub struct Context(Box<str>);
#[derive(Default, Clone, Debug)]
pub struct NoService;
#[derive(Clone, Debug)]
pub struct Service(Box<str>);
#[derive(Default, Clone, Debug)]
pub struct NoEncrypt;
#[derive(Default, Clone, Debug)]
pub struct Encrypt(Box<[u8]>);
#[derive(Default, Clone, Debug)]
pub struct Hashed(String);
#[derive(Default, Clone, Debug)]
pub struct NoHashed;

/// Definines the password builder that implements default values and can be cloned.
#[derive(Clone, Default, Debug)]
pub struct PasswordBuilder<C, S, B, H> {
    context: C,
    service: S,
    encrypt: B,
    hashed: H,
    repeat: Option<u8>,
}

pub trait Encrypter {
    fn encrypt(self, context_word: &[u8], service_word: &[u8]) -> Result<Box<[u8]>>;
}

/// Implementation of PasswordBuilder when nothing is set yet.
impl PasswordBuilder<NoContext, NoService, NoEncrypt, NoHashed> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Implementation of PasswordBuilder when it is already instantiated.
impl<C, S> PasswordBuilder<C, S, NoEncrypt, NoHashed> {
    /// Sets the unique word to build the passoword.
    pub fn context(
        self,
        word: impl Into<String>,
    ) -> PasswordBuilder<Context, S, NoEncrypt, NoHashed> {
        PasswordBuilder {
            context: Context(Box::from(word.into())),
            service: self.service,
            encrypt: self.encrypt,
            hashed: self.hashed,
            repeat: self.repeat,
        }
    }

    /// Sets the variable word to build the password.
    pub fn service(
        self,
        word: impl Into<String>,
    ) -> PasswordBuilder<C, Service, NoEncrypt, NoHashed> {
        PasswordBuilder {
            context: self.context,
            service: Service(Box::from(word.into())),
            encrypt: self.encrypt,
            hashed: self.hashed,
            repeat: self.repeat,
        }
    }
}

/// Scramble the context and service words when they are set.
impl PasswordBuilder<Context, Service, NoEncrypt, NoHashed> {
    pub fn encrypt(
        self,
        encrypt_method: impl Encrypter,
    ) -> Result<PasswordBuilder<Context, Service, Encrypt, NoHashed>> {
        let encrypted_result =
            encrypt_method.encrypt(self.context.0.as_bytes(), self.service.0.as_bytes())?;
        Ok(PasswordBuilder {
            context: self.context,
            service: self.service,
            encrypt: Encrypt(encrypted_result),
            hashed: self.hashed,
            repeat: self.repeat,
        })
    }
}

type HasherArg<'a> = &'a mut dyn DynDigest;

/// Implementation of PasswordBuilder when "context" and "service" fields are set
/// and they are scrambled
impl<H> PasswordBuilder<Context, Service, Encrypt, H> {
    pub fn hash(self, hasher: HasherArg) -> PasswordBuilder<Context, Service, Encrypt, Hashed> {
        hasher.update(&self.encrypt.0);
        let mut hash = hasher.finalize_reset();
        let mut encoded_hash = Base64::encode_string(&hash);

        if let Some(r) = self.repeat {
            for _ in 1..=r {
                encoded_hash = Base64::encode_string(&hash);
                hasher.update(encoded_hash.as_bytes());
                hash = hasher.finalize_reset();
            }
        }

        PasswordBuilder {
            hashed: Hashed(encoded_hash),
            context: self.context,
            service: self.service,
            encrypt: self.encrypt,
            repeat: None,
        }
    }

    /// Sets a number of repetitions to use on the following specified method.
    pub fn repeat(self, number: impl Into<u8>) -> Self {
        PasswordBuilder {
            repeat: Some(number.into()),
            ..self
        }
    }
}

impl PasswordBuilder<Context, Service, Encrypt, Hashed> {
    pub fn encode(self) -> String {
        self.hashed.0
    }
}
