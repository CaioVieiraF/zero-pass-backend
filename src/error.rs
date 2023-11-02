#[derive(Debug, Clone, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Error {
    #[error("The inserted character is invalid!")]
    InvalidCharacterError,
    #[error("The method `{0}` does not exist")]
    InvalidMethodError(String),
}
