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
