use std::ptr::null;
use uuid::uuid;
use time::Date;
use crate::model::db::init_db;
use crate::model::structures::transaction::{TransactionMac, TransactionTypes, TransactionCategories};

#[tokio::test]
async fn model_finances_transactionmac_list() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;

    // Action
    let transactions = TransactionMac::list(&db).await?;
    
    // Check
    assert_eq!(2, transactions.len());

    Ok(())
}

#[tokio::test]
async fn model_finances_transactionmac_get() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;

    let id = uuid!("00000000-0000-0000-0000-000000000002");

    // Action
    let transaction = TransactionMac::get(&db, id).await?;

    // Check
    let account_id = uuid!("00000000-0000-0000-0000-000000000001");
    let transaction_date = Date::from_calendar_date(2023, time::Month::October, 6)?;

    assert_eq!(id, transaction.id);
    assert_eq!(account_id, transaction.account_id);
    assert_eq!(transaction_date, transaction.transaction_date);
    assert_eq!(TransactionTypes::Transfer, transaction.transaction_type);
    assert_eq!(TransactionCategories::Transfer, transaction.category);
    assert_eq!(8, transaction.transaction_integer);
    assert_eq!(9966, transaction.transaction_decimal);
    assert_eq!(2, transaction.transaction_exponent);
    assert_eq!("Webull Transfer", transaction.comment);

    Ok(())
}