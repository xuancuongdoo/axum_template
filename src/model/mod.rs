mod error;
pub mod models;
mod store;
pub use crate::error::{Error, Result};

use self::store::{new_db_pool, Db};

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
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;
        Ok(ModelManager { db })
    }
}
