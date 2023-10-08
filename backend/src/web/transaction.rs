use crate::model::Db;
use crate::model::TransactionMac;
use std::convert::Infallible;
use std::sync::Arc;
use serde_json::json;
use warp::Filter;
use warp::reply::Json;

pub fn transaction_rest_filters(base_path: &'static str, db: Arc<Db>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // /api/transaction
    let transaction_path = warp::path(base_path).and(warp::path("transaction"));
    let common = with_db(db.clone());

    let list = transaction_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(transaction_list);

    list
}

async fn transaction_list(db: Arc<Db>) -> Result<Json, warp::Rejection> {
    // TODO: Add proper error handling
    let transactions = TransactionMac::list(&db).await.unwrap();

    let response = json!({ "data": transactions });
    Ok(warp::reply::json(&response))
}

// region: Filter Utils

pub fn with_db(db: Arc<Db>) -> impl Filter<Extract = (Arc<Db>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

// endregion: Filter Utils
