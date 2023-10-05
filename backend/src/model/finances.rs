use uuid::{uuid, Uuid};
use sqlb::{Fields, HasFields, SqlBuilder};
use super::{db::Db, reference::{USER_MAC_TABLE, USER_MAC_COLUMNS, ACCOUNT_MAC_TABLE, ACCOUNT_MAC_COLUMNS}};
use crate::model;

// region: Finance types

// region: Users type

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

// endregion: Users type

// endregion: Finance types

// region: Finance MACs

// region: User MAC

pub struct UserMac;

// endregion: User MAC

// endregion: Finance MACs
