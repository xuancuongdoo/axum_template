mod error;
pub mod models;
pub use crate::error::{Error, Result};

#[derive(Clone)]
pub struct ModelManager {}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        Ok(ModelManager {})
    }
}
