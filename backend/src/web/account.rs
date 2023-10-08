use crate::model::Db;
use crate::model::AccountMac;
use std::convert::Infallible;
use std::sync::Arc;
use serde_json::json;
use warp::Filter;
use warp::reply::Json;
use super::with_db;

pub fn account_rest_filters(base_path: &'static str, db: Arc<Db>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // /api/account
    let transaction_path = warp::path(base_path).and(warp::path("account"));
    let common = with_db(db.clone());

    let list = transaction_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(account_list);

    list
}

async fn account_list(db: Arc<Db>) -> Result<Json, warp::Rejection> {
    // TODO: Add proper error handling
    let accounts = AccountMac::list(&db).await.unwrap();

    let response = json!({ "data": accounts });
    Ok(warp::reply::json(&response))
}
