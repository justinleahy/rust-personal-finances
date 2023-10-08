use uuid::{uuid, Uuid};
use sqlb::{Fields, HasFields, SqlBuilder};
use serde_derive::{ Serialize, Deserialize };
use super::super::db::Db;
use super::super::reference::{ACCOUNT_MAC_TABLE, ACCOUNT_MAC_COLUMNS};
use crate::model;

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "account_types")]
#[sqlx(rename_all = "lowercase")]
pub enum AccountTypes {
    Checking,
    Savings
}
sqlb::bindable!(AccountTypes);

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "interest_frequency_units")]
#[sqlx(rename_all = "lowercase")]
pub enum InterestFrequencyUnits {
    Day,
    Week,
    Month,
    Year
}
sqlb::bindable!(InterestFrequencyUnits);

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub user_id: Uuid,
    pub account_type: AccountTypes,
    pub nickname: String,
    pub interest: f64,
    pub interest_frequency: i32,
    pub interest_frequency_unit: InterestFrequencyUnits
}

#[derive(sqlb::Fields, Clone)]
pub struct AccountPatch {
    pub user_id: Option<Uuid>,
    pub account_type: Option<AccountTypes>,
    pub nickname: Option<String>,
    pub interest: Option<f64>,
    pub interest_frequency: Option<i32>,
    pub interest_frequency_unit: Option<InterestFrequencyUnits>
}

pub struct AccountMac;

impl AccountMac {
    pub async fn create(db: &Db, data: AccountPatch) -> Result<Account, model::Error> {
        let mut fields = data.not_none_fields();
        fields.push(("id", Uuid::new_v4()).into());

        let sb = sqlb::insert()
            .table(ACCOUNT_MAC_TABLE)
            .data(fields)
            .returning(ACCOUNT_MAC_COLUMNS);

        let account = sb.fetch_one(db).await?;
        
        Ok(account)
    }

    pub async fn list(db: &Db) -> Result<Vec<Account>, model::Error> {
        let sb = sqlb::select()
            .table(ACCOUNT_MAC_TABLE)
            .columns(ACCOUNT_MAC_COLUMNS)
            .order_by("!id");
        
        let accounts = sb.fetch_all(db).await?;

        Ok(accounts)
    }

    pub async fn get(db: &Db, id: Uuid) -> Result<Account, model::Error> {
        let sb = sqlb::select()
            .table(ACCOUNT_MAC_TABLE)
            .columns(ACCOUNT_MAC_COLUMNS)
            .and_where_eq("id", id);

        let account = sb.fetch_one(db).await?;

        Ok(account)
    }

    pub async fn update(db: &Db, id: Uuid, data: AccountPatch) -> Result<Account, model::Error> {
        let sb = sqlb::update()
            .table(ACCOUNT_MAC_TABLE)
            .data(data.not_none_fields())
            .and_where_eq("id", id)
            .returning(ACCOUNT_MAC_COLUMNS);

        let user = sb.fetch_one(db).await?;
        
        Ok(user)
    }
}

#[cfg(test)]
#[path = "../../_tests/model_structures_account.rs"]
mod tests;