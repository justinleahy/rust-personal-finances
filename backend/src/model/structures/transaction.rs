use time::OffsetDateTime;
use uuid::{uuid, Uuid};
use serde_derive::{ Serialize, Deserialize };
use sqlb::{Fields, HasFields, SqlBuilder};
use super::super::db::Db;
use super::super::reference::{TRANSACTION_MAC_TABLE, TRANSACTION_MAC_COLUMNS};
use crate::model;

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "transaction_types")]
#[sqlx(rename_all = "lowercase")]
pub enum TransactionTypes {
    Deposit,
    Withdraw,
    Expense,
    Transfer
}
sqlb::bindable!(TransactionTypes);

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "transaction_categories")]
#[sqlx(rename_all = "lowercase")]
pub enum TransactionCategories {
    Income,
    Dividend,
    Transfer,
    Expense
}
sqlb::bindable!(TransactionCategories);

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub transaction_date: OffsetDateTime,
    pub transaction_type: TransactionTypes,
    pub category: TransactionCategories,
    pub transaction_integer: i32,
    pub transaction_decimal: i64,
    pub transaction_exponent: i32,
    pub comment: String
}

#[derive(sqlb::Fields, Clone)]
struct TransactionCreate {
    id: Uuid,
    account_id: Option<Uuid>,
    transaction_date: Option<OffsetDateTime>,
    transaction_type: Option<TransactionTypes>,
    category: Option<TransactionCategories>,
    transaction_integer: Option<i32>,
    transaction_decimal: Option<i64>,
    transaction_exponent: Option<i32>,
    comment: Option<String>
}

#[derive(sqlb::Fields, Clone)]
pub struct TransactionPatch {
    account_id: Option<Uuid>,
    transaction_date: Option<OffsetDateTime>,
    transaction_type: Option<TransactionTypes>,
    category: Option<TransactionCategories>,
    transaction_integer: Option<i32>,
    transaction_decimal: Option<i64>,
    transaction_exponent: Option<i32>,
    comment: Option<String>
}

pub struct TransactionMac;

impl TransactionMac {
    pub async fn create(db: &Db, data: TransactionPatch) -> Result<Transaction, model::Error> {
        let new_transaction = TransactionCreate {
            id: Uuid::new_v4(),
            account_id: data.account_id,
            transaction_date: data.transaction_date,
            transaction_type: data.transaction_type,
            category: data.category,
            transaction_integer: data.transaction_integer,
            transaction_decimal: data.transaction_decimal,
            transaction_exponent: data.transaction_exponent,
            comment: data.comment
        };

        let sb = sqlb::insert()
            .table(TRANSACTION_MAC_TABLE)
            .data(new_transaction.not_none_fields())
            .returning(TRANSACTION_MAC_COLUMNS);

        let transaction = sb.fetch_one(db).await?;

        Ok(transaction)
    }

    pub async fn list(db: &Db) -> Result<Vec<Transaction>, model::Error> {
        let sb = sqlb::select()
            .table(TRANSACTION_MAC_TABLE)
            .columns(TRANSACTION_MAC_COLUMNS)
            .order_by("!id");
    
        let transactions = sb.fetch_all(db).await?;

        Ok(transactions)
    }

    pub async fn get(db: &Db, id: Uuid) -> Result<Transaction, model::Error> {
        let sb = sqlb::select()
            .table(TRANSACTION_MAC_TABLE)
            .columns(TRANSACTION_MAC_COLUMNS)
            .and_where_eq("id", id);

        let transaction = sb.fetch_one(db).await?;

        Ok(transaction)
    }

    pub async fn update(db: &Db, id: Uuid, data: TransactionPatch) -> Result<Transaction, model::Error> {
        let sb = sqlb::update()
            .table(TRANSACTION_MAC_TABLE)
            .data(data.not_none_fields())
            .and_where_eq("id", id)
            .returning(TRANSACTION_MAC_COLUMNS);

        let transaction = sb.fetch_one(db).await?;

        Ok(transaction)
    }
}

#[cfg(test)]
#[path = "../../_tests/model_structures_transaction.rs"]
mod tests;