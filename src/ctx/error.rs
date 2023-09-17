use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Debug)]
pub enum Error {
    CtxCannotNewRootCtx,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
