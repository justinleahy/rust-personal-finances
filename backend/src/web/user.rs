use crate::model::Db;
use crate::model::UserMac;
use crate::model::UserPatch;
use std::convert::Infallible;
use std::sync::Arc;
use serde_json::json;
use warp::Filter;
use warp::reply::Json;
use super::json_response;
use super::with_db;
use uuid::Uuid;

pub fn user_rest_filters(base_path: &'static str, db: Arc<Db>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // /api/user
    let user_path = warp::path(base_path).and(warp::path("user"));
    let common = with_db(db.clone());

    // LIST users `GET /api/user`
    let list = user_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(user_list);

    // GET user `GET /api/user/{id}`
    let get = user_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(user_get);

    // CREATE user `POST /api/user with body UserPatch`
    let create = user_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(user_create);

    // UPDATE user `PATCH /api/user/{id} with body UserPatch`
    let update = user_path
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(user_update);

    // DELETE user `DELETE /api/user/{id}`
    let delete = user_path
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(user_delete);

    list.or(get).or(create).or(update).or(delete)
}

async fn user_list(db: Arc<Db>) -> Result<Json, warp::Rejection> {
    let users = UserMac::list(&db).await?;
    json_response(users)
}

async fn user_get(db: Arc<Db>, id: Uuid) -> Result<Json, warp::Rejection> {
    let user = UserMac::get(&db, id).await?;
    json_response(user)
}

async fn user_create(db: Arc<Db>, patch: UserPatch) -> Result<Json, warp::Rejection> {
    let user = UserMac::create(&db, patch).await?;
    json_response(user)
}

async fn user_update(db: Arc<Db>, id: Uuid, patch: UserPatch) -> Result<Json, warp::Rejection> {
    let user = UserMac::update(&db, id, patch).await?;
    json_response(user)
}

async fn user_delete(db: Arc<Db>, id: Uuid) -> Result<Json, warp::Rejection> {
    let user = UserMac::delete(&db, id).await?;
    json_response(user)
}

#[cfg(test)]
#[path = "../_tests/web_user.rs"]
mod tests;