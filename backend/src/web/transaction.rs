use crate::model::Db;
use crate::model::TransactionMac;
use std::convert::Infallible;
use std::sync::Arc;
use serde_json::json;
use warp::Filter;
use warp::reply::Json;
use super::json_response;
use super::with_db;

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
    let transactions = TransactionMac::list(&db).await?;
    json_response(transactions)
}

#[cfg(test)]
#[path = "../_tests/web_transaction.rs"]
mod tests;