use anyhow::{Result, Context};
use serde_json::{Value, from_str, from_value, json};
use uuid::uuid;
use std::str::from_utf8;
use crate::init_db;
use crate::model::{Transaction, TransactionTypes, TransactionCategories};
use crate::web::transaction::transaction_rest_filters;
use crate::web::handle_rejection;
use serde::Deserialize;
use std::sync::Arc;
use warp::hyper::{Response, body::Bytes};
use warp::Filter;
use time::macros::datetime;

#[tokio::test]
async fn transaction_list() -> Result<()> {
    // Fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let transaction_apis = transaction_rest_filters("api", db.clone()).recover(handle_rejection);

    // Action
    let resp = warp::test::request()
        .method("GET")
        .path("/api/account/00000000-0000-0000-0000-000000000001/transaction")
        .reply(&transaction_apis)
        .await;

    // Check
    assert_eq!(200, resp.status(), "http status");

    // Extract response data
    let transactions: Vec<Transaction> = extract_body_data(resp)?;
    let transaction_date = datetime!(2023-10-07 00:00:00 +00:00:00);

    assert_eq!(2, transactions.len());
    assert_eq!(uuid!("00000000-0000-0000-0000-000000000003"), transactions[0].id);
    assert_eq!(uuid!("00000000-0000-0000-0000-000000000001"), transactions[0].account_id);
    assert_eq!(transaction_date, transactions[0].transaction_date);
    assert_eq!(TransactionTypes::Expense, transactions[0].transaction_type);
    assert_eq!(TransactionCategories::Expense, transactions[0].category);
    assert_eq!(100.43, transactions[0].amount);
    assert_eq!("Groceries", transactions[0].title);
    assert_eq!(Some("Tops".to_string()), transactions[0].vendor);
    assert_eq!(None, transactions[0].comment);

    Ok(())
}

#[tokio::test]
async fn transaction_get() -> Result<()> {
    // Fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let transaction_apis = transaction_rest_filters("api", db).recover(handle_rejection);

    // Action
    let resp = warp::test::request()
        .method("GET")
        .path("/api/account/00000000-0000-0000-0000-000000000001/transaction/00000000-0000-0000-0000-000000000003")
        .reply(&transaction_apis)
        .await;

    // Check
    assert_eq!(200, resp.status(), "http status");

    // Extract response data
    let transaction: Transaction = extract_body_data(resp)?;
    let transaction_date = datetime!(2023-10-07 00:00:00 +00:00:00);

    assert_eq!(uuid!("00000000-0000-0000-0000-000000000003"), transaction.id);
    assert_eq!(uuid!("00000000-0000-0000-0000-000000000001"), transaction.account_id);
    assert_eq!(transaction_date, transaction.transaction_date);
    assert_eq!(TransactionTypes::Expense, transaction.transaction_type);
    assert_eq!(TransactionCategories::Expense, transaction.category);
    assert_eq!(100.43, transaction.amount);
    assert_eq!("Groceries", transaction.title);
    assert_eq!(Some("Tops".to_string()), transaction.vendor);
    assert_eq!(None, transaction.comment);

    Ok(())
}

#[tokio::test]
async fn transaction_create() -> Result<()> {
    // Fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let transaction_apis = transaction_rest_filters("api", db).recover(handle_rejection);
    let body = json!({
        "account_id": "00000000-0000-0000-0000-000000000001",
        "transaction_date": [2023, 282, 0, 0, 0, 0, 0, 0, 0],
        "transaction_type": "Expense",
        "category": "Expense",
        "amount": 150.60,
        "title": "New Mouse"
    });
    
    // Action
    let resp = warp::test::request()
        .method("POST")
        .path("/api/account/00000000-0000-0000-0000-000000000001/transaction")
        .json(&body)
        .reply(&transaction_apis)
        .await;

    // Check
    assert_eq!(200, resp.status(), "http status");

    let transaction: Transaction = extract_body_data(resp)?;
    let transaction_date = datetime!(2023-10-09 00:00:00 +00:00:00);

    assert_eq!(uuid!("00000000-0000-0000-0000-000000000001"), transaction.account_id);
    assert_eq!(transaction_date, transaction.transaction_date);
    assert_eq!(TransactionTypes::Expense, transaction.transaction_type);
    assert_eq!(TransactionCategories::Expense, transaction.category);
    assert_eq!(150.60, transaction.amount);
    assert_eq!("New Mouse", transaction.title);
    assert_eq!(None, transaction.vendor);
    assert_eq!(None, transaction.comment);

    Ok(())
}

#[tokio::test]
async fn transaction_update() -> Result<()> {
    // Fixture
    let db = init_db().await?;
    let db = Arc::new(db);
    let transaction_apis = transaction_rest_filters("api", db).recover(handle_rejection);
    let body = json!({
        "title": "Updated Title"
    });
    let original_transaction_resp = warp::test::request()
        .method("GET")
        .path("/api/account/00000000-0000-0000-0000-000000000001/transaction/00000000-0000-0000-0000-000000000003")
        .reply(&transaction_apis)
        .await;

    assert_eq!(200, original_transaction_resp.status(), "http status");

    let original_transaction: Transaction = extract_body_data(original_transaction_resp)?;

    // Action
    let resp = warp::test::request()
        .method("PATCH")
        .path("/api/account/00000000-0000-0000-0000-000000000001/transaction/00000000-0000-0000-0000-000000000003")
        .json(&body)
        .reply(&transaction_apis)
        .await;

    // Check
    assert_eq!(200, resp.status(), "http status");

    let transaction: Transaction = extract_body_data(resp)?;

    assert_eq!(original_transaction.id, transaction.id);
    assert_eq!(original_transaction.account_id, transaction.account_id);
    assert_eq!(original_transaction.transaction_type, transaction.transaction_type);
    assert_eq!(original_transaction.category, transaction.category);
    assert_eq!(original_transaction.amount, transaction.amount);
    assert_ne!(original_transaction.title, transaction.title);
    assert_eq!("Updated Title", transaction.title);
    assert_eq!(original_transaction.vendor, transaction.vendor);
    assert_eq!(original_transaction.comment, transaction.comment);

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