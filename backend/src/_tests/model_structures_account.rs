use std::ptr::null;
use uuid::uuid;
use crate::model::db::init_db;
use crate::model::structures::account::{AccountTypes, InterestFrequencyUnits, AccountPatch, AccountMac};

#[tokio::test]
async fn model_finances_accountmac_create() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;

    let user_id = uuid!("00000000-0000-0000-0000-000000000000");

    let account_fx = AccountPatch {
        user_id: Some(user_id),
        account_type: Some(AccountTypes::Savings),
        nickname: Some("Main Savings".to_string()),
        interest_integer: Some(4),
        interest_decimal: Some(16240),
        interest_exponent: Some(-2),
        interest_frequency: Some(1),
        interest_frequency_unit: Some(InterestFrequencyUnits::Day)
    };

    // Action
    let account_created = AccountMac::create(&db, account_fx.clone()).await?;

    assert_eq!(user_id, account_created.user_id);
    assert_eq!(AccountTypes::Savings, account_created.account_type);
    assert_eq!("Main Savings", account_created.nickname);
    assert_eq!(4, account_created.interest_integer);
    assert_eq!(16240, account_created.interest_decimal);
    assert_eq!(-2, account_created.interest_exponent);
    assert_eq!(1, account_created.interest_frequency);
    assert_eq!(InterestFrequencyUnits::Day, account_created.interest_frequency_unit);

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
