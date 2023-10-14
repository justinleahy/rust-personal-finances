use time::macros::datetime;
use uuid::uuid;
use time::OffsetDateTime;
use crate::model::db::init_db;
use crate::model::structures::transaction::{TransactionTypes, TransactionCategories, TransactionPatch, TransactionMac};
use crate::model::structures::account::AccountMac;
use crate::model::structures::user::UserMac;
use crate::security::user_context_from_token;

#[tokio::test]
async fn transactionmac_create() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;
    let account_id = uuid!("00000000-0000-0000-0000-000000000001");
    let account = AccountMac::get(&db, account_id).await?;
    let user = UserMac::get(&db, account.user_id).await?;
    let user_context = user_context_from_token(user.user_context.as_str()).await?;
    let transaction_date = OffsetDateTime::now_utc();
    let transaction_fx = TransactionPatch {
        transaction_date: Some(transaction_date),
        transaction_type: Some(TransactionTypes::Expense),
        category: Some(TransactionCategories::Expense),
        amount: Some(122.78),
        title: Some("Test Transaction".to_string()),
        vendor: None,
        comment: Some("Test Transaction".to_string())
    };
    
    // Action
    let transaction_created = TransactionMac::create(&db, account_id, transaction_fx.clone()).await?;

    // Check
    assert_eq!(account.id, transaction_created.account_id);
    assert_eq!(transaction_date.date(), transaction_created.transaction_date.date());
    assert_eq!(TransactionTypes::Expense, transaction_created.transaction_type);
    assert_eq!(TransactionCategories::Expense, transaction_created.category);
    assert_eq!(122.78, transaction_created.amount);
    assert_eq!("Test Transaction", transaction_created.title);
    assert_eq!(None, transaction_created.vendor);
    assert_eq!(Some("Test Transaction".to_string()), transaction_created.comment);
    
    Ok(())
}

#[tokio::test]
async fn transactionmac_list() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;
    let account_id = uuid!("00000000-0000-0000-0000-000000000001");

    // Action
    let transactions = TransactionMac::list(&db, account_id).await?;

    // Check
    assert_eq!(2, transactions.len());

    Ok(())
}

#[tokio::test]
async fn transactionmac_get() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;
    let transaction_id = uuid!("00000000-0000-0000-0000-000000000002");
    let account_id = uuid!("00000000-0000-0000-0000-000000000001");

    // Action
    let transaction = TransactionMac::get(&db, account_id, transaction_id).await?;

    // Check
    let account = AccountMac::get(&db, transaction.account_id).await?;
    let transaction_date = datetime!(2023-10-06 00:00:00 +00:00:00);

    assert_eq!(transaction_id, transaction.id);
    assert_eq!(account.id, transaction.account_id);
    assert_eq!(transaction_date, transaction.transaction_date);
    assert_eq!(TransactionTypes::Transfer, transaction.transaction_type);
    assert_eq!(TransactionCategories::Transfer, transaction.category);
    assert_eq!(899.66, transaction.amount);
    assert_eq!("Transfer", transaction.title);
    assert_eq!(None, transaction.vendor);
    assert_eq!("Transfer from Webull", transaction.comment.unwrap());

    Ok(())
}

#[tokio::test]
async fn transactionmac_update() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;
    let id = uuid!("00000000-0000-0000-0000-000000000002");
    let account_id = uuid!("00000000-0000-0000-0000-000000000001");
    let transaction_data_fx: TransactionPatch = TransactionPatch {
        transaction_date: None,
        transaction_type: None,
        category: None,
        amount: Some(700.55),
        title: None,
        vendor: None,
        comment: None
    };

    // Action
    let transaction_original = TransactionMac::get(&db, account_id, id).await?;
    let transaction_updated = TransactionMac::update(&db, account_id, id, transaction_data_fx).await?;

    // Check
    assert_eq!(transaction_original.id, transaction_updated.id);
    assert_eq!(transaction_original.account_id, transaction_updated.account_id);
    assert_eq!(transaction_original.transaction_date, transaction_updated.transaction_date);
    assert_eq!(transaction_original.transaction_type, transaction_updated.transaction_type);
    assert_eq!(700.55, transaction_updated.amount);
    assert_eq!(transaction_original.comment, transaction_updated.comment);

    Ok(())
}