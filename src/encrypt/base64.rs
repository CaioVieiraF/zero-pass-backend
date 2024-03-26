use crate::prelude::*;
use crate::Method;

use std::sync::Arc;
use std::str::FromStr;
use async_trait::async_trait;

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(featue = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Base64;

impl FromStr for Base64 {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        if input.to_uppercase() == "BASE64" || input.to_uppercase() == "B64" {
            Ok(Base64)
        } else {
            Err(Error::InvalidMethodError(input.to_string()))
        }
    }
}

#[async_trait]
impl Method for Base64 {
    async fn encrypt(&self, uw: Arc<str>, _vw: Arc<str>) -> Result<String> {
        use base64::{engine::general_purpose, Engine as _};

        let encoded: String = general_purpose::STANDARD.encode(uw.as_bytes());

        Ok(encoded)
    }
}
