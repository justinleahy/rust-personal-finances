use std::ptr::null;
use uuid::uuid;
use time::Date;
use crate::model::db::init_db;
use crate::model::structures::user::{UserMac, UserPatch};

#[tokio::test]
async fn usermac_create() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;
    let user_fx = UserPatch {
        first_name: Some("John".to_string()),
        last_name: Some("Leahy".to_string()),
        username: Some("johnleahy".to_string())
    };
    
    // Action
    let user_created = UserMac::create(&db, user_fx.clone()).await?;

    // Check
    assert_eq!(user_created.username, "johnleahy");
    assert_eq!(user_created.first_name, "John");
    assert_eq!(user_created.last_name, "Leahy");

    Ok(())
}

#[tokio::test]
async fn usermac_list() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;
    
    // Action
    let users = UserMac::list(&db).await?;

    // Check
    assert_eq!(1, users.len());

    Ok(())
}

#[tokio::test]
async fn usermac_get() -> Result<(), Box<dyn std::error::Error>> {
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
async fn usermac_update() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let db = init_db().await?;
    let id = uuid!("00000000-0000-0000-0000-000000000000");
    let user_data_fx: UserPatch = UserPatch {
        username: Some("jjleahy".to_string()),
        first_name: None,
        last_name: None
    };

    // Action
    let user_original = UserMac::get(&db, id).await?;
    let user_updated = UserMac::update(&db, id, user_data_fx).await?;

    // Check
    assert_eq!("jjleahy", user_updated.username);
    assert_eq!(user_original.first_name, user_updated.first_name);
    assert_eq!(user_original.last_name, user_updated.last_name);

    Ok(())
}