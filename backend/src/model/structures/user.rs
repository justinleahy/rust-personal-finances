use time::Date;
use uuid::{uuid, Uuid};
use sqlb::{Fields, HasFields, SqlBuilder};
use super::super::db::Db;
use super::super::reference::{USER_MAC_TABLE, USER_MAC_COLUMNS};
use crate::model;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(sqlb::Fields, Clone)]
struct UserCreate {
    id: Option<Uuid>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>
}

#[derive(sqlb::Fields, Clone)]
pub struct UserPatch {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>
}

pub struct UserMac;

impl UserMac {
    pub async fn create(db: &Db, data: UserPatch) -> Result<User, model::Error> {
        let new_user = UserCreate {
            id: Some(Uuid::new_v4()),
            first_name: data.first_name,
            last_name: data.last_name,
            username: data.username
        };

        let sb = sqlb::insert()
            .table(USER_MAC_TABLE)
            .data(new_user.not_none_fields())
            .returning(USER_MAC_COLUMNS);

        let user = sb.fetch_one(db).await?;

        Ok(user)
    }

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

    pub async fn update(db: &Db, id: Uuid, data: UserPatch) -> Result<User, model::Error> {
        let sb = sqlb::update()
            .table(USER_MAC_TABLE)
            .data(data.not_none_fields())
            .and_where_eq("id", id)
            .returning(USER_MAC_COLUMNS);

        let user = sb.fetch_one(db).await?;

        Ok(user)
    }
}

#[cfg(test)]
#[path = "../../_tests/model_structures_user.rs"]
mod tests;