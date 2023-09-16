pub type Result<T> = core::result::Result<T, Error>;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Error {
    FailToCreatePool(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
