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

pub fn gen_pass(method: &Methods, repeat: Option<u8>) -> CipherResult {
    let encryption_layers = match repeat {
        Some(i) => {
            if i < 1 {
                1
            } else {
                i
            }
        }
        None => 1,
    };
    let mut result: CipherResult = Ok(String::new());

    for _ in 0..encryption_layers {
        result = match method {
            Methods::Vigenere(args) => {
                if result == Ok(String::new()) {
                    result = Ok(args.word.to_owned());
                }
                vigenere(result.unwrap().as_str(), args.password)
            }
            Methods::B64(args) => {
                if result == Ok(String::new()) {
                    result = Ok(args.password.to_owned());
                }
                b64(result.unwrap().as_str())
            }
            Methods::Xor(args) => {
                if result == Ok(String::new()) {
                    result = Ok(args.word.to_owned());
                }
                xor(result.unwrap().as_str(), args.password)
            }
        }
    }

    result
}
