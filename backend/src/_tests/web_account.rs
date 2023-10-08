use anyhow::{Result, Context};
use serde_json::{Value, from_str, from_value};
use uuid::uuid;
use std::str::from_utf8;
use crate::init_db;
use crate::model::Account;
use crate::web::account::account_rest_filters;
use serde::Deserialize;
use std::sync::Arc;
use warp::hyper::{Response, body::Bytes};

#[tokio::test]
async fn account_list() -> Result<()> {
    // Fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let account_apis = account_rest_filters("api", db.clone());

    // Action
    let resp = warp::test::request()
        .method("GET")
        .path("/api/account")
        .reply(&account_apis)
        .await;

    // Check
    assert_eq!(200, resp.status(), "http status");

    // Extract response data
    let accounts: Vec<Account> = extract_body_data(resp)?;

    assert_eq!(1, accounts.len());

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