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

#[derive(sqlb::Fields, Clone)]
pub struct UserPatch {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>
}

// endregion: Users type

// endregion: Finance types

// region: Finance MACs

// region: User MAC

pub struct UserMac;

impl UserMac {
    pub async fn list(db: &Db) -> Result<Vec<User>, model::Error> {
        let sb = sqlb::select()
            .table(USER_MAC_TABLE)
            .columns(USER_MAC_COLUMNS);

        let users = sb.fetch_all(db).await?;
        
        Ok(users)
    }

    pub async fn get(db: &Db, id: Uuid) -> Result<User, model::Error> {
        let sb = sqlb::select()
            .table(USER_MAC_TABLE)
            .columns(USER_MAC_COLUMNS)
            .and_where_eq("id", id);

        let user = sb.fetch_one(db).await?;

        Ok(user)
    }
}

// endregion: User MAC

// endregion: Finance MACs

#[cfg(test)]
#[path = "../_tests/model_finances.rs"]
mod tests;