use poem_openapi::{Enum, Object, OpenApi};
use sea_query::Iden;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::Db;
#[derive(Debug, Enum, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum AccountTypes {
    Checking,
    Savings
}

#[derive(Debug, Enum, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum InterestFrequencyUnits {
    Day,
    Week,
    Month,
    Year
}

#[derive(Debug, Object, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub account_type: AccountTypes,
    pub nickname: String,
    pub interest: f64,
    pub interest_frequency: i32,
    pub interest_frequency_unit: InterestFrequencyUnits
}

pub struct AccountMac;

impl AccountMac {
    const TABLE: &'static str = "accounts";
    const COLUMNS: &'static [&'static str] = &["id", "user_id", "account_type", "nickname", "interest", "interest_frequency", "interest_frequency_unit"];
}

impl AccountMac {
    pub async fn get(db: &Db) -> () {

    }
}