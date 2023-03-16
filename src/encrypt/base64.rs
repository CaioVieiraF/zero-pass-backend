use crate::{CipherError, CipherResult, Method};
use serde::Serialize;
use std::str::FromStr;

#[derive(Serialize, Clone, Debug)]
pub struct Base64;

impl FromStr for Base64 {
    type Err = CipherError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.to_uppercase() == "BASE64" || input.to_uppercase() == "B64" {
            Ok(Base64)
        } else {
            Err(CipherError::InvalidMethodError)
        }
    }
}

impl Method for Base64 {
    fn encrypt(&self, uw: String, _vw: String) -> CipherResult {
        use base64::{engine::general_purpose, Engine as _};

        let encoded: String = general_purpose::STANDARD.encode(uw.as_bytes());

        Ok(encoded)
    }
}
