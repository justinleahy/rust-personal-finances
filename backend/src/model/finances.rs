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

impl UserMac {
    pub async fn list(db: &Db) -> Result<Vec<User>, model::Error> {
        let sb = sqlb::select()
            .table(USER_MAC_TABLE)
            .columns(USER_MAC_COLUMNS);

        let users = sb.fetch_all(db).await?;
        
        Ok(users)
    }
}
// endregion: User MAC

// endregion: Finance MACs
