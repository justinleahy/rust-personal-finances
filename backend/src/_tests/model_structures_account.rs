use uuid::uuid;
use crate::model::db::init_db;
use crate::model::structures::user::UserMac;
use crate::model::structures::account::{AccountTypes, InterestFrequencyUnits, AccountPatch, AccountMac};

#[tokio::test]
async fn accountmac_create() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;

    let user_id = uuid!("00000000-0000-0000-0000-000000000000");
    let user = UserMac::get(&db, user_id).await?;

    let account_fx = AccountPatch {
        user_id: Some(user.id),
        account_type: Some(AccountTypes::Savings),
        nickname: Some("Main Savings".to_string()),
        interest: Some(0.041624),
        interest_frequency: Some(1),
        interest_frequency_unit: Some(InterestFrequencyUnits::Day)
    };

    // Action
    let account_created = AccountMac::create(&db, account_fx.clone()).await?;

    assert_eq!(user.id, account_created.user_id);
    assert_eq!(AccountTypes::Savings, account_created.account_type);
    assert_eq!("Main Savings", account_created.nickname);
    assert_eq!(0.041624, account_created.interest);
    assert_eq!(1, account_created.interest_frequency);
    assert_eq!(InterestFrequencyUnits::Day, account_created.interest_frequency_unit);

    Ok(())
}

#[tokio::test]
async fn accountmac_list() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;

    // Action
    let accounts = AccountMac::list(&db).await?;

    // Check
    assert_eq!(1, accounts.len());

    Ok(())
}

#[tokio::test]
async fn accountmac_get() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;

    let id = uuid!("00000000-0000-0000-0000-000000000001");

    // Action
    let account = AccountMac::get(&db, id).await?;
    let user = UserMac::get(&db, account.user_id).await?;

    // Check

    assert_eq!(id, account.id);
    assert_eq!(user.id, account.user_id);
    assert_eq!(AccountTypes::Checking, account.account_type);
    assert_eq!("Main Checking", account.nickname);
    assert_eq!(0.0009995, account.interest);
    assert_eq!(1, account.interest_frequency);
    assert_eq!(InterestFrequencyUnits::Day, account.interest_frequency_unit);

    Ok(())
}

#[tokio::test]
async fn accountmac_update() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;
    let id = uuid!("00000000-0000-0000-0000-000000000001");
    let account_data_fx: AccountPatch = AccountPatch {
        user_id: None,
        account_type: None,
        nickname: Some("Renamed Checking".to_string()),
        interest: None,
        interest_frequency: None,
        interest_frequency_unit: None
    };

    // Action
    let account_original = AccountMac::get(&db, id).await?;
    let account_updated = AccountMac::update(&db, id, account_data_fx).await?;

    // Check
    assert_eq!(account_original.id, account_updated.id);
    assert_eq!(account_original.user_id, account_updated.user_id);
    assert_eq!(account_original.account_type, account_updated.account_type);
    assert_eq!("Renamed Checking", account_updated.nickname);
    assert_eq!(account_original.interest, account_updated.interest);
    assert_eq!(account_original.interest_frequency, account_updated.interest_frequency);
    assert_eq!(account_original.interest_frequency_unit, account_updated.interest_frequency_unit);

    Ok(())
}