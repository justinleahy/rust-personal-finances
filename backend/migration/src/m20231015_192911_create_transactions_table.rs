use sea_orm_migration::prelude::*;
use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::prelude::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(TransactionTypes::TransactionTypes)
                    .values(TransactionTypes::iter().skip(1))
                    .to_owned()
            )
            .await?;
        manager
            .create_type(
                Type::create()
                    .as_enum(TransactionCategories::TransactionCategories)
                    .values(TransactionCategories::iter().skip(1))
                    .to_owned()
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Transactions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transactions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(ColumnDef::new(Transactions::Uuid).uuid().not_null())
                    .col(ColumnDef::new(Transactions::AccountId).integer().not_null())
                    .col(ColumnDef::new(Transactions::TransactionDate).date_time().not_null())
                    .col(ColumnDef::new(Transactions::TransactionType)
                        .enumeration(TransactionTypes::TransactionTypes, TransactionTypes::iter().skip(1)).not_null())
                    .col(ColumnDef::new(Transactions::TransactionCategory)
                        .enumeration(TransactionCategories::TransactionCategories, TransactionCategories::iter().skip(1)).not_null())
                    .col(ColumnDef::new(Transactions::Amount).decimal().not_null())
                    .col(ColumnDef::new(Transactions::Title).string().not_null())
                    .col(ColumnDef::new(Transactions::Vendor).string())
                    .col(ColumnDef::new(Transactions::Comment).string())
                    .to_owned()
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transactions::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(TransactionCategories::TransactionCategories).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(TransactionTypes::TransactionTypes).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden, EnumIter)]
pub enum TransactionTypes {
    TransactionTypes,
    Deposit,
    Withdraw,
    Expense,
    Transfer
}

#[derive(Iden, EnumIter)]
pub enum TransactionCategories {
    TransactionCategories,
    Income,
    Transfer,
    Expense,
    Interest,
    Dividend
}

#[derive(DeriveIden)]
pub enum Transactions {
    Table,
    Id,
    Uuid,
    AccountId,
    TransactionDate,
    TransactionType,
    TransactionCategory,
    Amount,
    Title,
    Vendor,
    Comment
}