use thiserror::Error as ThisError;

use crate::model::Db;
use crate::model;
use crate::web::account::account_rest_filters;
use crate::web::transaction::transaction_rest_filters;
use crate::web::user::user_rest_filters;
use std::path::Path;
use std::sync::Arc;
use warp::Filter;
use std::convert::Infallible;
use warp::Reply;
use warp::Rejection;
use serde_json::json;
use serde::Serialize;
use warp::reply::Json;

mod transaction;
mod account;
mod user;

pub async fn start_web(web_folder: &str, web_port: u16, db: Arc<Db>) -> Result<(), Error> {
    // Validate web_folder
    if !Path::new(web_folder).exists() {
        return Err(Error::FailStartWebFolderNotFound((web_folder.to_string())));
    }

    // I don't think I should do db.clone() for these but I'm not sure where to proceed
    let user_apis = user_rest_filters("api", db.clone());
    let account_apis = account_rest_filters("api", db.clone());
    let transaction_apis = transaction_rest_filters("api", db.clone());

    // Static content
    let content = warp::fs::dir(web_folder.to_string());
    let root_index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", web_folder)));
    let static_site = content.or(root_index);

    // Combine all routes, ordering matters. Routes with a longer url length should come first.
    let routes = transaction_apis.or(account_apis).or(user_apis).or(static_site).recover(handle_rejection);

    println!("Start 127.0.0.1:{} at {}", web_port, web_folder);
    warp::serve(routes).run(([127, 0, 0, 1], web_port)).await;

    Ok(())
}

async fn handle_rejection(rejection_error: Rejection) -> Result<impl Reply, Infallible> {
    // Print to server side
    println!("Error - {:?}", rejection_error);

    // TODO - Call log API for capture and store

    // Build user message
    let user_message = match rejection_error.find::<WebErrorMessage>() {
        Some(err) => err.error_type.to_string(),
        None => "Unknown".to_string()
    };

    let result = json!({ "errorMessage" : user_message });
    let result = warp::reply::json(&result);

    Ok(warp::reply::with_status(result, warp::http::StatusCode::BAD_REQUEST))
}

pub fn with_db(db: Arc<Db>) -> impl Filter<Extract = (Arc<Db>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!({ "data": data });
    Ok(warp::reply::json(&response))
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Web server failed to start because web-folder '{0}' not found.")]
    FailStartWebFolderNotFound(String)
}

// region: Warp Custom Error
#[derive(Debug)]
pub struct WebErrorMessage {
    pub error_type: &'static str,
    pub message: String
}
impl warp::reject::Reject for WebErrorMessage {}

impl WebErrorMessage {
    pub fn rejection(error_type: &'static str, message: String) -> warp::Rejection {
        warp::reject::custom(WebErrorMessage{ error_type, message })
    }
}

impl From<self::Error> for warp::Rejection {
    fn from(other: self::Error) -> Self {
        WebErrorMessage::rejection("web::Error", format!("{}", other))
    }
}

impl From<model::Error> for warp::Rejection {
    fn from(other: model::Error) -> Self {
        WebErrorMessage::rejection("model::Error", format!("{}", other))
    }
}
// endregion: Warp Custom Error