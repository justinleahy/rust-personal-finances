use std::ptr::null;
use uuid::uuid;
use crate::model::db::init_db;
use crate::model::finances::{UserMac, AccountMac, AccountTypes, InterestFrequencyUnits};

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
