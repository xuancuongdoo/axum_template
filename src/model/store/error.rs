pub type Result<T> = core::result::Result<T, Error>;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}
