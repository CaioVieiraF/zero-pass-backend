pub mod base64;
pub mod vigenere;
pub mod xor;

use crate::CipherResult;

pub use base64::b64;
pub use vigenere::vigenere;
pub use xor::xor;

#[derive(Debug, Clone, PartialEq)]
pub struct MethodArgs<'a> {
    pub word: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Methods<'a> {
    Vigenere(MethodArgs<'a>),
    B64(MethodArgs<'a>),
    Xor(MethodArgs<'a>),
}

pub fn gen_pass(method: &Methods) -> CipherResult {
    match method {
        Methods::Vigenere(args) => vigenere(args.word, args.password),
        Methods::B64(args) => b64(args.password),
        Methods::Xor(args) => xor(args.word, args.password),
    }
}
