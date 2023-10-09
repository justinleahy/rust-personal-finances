use anyhow::{Result, Context};
use serde_json::{Value, from_str, from_value};
use uuid::uuid;
use std::str::from_utf8;
use crate::init_db;
use crate::model::{Account, AccountTypes, InterestFrequencyUnits};
use crate::web::account::account_rest_filters;
use crate::web::handle_rejection;
use serde::Deserialize;
use std::sync::Arc;
use warp::hyper::{Response, body::Bytes};
use warp::Filter;
use serde_json::json;

#[tokio::test]
async fn account_list() -> Result<()> {
    // Fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let account_apis = account_rest_filters("api", db.clone()).recover(handle_rejection);

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

#[tokio::test]
async fn account_get() -> Result<()> {
    // Fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let account_apis = account_rest_filters("api", db).recover(handle_rejection);

    // Action
    let resp = warp::test::request()
        .method("GET")
        .path("/api/account/00000000-0000-0000-0000-000000000001")
        .reply(&account_apis)
        .await;
    
    // Check
    assert_eq!(200, resp.status(), "http status");

    // Extract response data
    let account: Account = extract_body_data(resp)?;

    assert_eq!(uuid!("00000000-0000-0000-0000-000000000001"), account.id);
    assert_eq!(uuid!("00000000-0000-0000-0000-000000000000"), account.user_id);
    assert_eq!(AccountTypes::Checking, account.account_type);
    assert_eq!("Main Checking", account.nickname);
    assert_eq!(0.0009995, account.interest);
    assert_eq!(1, account.interest_frequency);
    assert_eq!(InterestFrequencyUnits::Day, account.interest_frequency_unit);

    Ok(())
}

#[tokio::test]
async fn account_create() -> Result<()> {
    // Fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let account_apis = account_rest_filters("api", db).recover(handle_rejection);
    let body = json!({
        "user_id": "00000000-0000-0000-0000-000000000000",
        "account_type": "Savings",
        "nickname": "Ally Savings",
        "interest": 0.0456,
        "interest_frequency": 1,
        "interest_frequency_unit": "Day"
    });

    // Action
    let resp = warp::test::request()
        .method("POST")
        .path("/api/account")
        .json(&body)
        .reply(&account_apis)
        .await;

    // Check
    assert_eq!(200, resp.status(), "http status");

    let account: Account = extract_body_data(resp)?;

    assert_eq!(AccountTypes::Savings, account.account_type);
    assert_eq!("Ally Savings", account.nickname);
    assert_eq!(0.0456, account.interest);
    assert_eq!(1, account.interest_frequency);
    assert_eq!(InterestFrequencyUnits::Day, account.interest_frequency_unit);

    Ok(())
}

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