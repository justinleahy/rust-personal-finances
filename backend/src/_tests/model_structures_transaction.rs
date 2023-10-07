use time::macros::datetime;
use uuid::uuid;
use time::OffsetDateTime;
use crate::model::db::init_db;
use crate::model::structures::transaction::{TransactionTypes, TransactionCategories, TransactionPatch, TransactionMac};

#[tokio::test]
async fn transactionmac_create() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;
    let account_id = uuid!("00000000-0000-0000-0000-000000000001");
    let transaction_date = OffsetDateTime::now_utc();
    let transaction_fx = TransactionPatch {
        account_id: Some(account_id),
        transaction_date: Some(transaction_date),
        transaction_type: Some(TransactionTypes::Expense),
        category: Some(TransactionCategories::Expense),
        transaction_integer: Some(1),
        transaction_decimal: Some(2278),
        transaction_exponent: Some(2),
        comment: Some("Test Transaction".to_string())
    };
    
    // Action
    let transaction_created = TransactionMac::create(&db, transaction_fx.clone()).await?;

    // Check
    assert_eq!(account_id, transaction_created.account_id);
    assert_eq!(transaction_date.date(), transaction_created.transaction_date.date());
    assert_eq!(TransactionTypes::Expense, transaction_created.transaction_type);
    assert_eq!(TransactionCategories::Expense, transaction_created.category);
    assert_eq!(1, transaction_created.transaction_integer);
    assert_eq!(2278, transaction_created.transaction_decimal);
    assert_eq!(2, transaction_created.transaction_exponent);
    assert_eq!("Test Transaction", transaction_created.comment);
    
    Ok(())
}

#[tokio::test]
async fn transactionmac_list() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;

    // Action
    let transactions = TransactionMac::list(&db).await?;
    
    // Check
    assert_eq!(2, transactions.len());

    Ok(())
}

#[tokio::test]
async fn transactionmac_get() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;

    let id = uuid!("00000000-0000-0000-0000-000000000002");

    // Action
    let transaction = TransactionMac::get(&db, id).await?;

    // Check
    let account_id = uuid!("00000000-0000-0000-0000-000000000001");
    let transaction_date = datetime!(2023-10-06 00:00:00 +00:00:00);

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

#[tokio::test]
async fn transactionmac_update() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;
    let id = uuid!("00000000-0000-0000-0000-000000000002");
    let transaction_data_fx: TransactionPatch = TransactionPatch {
        account_id: None,
        transaction_date: None,
        transaction_type: None,
        category: None,
        transaction_integer: Some(7),
        transaction_decimal: None,
        transaction_exponent: None,
        comment: None
    };

    // Action
    let transaction_original = TransactionMac::get(&db, id).await?;

    Ok(())
}