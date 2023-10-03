use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use super::reference::{PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PASSWORD, PG_APP_DB, PG_APP_USER, PG_APP_PASSWORD, PG_APP_MAX_CONNECTIONS, SQL_DIR, SQL_RECREATE};

pub type Db = Pool<Postgres>;