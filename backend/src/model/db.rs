use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use super::reference::{PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PASSWORD, PG_APP_DB, PG_APP_USER, PG_APP_PASSWORD, PG_APP_MAX_CONNECTIONS, SQL_DIR, SQL_RECREATE};

pub type Db = Pool<Postgres>;

async fn new_db_pool(host: &str, db: &str, user: &str, password: &str, max_connections: u32) -> Result<Db, sqlx::Error> {
    let con_string = format!("postgres://{}:{}@{}/{}", user, password, host, db);
    PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_millis(500))
        .connect(&con_string)
        .await
}
