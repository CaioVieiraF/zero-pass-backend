pub mod encrypt;
pub mod error;
pub mod prelude;

use crate::encrypt::*;

use digest::{Digest, DynDigest};
use prelude::*;
use sha3::Sha3_256;

#[derive(Debug, Default)]
pub struct Method;

pub fn hash_from_string(name: impl Into<String>) -> Result<Box<dyn DynDigest>> {
    let hash_name: String = name.into();

    let method: Box<dyn DynDigest> = match hash_name.as_str() {
        "md5" => Box::new(md5::Md5::default()),
        "sha1" => Box::new(sha1::Sha1::default()),
        "sha224" => Box::new(sha2::Sha224::default()),
        "sha256" => Box::new(sha2::Sha256::default()),
        "sha384" => Box::new(sha2::Sha384::default()),
        "sha512" => Box::new(sha2::Sha512::default()),
        "sha3_256" => Box::new(sha3::Sha3_256::default()),
        _ => {
            return Err(Error::InvalidMethodError(hash_name));
        }
    };

    Ok(method)
}

impl Method {
    pub fn get_list() -> [Box<impl Digest>; 1] {
        [Box::new(Sha3_256::new())]
    }
}

#[cfg(test)]
mod tests;
