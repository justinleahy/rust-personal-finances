use std::ptr::null;
use uuid::uuid;
use time::Date;
use crate::model::db::init_db;
use crate::model::finances::{UserMac, AccountMac, TransactionMac};
use crate::model::finances::{AccountTypes, InterestFrequencyUnits};

#[tokio::test]
async fn model_finances_usermac_list() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;
    
    // Action
    let users = UserMac::list(&db).await?;

    // Check
    assert_eq!(1, users.len());

    Ok(())
}

#[tokio::test]
async fn model_finances_usermac_get() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;

    let id = uuid!("00000000-0000-0000-0000-000000000000");

    // Action
    let user = UserMac::get(&db, id).await?;

    assert_eq!(id, user.id);
    assert_eq!("justinleahy", user.username);
    assert_eq!("Justin", user.first_name);
    assert_eq!("Leahy", user.last_name);

    Ok(())
}

#[tokio::test]
async fn model_finances_accountmac_list() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;

    // Action
    let accounts = AccountMac::list(&db).await?;

    // Check
    assert_eq!(1, accounts.len());

    Ok(())
}

#[tokio::test]
async fn model_finances_accountmac_get() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;

    let id = uuid!("00000000-0000-0000-0000-000000000001");

    // Action
    let account = AccountMac::get(&db, id).await?;

    // Check
    let user_id = uuid!("00000000-0000-0000-0000-000000000000");

    assert_eq!(id, account.id);
    assert_eq!(user_id, account.user_id);
    assert_eq!(AccountTypes::Checking, account.account_type);
    assert_eq!("Main Checking", account.nickname);
    assert_eq!(1, account.interest_integer);
    assert_eq!(0, account.interest_decimal);
    assert_eq!(-3, account.interest_exponent);
    assert_eq!(1, account.interest_frequency);
    assert_eq!(InterestFrequencyUnits::Day, account.interest_frequency_unit);

    Ok(())
}

#[tokio::test]
async fn model_finances_transactionmac_list() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;

    let transactions = TransactionMac::list(&db).await?;
    
    assert_eq!(1, transactions.len());

    Ok(())
}