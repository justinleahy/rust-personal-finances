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
    //let transaction_path = warp::path(base_path).and(warp::path("account")).and(warp::path::param::<Uuid>());
    let transaction_path = warp::path!("api" / "account" / Uuid / "transaction");
    let common = with_db(db.clone());

    // LIST transactions `GET /api/account{id}/transaction/{id}`
    let list = transaction_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::end())
        .and_then(transaction_list);

    // GET transaction `GET /api/account/{id}/transaction/{id}`
    let get = transaction_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param::<Uuid>())
        .and_then(transaction_get);

    // CREATE transaction `POST /api/account/{id}/transaction with body TransactionPatch`
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

    /*
    // DELETE transaction `DELETE /api/transaction/{id}`
    let delete = transaction_path
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(transaction_delete);

    list.or(get).or(create).or(update).or(delete)
    */
    list.or(get).or(create).or(update)
}

async fn transaction_list(account_id: Uuid, db: Arc<Db>) -> Result<Json, warp::Rejection> {
    let transactions = TransactionMac::list(&db, account_id).await?;
    json_response(transactions)
}

async fn transaction_get(account_id: Uuid, db: Arc<Db>, transaction_id: Uuid) -> Result<Json, warp::Rejection> {
    let transaction = TransactionMac::get(&db, account_id, transaction_id).await?;
    json_response(transaction)
}

async fn transaction_create(account_id: Uuid, db: Arc<Db>, patch: TransactionPatch) -> Result<Json, warp::Rejection> {
    let transaction = TransactionMac::create(&db, account_id, patch).await?;
    json_response(transaction)
}

async fn transaction_update(account_id: Uuid, db: Arc<Db>, id: Uuid, patch: TransactionPatch) -> Result<Json, warp::Rejection> {
    let transaction = TransactionMac::update(&db, account_id, id, patch).await?;
    json_response(transaction)
}

async fn transaction_delete(db: Arc<Db>, id: Uuid) -> Result<Json, warp::Rejection> {
    let transaction = TransactionMac::delete(&db, id).await?;
    json_response(transaction)
}

#[cfg(test)]
#[path = "../_tests/web_transaction.rs"]
mod tests;