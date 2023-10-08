use crate::model::Db;
use crate::model::UserMac;
use std::convert::Infallible;
use std::sync::Arc;
use serde_json::json;
use warp::Filter;
use warp::reply::Json;
use super::with_db;

pub fn user_rest_filters(base_path: &'static str, db: Arc<Db>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // /api/user
    let transaction_path = warp::path(base_path).and(warp::path("user"));
    let common = with_db(db.clone());

    let list = transaction_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(user_list);

    list
}

async fn user_list(db: Arc<Db>) -> Result<Json, warp::Rejection> {
    // TODO: Add proper error handling
    let users = UserMac::list(&db).await.unwrap();

    let response = json!({ "data": users });
    Ok(warp::reply::json(&response))
}

#[cfg(test)]
#[path = "../_tests/web_user.rs"]
mod tests;