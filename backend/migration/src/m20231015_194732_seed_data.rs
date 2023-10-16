use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::entity::*;
use crate::entity::*;
use crate::m20231015_194732_seed_data::sea_orm_active_enums::*;
use uuid::Uuid;
use chrono::NaiveDateTime;
use rust_decimal::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        users::ActiveModel {
            uuid: Set(Uuid::new_v4().to_owned()),
            user_name: Set("justinleahy".to_owned()),
            password: Set("password".to_owned()),
            first_name: Set("Justin".to_owned()),
            last_name: Set("Leahy".to_owned()),
            ..Default::default()
        }.insert(db).await?;

        accounts::ActiveModel {
            uuid: Set(Uuid::new_v4().to_owned()),
            user_id: Set(1.to_owned()),
            account_type: Set(AccountTypes::Checking.to_owned()),
            nickname: Set("Ally Checking".to_owned()),
            interest: Set(Decimal::new(9995, 7).to_owned()),
            interest_frequency: Set(1.to_owned()),
            interest_frequency_unit: Set(InterestFrequencyUnits::Day.to_owned()),
            ..Default::default()
        }.insert(db).await?;

        accounts::ActiveModel {
            uuid: Set(Uuid::new_v4().to_owned()),
            user_id: Set(1.to_owned()),
            account_type: Set(AccountTypes::Saving.to_owned()),
            nickname: Set("Ally Saving".to_owned()),
            interest: Set(Decimal::new(416240, 7).to_owned()),
            interest_frequency: Set(1.to_owned()),
            interest_frequency_unit: Set(InterestFrequencyUnits::Day.to_owned()),
            ..Default::default()
        }.insert(db).await?;

        transactions::ActiveModel {
            uuid: Set(Uuid::new_v4().to_owned()),
            account_id: Set(1.to_owned()),
            transaction_date: Set(NaiveDateTime::from_timestamp_opt(1697414400, 0).unwrap().to_owned()),
            transaction_type: Set(TransactionTypes::Expense.to_owned()),
            transaction_category: Set(TransactionCategories::Expense.to_owned()),
            amount: Set(Decimal::new(100, 0).to_owned()),
            title: Set("Expense".to_owned()),
            ..Default::default()
        }.insert(db).await?;

        transactions::ActiveModel {
            uuid: Set(Uuid::new_v4().to_owned()),
            account_id: Set(1.to_owned()),
            transaction_date: Set(NaiveDateTime::from_timestamp_opt(1697414400, 0).unwrap().to_owned()),
            transaction_type: Set(TransactionTypes::Transfer.to_owned()),
            transaction_category: Set(TransactionCategories::Transfer.to_owned()),
            amount: Set(Decimal::new(-12712, 2).to_owned()),
            title: Set("Transfer to Ally Savings".to_owned()),
            vendor: Set(Some("Ally".to_owned())),
            ..Default::default()
        }.insert(db).await?;

        transactions::ActiveModel {
            uuid: Set(Uuid::new_v4().to_owned()),
            account_id: Set(2.to_owned()),
            transaction_date: Set(NaiveDateTime::from_timestamp_opt(1697414400, 0).unwrap().to_owned()),
            transaction_type: Set(TransactionTypes::Transfer.to_owned()),
            transaction_category: Set(TransactionCategories::Transfer.to_owned()),
            amount: Set(Decimal::new(12712, 2).to_owned()),
            title: Set("Transfer from Ally Checking".to_owned()),
            vendor: Set(Some("Ally".to_owned())),
            ..Default::default()
        }.insert(db).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {

        Ok(())
    }
}
