pub use self::error::{Error, Result};
pub use config::Config;

use crate::models::ModelController;

use axum::response::Response;
use axum::routing::get_service;
use axum::{middleware, Router};

use serde::Deserialize;

use std::net::SocketAddr;

use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

use tracing::info;
use tracing_subscriber::EnvFilter;
use web::{mw_auth::mw_require_auth, route_login, route_tickets};

mod config;
mod ctx;
mod error;
mod models;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mc = ModelController::new().await?;

    let routes_apis =
        route_tickets::routes(mc.clone()).route_layer(middleware::from_fn(mw_require_auth));

    let routes_all = Router::new()
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
    info!("--> LISTENING ON {:?}", addr);
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    info!("-> {:<12} -  main_response_mapper - {res:?}", "HANDLER");
    res
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
