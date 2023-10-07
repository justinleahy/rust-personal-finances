use time::Date;
use uuid::{uuid, Uuid};
use sqlb::{Fields, HasFields, SqlBuilder};
use super::super::db::Db;
use super::super::reference::{TRANSACTION_MAC_TABLE, TRANSACTION_MAC_COLUMNS};
use crate::model;

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "transaction_types")]
#[sqlx(rename_all = "lowercase")]
pub enum TransactionTypes {
    Deposit,
    Withdraw,
    Expense,
    Transfer
}
sqlb::bindable!(TransactionTypes);

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "transaction_categories")]
#[sqlx(rename_all = "lowercase")]
pub enum TransactionCategories {
    Income,
    Dividend,
    Transfer,
    Expense
}
sqlb::bindable!(TransactionCategories);

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub transaction_date: Date,
    pub transaction_type: TransactionTypes,
    pub category: TransactionCategories,
    pub transaction_integer: i32,
    pub transaction_decimal: i64,
    pub transaction_exponent: i32,
    pub comment: String
}

pub struct TransactionMac;

impl TransactionMac {
    pub async fn list(db: &Db) -> Result<Vec<Transaction>, model::Error> {
        let sb = sqlb::select()
            .table(TRANSACTION_MAC_TABLE)
            .columns(TRANSACTION_MAC_COLUMNS);
    
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
}

#[cfg(test)]
#[path = "../../_tests/model_structures_transaction.rs"]
mod tests;