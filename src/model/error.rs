pub type Result<T> = core::result::Result<T, Error>;
use crate::model::store;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Error {
    Store(store::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<store::Error> for Error {
    fn from(val: store::Error) -> Self {
        Self::Store(val)
    }
}
