use crate::{Error, Result};
use serde::Deserialize;
use axum::Json;
use serde_json::Value;

async fn api_login(payload : Json<LoginPayload>) -> Result<Json<Value>> {
    todo!()
} 

#[derive(Debug)]

