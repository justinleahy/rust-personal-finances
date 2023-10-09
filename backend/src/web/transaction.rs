use crate::model::Db;
use crate::model::TransactionMac;
use crate::model::TransactionPatch;
use std::convert::Infallible;
use std::sync::Arc;
use serde_json::json;
use warp::Filter;
use warp::reply::Json;
use super::json_response;
use super::with_db;
use uuid::Uuid;

pub fn transaction_rest_filters(base_path: &'static str, db: Arc<Db>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // /api/transaction
    let transaction_path = warp::path(base_path).and(warp::path("transaction"));
    let common = with_db(db.clone());

    // LIST transactions `GET /api/transaction`
    let list = transaction_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(transaction_list);

    // GET transaction `GET /api/transaction/{id}`
    let get = transaction_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(transaction_get);

    // CREATE transaction `POST /api/transaction with body TransactionPatch`
    let create = transaction_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(transaction_create);

    // UPDATE transaction `PATCH /api/transaction/{id} with body TransactionPatch`
    let update = transaction_path
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(transaction_update);

    // DELETE transaction `DELETE /api/transaction/{id}`
    let delete = transaction_path
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(transaction_delete);

    list.or(get).or(create).or(update).or(delete)
}

async fn transaction_list(db: Arc<Db>) -> Result<Json, warp::Rejection> {
    let transactions = TransactionMac::list(&db).await?;
    json_response(transactions)
}

async fn transaction_get(db: Arc<Db>, id: Uuid) -> Result<Json, warp::Rejection> {
    let transaction = TransactionMac::get(&db, id).await?;
    json_response(transaction)
}

async fn transaction_create(db: Arc<Db>, patch: TransactionPatch) -> Result<Json, warp::Rejection> {
    let transaction = TransactionMac::create(&db, patch).await?;
    json_response(transaction)
}

async fn transaction_update(db: Arc<Db>, id: Uuid, patch: TransactionPatch) -> Result<Json, warp::Rejection> {
    let transaction = TransactionMac::update(&db, id, patch).await?;
    json_response(transaction)
}

async fn transaction_delete(db: Arc<Db>, id: Uuid) -> Result<Json, warp::Rejection> {
    let transaction = TransactionMac::delete(&db, id).await?;
    json_response(transaction)
}

#[cfg(test)]
#[path = "../_tests/web_transaction.rs"]
mod tests;