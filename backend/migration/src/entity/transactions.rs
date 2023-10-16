//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use super::sea_orm_active_enums::TransactionCategories;
use super::sea_orm_active_enums::TransactionTypes;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "transactions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub uuid: Uuid,
    pub account_id: i32,
    pub transaction_date: DateTime,
    pub transaction_type: TransactionTypes,
    pub transaction_category: TransactionCategories,
    pub amount: Decimal,
    pub title: String,
    pub vendor: Option<String>,
    pub comment: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
