use uuid::{uuid, Uuid};
use serde_derive::{ Serialize, Deserialize };
use sqlb::{Fields, HasFields, SqlBuilder};
use super::super::db::Db;
use crate::model;

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub user_context: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Fields, Clone, Serialize, Deserialize)]
pub struct UserPatch {
    pub username: Option<String>,
    pub password_hash: Option<String>,
    pub user_context: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>
}

pub struct UserMac;

impl UserMac {
    const TABLE: &'static str = "users";
    const COLUMNS: &'static [&'static str] = &["id", "username", "password_hash", "user_context", "first_name", "last_name"];
}

impl UserMac {
    pub async fn create(db: &Db, data: UserPatch) -> Result<User, model::Error> {
        let mut fields = data.not_none_fields();
        fields.push(("id", Uuid::new_v4()).into());

        let sb = sqlb::insert()
            .table(Self::TABLE)
            .data(fields)
            .returning(Self::COLUMNS);

        let user = sb.fetch_one(db).await?;

        Ok(user)
    }

    pub async fn delete(db: &Db, id: Uuid) -> Result<User, model::Error> {
        let sb = sqlb::delete()
            .table(Self::TABLE)
            .returning(Self::COLUMNS)
            .and_where_eq("id", id);

        let user = sb.fetch_one(db).await?;

        Ok(user)
    }

    pub async fn list(db: &Db) -> Result<Vec<User>, model::Error> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .order_by("!id");

        let users = sb.fetch_all(db).await?;
        
        Ok(users)
    }

    pub async fn get(db: &Db, id: Uuid) -> Result<User, model::Error> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .and_where_eq("id", id);

        let user = sb.fetch_one(db).await?;

        Ok(user)
    }

    pub async fn update(db: &Db, id: Uuid, data: UserPatch) -> Result<User, model::Error> {
        let sb = sqlb::update()
            .table(Self::TABLE)
            .data(data.not_none_fields())
            .and_where_eq("id", id)
            .returning(Self::COLUMNS);

        let user = sb.fetch_one(db).await?;

        Ok(user)
    }
}

#[cfg(test)]
#[path = "../../_tests/model_structures_user.rs"]
mod tests;