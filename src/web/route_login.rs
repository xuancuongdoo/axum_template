use crate::{Error, Result};
use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{json, Value};


pub fn routes() -> Router {
    Router::new().route("/api/login" , post(api_login))
}


async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("-> {:<12} -  api_login - {payload:?}", "HANDLER");

    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }
    // Set Cookies
    let body = Json(json!({
        "result" : {
            "success" : true
    }
    }));
    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
