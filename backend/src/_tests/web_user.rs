use anyhow::{Result, Context};
use serde_json::{Value, from_str, from_value};
use uuid::uuid;
use std::str::from_utf8;
use crate::init_db;
use crate::model::User;
use crate::web::user::user_rest_filters;
use crate::web::handle_rejection;
use serde::Deserialize;
use std::sync::Arc;
use warp::hyper::{Response, body::Bytes};
use warp::Filter;

#[tokio::test]
async fn user_list() -> Result<()> {
    // Fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let user_apis = user_rest_filters("api", db.clone()).recover(handle_rejection);

    // Action
    let resp = warp::test::request()
        .method("GET")
        .path("/api/user")
        .reply(&user_apis)
        .await;

    // Check
    assert_eq!(200, resp.status(), "http status");

    // Extract response data
    let users: Vec<User> = extract_body_data(resp)?;

    assert_eq!(1, users.len());
    assert_eq!(uuid!("00000000-0000-0000-0000-000000000000"), users[0].id);

    Ok(())
}

// region: Web Test Utils

fn extract_body_data<D>(resp: Response<Bytes>) -> Result<D> where for <'de> D: Deserialize<'de> {
    // Parse the body as serde_json::Value
    let body = from_utf8(resp.body())?;
    let mut body: Value = from_str(body).with_context(|| format!("Connot parse resp.body to JSON. resp.body: '{}'", body))?;

    // Extract the data
    let data = body["data"].take();

    // Deserialize the data to D
    let data: D = from_value(data)?;

    Ok(data)
}

// endregion: Web Test Utils