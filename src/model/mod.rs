mod base;
mod error;
pub mod models;
mod store;
mod user;

pub use self::error::{Error, Result};
pub mod task;

use crate::model::store::{new_db_pool, Db};

// Model Layer
//
// Design
//
// - The model layer normalizes the applications' data type
// structure and access
//
// - All application code data access must go through the model layer
//
// - Model Controller ( e.g . 'taskBmc' , 'projectBmc') implement
// CRUD and other data access methods on the given "entity"
// (e.g., 'Task' , 'Project')
//
// (`Bmc` is short for Backend Model Controller)
// - In framework like Axum, Tauri, `ModelManager` are typically used as App State
//
// ModelManager are designed to be passed as an argurment
//  to all  Model Controllers functions
//
//

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    /// Constructor
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;

        Ok(ModelManager { db })
    }

    /// Returns the sqlx db pool reference.
    /// (Only for the model layer)
    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}
