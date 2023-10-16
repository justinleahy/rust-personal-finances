use std::fs;
use std::path::PathBuf;
use sqlx::{PgPool, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use sea_query::{ColumnDef, PostgresQueryBuilder, Table};

// Root DB
pub const PG_HOST: &str = "localhost";
pub const PG_ROOT_DB: &str = "postgres";
pub const PG_ROOT_USER: &str = "postgres";
pub const PG_ROOT_PASSWORD: &str = "postgres";

// App DB
pub const PG_APP_DB: &str = "finance_db";
pub const PG_APP_USER: &str = "finance_user";
pub const PG_APP_PASSWORD: &str = "finance_pwd";
pub const PG_APP_MAX_CONNECTIONS: u32 = 5;

// SQL Files
const SQL_DIR: &str = "sql/";
const SQL_RECREATE: &str = "sql/00-recreate-db.sql";
const SQL_DEV_SEED: &str = "sql/02-dev-seed.sql";

pub type Db = Pool<Postgres>;

pub async fn init_db() -> Result<Db, sqlx::Error> {
    new_db_pool(PG_HOST, PG_APP_DB, PG_APP_USER, PG_APP_PASSWORD, PG_APP_MAX_CONNECTIONS).await
}

async fn new_db_pool(host: &str, db: &str, user: &str, password: &str, max_connections: u32) -> Result<Db, sqlx::Error> {
    let con_string = format!("postgres://{}:{}@{}/{}", user, password, host, db);
    PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_millis(500))
        .connect(&con_string)
        .await
}

async fn exec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    // Read the file
    let content = fs::read_to_string(file).map_err(|ex| {
        println!("Error reading {} (cause: {:?}", file, ex);
        ex
    })?;

    // TODO: Make split more SQL proof
    let sqls: Vec<&str> = content.split(";").collect();

    for sql in sqls {
        match sqlx::query(&sql).execute(db).await {
            Ok(_) => (),
            Err(ex) => println!("WARNING - pexec - Sql file '{}' FAILED cause: {}", file, ex),
        }
    }

    Ok(())
}