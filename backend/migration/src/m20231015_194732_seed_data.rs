use sea_orm_migration::prelude::*;
use crate::m20231015_180325_create_users_table::Users;
use crate::m20231015_183517_create_accounts_table::Accounts;
use crate::m20231015_192911_create_transactions_table::Transactions;
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let user_uuid = Uuid::new_v4();
        let insert_user = Query::insert()
            .into_table(Users::Table)
            .columns([Users::Uuid, Users::UserName, Users::Password,
                Users::FirstName, Users::LastName])
            .values_panic([user_uuid.into(), "justinleahy".into(), "password".into(),
                "Justin".into(), "Leahy".into()])
            .to_owned();

        manager.exec_stmt(insert_user).await?;

        let account_uuid = Uuid::new_v4();
        let insert_account = Query::insert()
            .into_table(Accounts::Table)
            .columns([Accounts::Uuid, Accounts::UserId, Accounts::AccountType, Accounts::Nickname,
                Accounts::Interest, Accounts::InterestFrequency, Accounts::InterestFrequencyUnit])
            .values_panic([account_uuid.into(), 1.into(),
                Expr::val("checking").as_enum(Alias::new("account_types")), "Ally Checking".into(),
                0.0009995.into(), 1.into(),
                Expr::val("day").as_enum(Alias::new("interest_frequency_units"))])
            .to_owned();

        manager.exec_stmt(insert_account).await?;

        let account_uuid = Uuid::new_v4();
        let insert_account = Query::insert()
            .into_table(Accounts::Table)
            .columns([Accounts::Uuid, Accounts::UserId, Accounts::AccountType, Accounts::Nickname,
                Accounts::Interest, Accounts::InterestFrequency, Accounts::InterestFrequencyUnit])
            .values_panic([account_uuid.into(), 1.into(),
                Expr::val("saving").as_enum(Alias::new("account_types")), "Ally Savings".into(),
                0.0416240.into(), 1.into(),
                Expr::val("day").as_enum(Alias::new("interest_frequency_units"))])
            .to_owned();

        manager.exec_stmt(insert_account).await?;

        let transaction_uuid = Uuid::new_v4();
        let insert_transaction = Query::insert()
            .into_table(Transactions::Table)
            .columns([Transactions::Uuid, Transactions::AccountId, Transactions::TransactionDate,
                Transactions::TransactionType, Transactions::TransactionCategory,
                Transactions::Amount, Transactions::Title])
            .values_panic([transaction_uuid.into(), 1.into(), OffsetDateTime::now_utc().into(),
                Expr::val("expense").as_enum(Alias::new("transaction_types")),
                Expr::val("expense").as_enum(Alias::new("transaction_categories")),
                100.00.into(), "Expense".into()])
            .to_owned();

        manager.exec_stmt(insert_transaction).await?;

        let transaction_uuid = Uuid::new_v4();
        let insert_transaction = Query::insert()
            .into_table(Transactions::Table)
            .columns([Transactions::Uuid, Transactions::AccountId, Transactions::TransactionDate,
                Transactions::TransactionType, Transactions::TransactionCategory,
                Transactions::Amount, Transactions::Title, Transactions::Vendor])
            .values_panic([transaction_uuid.into(), 1.into(), OffsetDateTime::now_utc().into(),
                Expr::val("transfer").as_enum(Alias::new("transaction_types")),
                Expr::val("transfer").as_enum(Alias::new("transaction_categories")),
                (-127.12).into(), "Transfer to Ally Savings".into(), "Ally".into()])
            .to_owned();

        manager.exec_stmt(insert_transaction).await?;

        let transaction_uuid = Uuid::new_v4();
        let insert_transaction = Query::insert()
            .into_table(Transactions::Table)
            .columns([Transactions::Uuid, Transactions::AccountId, Transactions::TransactionDate,
                Transactions::TransactionType, Transactions::TransactionCategory,
                Transactions::Amount, Transactions::Title, Transactions::Vendor])
            .values_panic([transaction_uuid.into(), 2.into(), OffsetDateTime::now_utc().into(),
                Expr::val("transfer").as_enum(Alias::new("transaction_types")),
                Expr::val("transfer").as_enum(Alias::new("transaction_categories")),
                127.12.into(), "Transfer from Ally Checking".into(), "Ally".into()])
            .to_owned();

        manager.exec_stmt(insert_transaction).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {



        Ok(())
    }
}
