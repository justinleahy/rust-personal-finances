use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use super::reference::{PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PASSWORD, PG_APP_DB, PG_APP_USER, PG_APP_PASSWORD, PG_APP_MAX_CONNECTIONS, SQL_DIR, SQL_RECREATE};

pub type Db = Pool<Postgres>;

// SQL Files
const SQL_DIR: &str = "sql/";
const SQL_RECREATE: &str = "sql/00-recreate-db.sql";

pub async fn init_db() -> Result<Db, sqlx::Error> {
    // Create the database with PG_ROOT (dev only)
    {
        let root_db = new_db_pool(PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PASSWORD, 1).await?;
        pexec(&root_db, SQL_RECREATE).await?;
    }

    // Run the app database SQL files
    let app_db = new_db_pool(PG_HOST, PG_APP_DB, PG_APP_USER, PG_APP_PASSWORD, 1).await?;
    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .into_iter()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    
    paths.sort();

    // Execute each file
    for path in paths {
        if let Some(path) = path.to_str() {
            if path.ends_with(".sql") && path != SQL_RECREATE {
                pexec(&app_db, &path).await?;
            }
        }
    }

    // Returning the database
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

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
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

#[cfg(test)]
#[path = "../_tests/model_db.rs"]
mod tests;