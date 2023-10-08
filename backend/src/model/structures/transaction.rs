use time::OffsetDateTime;
use uuid::{uuid, Uuid};
use serde_derive::{ Serialize, Deserialize };
use sqlb::{Fields, HasFields, SqlBuilder};
use super::super::db::Db;
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
    pub amount: f64,
    pub title: String,
    // TODO: Move vendors to their own table and object
    pub vendor: Option<String>,
    pub comment: Option<String>
}

#[derive(sqlb::Fields, Clone)]
pub struct TransactionPatch {
    account_id: Option<Uuid>,
    transaction_date: Option<OffsetDateTime>,
    transaction_type: Option<TransactionTypes>,
    category: Option<TransactionCategories>,
    amount: Option<f64>,
    title: Option<String>,
    vendor: Option<String>,
    comment: Option<String>
}

pub struct TransactionMac;

impl TransactionMac {
    const TABLE:&'static str = "transactions";
    const COLUMNS: &'static [&'static str] = &["id", "account_id", "transaction_date", "transaction_type", "category", "amount", "title", "vendor", "comment"];
}

impl TransactionMac {
    pub async fn create(db: &Db, data: TransactionPatch) -> Result<Transaction, model::Error> {
        let mut fields = data.not_none_fields();
        fields.push(("id", Uuid::new_v4()).into());

        let sb = sqlb::insert()
            .table(Self::TABLE)
            .data(fields)
            .returning(Self::COLUMNS);

        let transaction = sb.fetch_one(db).await?;

        Ok(transaction)
    }

    pub async fn delete(db: &Db, id: Uuid) -> Result<Transaction, model::Error> {
        let sb = sqlb::delete()
            .table(Self::TABLE)
            .returning(Self::COLUMNS)
            .and_where_eq("id", id);

        let transaction = sb.fetch_one(db).await?;

        Ok(transaction)
    }

    pub async fn list(db: &Db) -> Result<Vec<Transaction>, model::Error> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .order_by("!id");
    
        let transactions = sb.fetch_all(db).await?;

        Ok(transactions)
    }

    pub async fn get(db: &Db, id: Uuid) -> Result<Transaction, model::Error> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .and_where_eq("id", id);

        let transaction = sb.fetch_one(db).await?;

        Ok(transaction)
    }

    pub async fn update(db: &Db, id: Uuid, data: TransactionPatch) -> Result<Transaction, model::Error> {
        let sb = sqlb::update()
            .table(Self::TABLE)
            .data(data.not_none_fields())
            .and_where_eq("id", id)
            .returning(Self::COLUMNS);

        let transaction = sb.fetch_one(db).await?;

        Ok(transaction)
    }
}

#[cfg(test)]
#[path = "../../_tests/model_structures_transaction.rs"]
mod tests;