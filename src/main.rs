pub use self::error::{Error, Result};
use crate::models::ModelController;
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Router};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use web::{mw_auth::mw_require_auth, route_login, route_tickets};

mod ctx;
mod error;
mod models;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_apis =
        route_tickets::routes(mc.clone()).route_layer(middleware::from_fn(mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(route_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("--> LISTENING ON {:?}", addr);
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("-> {:<12} -  main_response_mapper - {res:?}", "HANDLER");
    res
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("-> {:<12} -  handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("Worl");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("-> {:<12} -  handler_hello2 - {name:?}", "HANDLER");
    Html(format!("Hello <strong>{name}</strong>"))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
