use sea_orm_migration::prelude::*;
use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::prelude::extension::postgres::Type;
use crate::m20231015_180325_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(AccountTypes::AccountTypes)
                    .values(AccountTypes::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(InterestFrequencyUnits::InterestFrequencyUnits)
                    .values(InterestFrequencyUnits::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Accounts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Accounts::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(ColumnDef::new(Accounts::Uuid).uuid().not_null())
                    .col(ColumnDef::new(Accounts::UserId).integer().not_null())
                    .col(ColumnDef::new(Accounts::AccountType)
                        .enumeration(AccountTypes::AccountTypes, AccountTypes::iter().skip(1)))
                    .col(ColumnDef::new(Accounts::Nickname).string().not_null())
                    .col(ColumnDef::new(Accounts::Interest).decimal().not_null())
                    .col(ColumnDef::new(Accounts::InterestFrequency).integer().not_null())
                    .col(ColumnDef::new(Accounts::InterestFrequencyUnit)
                        .enumeration(InterestFrequencyUnits::InterestFrequencyUnits, InterestFrequencyUnits::iter().skip(1)))
                    .to_owned()
            ).await?;

        manager
            .create_foreign_key(sea_query::ForeignKey::create()
                .name("account_user_fkey")
                .from(Accounts::Table, Accounts::UserId)
                .to(Users::Table, Users::Id)
                .to_owned()
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Accounts::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(AccountTypes::AccountTypes).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(InterestFrequencyUnits::InterestFrequencyUnits).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden, EnumIter)]
pub enum AccountTypes {
    AccountTypes,
    Checking,
    Saving
}

#[derive(Iden, EnumIter)]
pub enum InterestFrequencyUnits {
    InterestFrequencyUnits,
    Day,
    Week,
    Month,
    Year
}

#[derive(Iden)]
pub enum Accounts {
    Table,
    Id,
    Uuid,
    UserId,
    AccountType,
    Nickname,
    Interest,
    InterestFrequency,
    InterestFrequencyUnit
}