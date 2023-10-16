#![allow(unused)]

mod structures;
mod endpoints;
mod db;

use std::env;
use std::sync::Arc;
use poem::{get, handler, listener::TcpListener, middleware::Tracing, web::Path, EndpointExt, Route, Server, endpoint};
use poem_openapi::OpenApiService;
use sqlx::postgres::PgPoolOptions;
use tokio::time::Duration;
use color_eyre::Result;
use crate::db::init_db;
use crate::endpoints::{AccountApi, UserApi};

const DEFAULT_WEB_FOLDER: &'static str = "web-folder/";
const DEFAULT_WEB_PORT: u16 = 8080;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = init_db().await?;
    let pool = Arc::new(pool);

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let api = OpenApiService::new((UserApi { pool: pool.clone() }, AccountApi { pool: pool.clone() }), "Personal Finances API", "0.0.1").server("http://localhost:8080/api");
    let explorer = api.openapi_explorer();
    let swagger = api.swagger_ui();
    let redoc = api.redoc();
    let rapidoc = api.rapidoc();

    let app = Route::new()
        .nest("/api", api)
        .nest("/", explorer)
        .nest("/redoc", redoc)
        .nest("/rapidoc", rapidoc)
        .nest("/swagger", swagger)
        .with(Tracing);

    let listener = TcpListener::bind("127.0.0.1:8080");
    let server = Server::new(listener).name("personal-finances");
    server.run_with_graceful_shutdown(
        app,
        async move {
            let _ = tokio::signal::ctrl_c().await;
        },
        Some(Duration::from_secs(5)),
    )
        .await?;
    Ok(())
}
