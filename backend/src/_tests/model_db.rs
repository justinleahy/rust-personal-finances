use super::init_db;

#[tokio::test]
async fn model_db_init_db() -> Result<(), Box<dyn std::error::Error>> {
    // Action
    let db = init_db().await?;

    let user_results = sqlx::query("SELECT * FROM user").fetch_all(&db).await?;
    assert_eq!(1, user_results.len(), "Number of seeded users");
    
    Ok(())
}