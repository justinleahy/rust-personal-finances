#![allow(unused)]

use model::init_db;
use std::env;
use std::sync::Arc;
use web::start_web;

mod model;
mod security;
mod web;

const DEFAULT_WEB_FOLDER: &'static str = "web-folder/";
const DEFAULT_WEB_PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    // Compute web_folder
    let mut args: Vec<String> = env::args().collect();
    let web_folder = args.pop().unwrap_or_else(|| DEFAULT_WEB_FOLDER.to_string());
    let web_port = DEFAULT_WEB_PORT;

    // Get the database
    // TODO - Loop until valid DB
    let db = init_db().await.expect("connect init db");
    let db = Arc::new(db);

    // Start the server
    match start_web(&web_folder, web_port, db).await {
        Ok(_) => println!("Server ended"),
        Err(ex) => println!("ERROR - Web server failed to start. Cause {:?}", ex),
    }
}