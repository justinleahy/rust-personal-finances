use crate::model::Db;
use crate::model::AccountMac;
use std::convert::Infallible;
use std::sync::Arc;
use serde_json::json;
use uuid::Uuid;
use warp::Filter;
use warp::reply::Json;
use super::json_response;
use super::with_db;
use crate::model::AccountPatch;

pub fn account_rest_filters(base_path: &'static str, db: Arc<Db>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // /api/account
    let account_path = warp::path(base_path).and(warp::path("account"));
    let common = with_db(db.clone());

    // LIST accounts `GET /api/account/`
    let list = account_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(account_list);

    // GET account `GET /api/account/{id}`
    let get = account_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(account_get);

    // CREATE account `POST /api/account with body AccountPatch`
    let create = account_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(account_create);

    // UPDATE account `PATCH /api/account/{id} with body AccountPatch`
    let update = account_path
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(account_update);

    // DELETE account `DELETE /api/account/{id}
    let delete = account_path
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(account_delete);

    list.or(get).or(create).or(update).or(delete)
}

async fn account_list(db: Arc<Db>) -> Result<Json, warp::Rejection> {
    // TODO: Add proper error handling
    let accounts = AccountMac::list(&db).await.unwrap();
    json_response(accounts)
}

async fn account_get(db: Arc<Db>, id: Uuid) -> Result<Json, warp::Rejection> {
    let account = AccountMac::get(&db, id).await?;
    json_response(account)
}

async fn account_create(db: Arc<Db>, patch: AccountPatch) -> Result<Json, warp::Rejection> {
    let account = AccountMac::create(&db, patch).await?;
    json_response(account)
}

async fn account_update(db: Arc<Db>, id: Uuid, patch: AccountPatch) -> Result<Json, warp::Rejection> {
    let account = AccountMac::update(&db, id, patch).await?;
    json_response(account)
}

async fn account_delete(db: Arc<Db>, id: Uuid) -> Result<Json, warp::Rejection> {
    let account = AccountMac::delete(&db, id).await?;
    json_response(account)
}

#[cfg(test)]
#[path = "../_tests/web_account.rs"]
mod tests;