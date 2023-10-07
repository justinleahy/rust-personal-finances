use time::Date;
use uuid::{uuid, Uuid};
use sqlb::{Fields, HasFields, SqlBuilder};
use super::{db::Db, reference::{USER_MAC_TABLE, USER_MAC_COLUMNS, ACCOUNT_MAC_TABLE, ACCOUNT_MAC_COLUMNS, TRANSACTION_MAC_TABLE, TRANSACTION_MAC_COLUMNS}};
use crate::model;

// region: Finance types

// region: Users type

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(sqlb::Fields, Clone)]
struct UserCreate {
    id: Option<Uuid>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>
}

#[derive(sqlb::Fields, Clone)]
pub struct UserPatch {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>
}

// endregion: Users type

// region: Account type

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "account_types")]
#[sqlx(rename_all = "lowercase")]
pub enum AccountTypes {
    Checking,
    Savings
}
sqlb::bindable!(AccountTypes);

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "interest_frequency_units")]
#[sqlx(rename_all = "lowercase")]
pub enum InterestFrequencyUnits {
    Day,
    Week,
    Month,
    Year
}
sqlb::bindable!(InterestFrequencyUnits);

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Account {
    pub id: Uuid,
    pub user_id: Uuid,
    pub account_type: AccountTypes,
    pub nickname: String,
    pub interest_integer: i32,
    pub interest_decimal: i64,
    pub interest_exponent: i32,
    pub interest_frequency: i32,
    pub interest_frequency_unit: InterestFrequencyUnits
}

#[derive(sqlb::Fields, Clone)]
struct AccountCreate {
    id: Option<Uuid>,
    user_id: Option<Uuid>,
    account_type: Option<AccountTypes>,
    nickname: Option<String>,
    interest_integer: Option<i32>,
    interest_decimal: Option<i64>,
    interest_exponent: Option<i32>,
    interest_frequency: Option<i32>,
    interest_frequency_unit: Option<InterestFrequencyUnits>
}

#[derive(sqlb::Fields, Clone)]
pub struct AccountPatch {
    pub user_id: Option<Uuid>,
    pub account_type: Option<AccountTypes>,
    pub nickname: Option<String>,
    pub interest_integer: Option<i32>,
    pub interest_decimal: Option<i64>,
    pub interest_exponent: Option<i32>,
    pub interest_frequency: Option<i32>,
    pub interest_frequency_unit: Option<InterestFrequencyUnits>
}

// endregion: Account type

// region: Transaction type

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
    Dividend
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



// endregion: Transaction type

// endregion: Finance types

// region: Finance MACs

// region: User MAC

pub struct UserMac;

impl UserMac {
    pub async fn create(db: &Db, data: UserPatch) -> Result<User, model::Error> {
        let new_user = UserCreate {
            id: Some(Uuid::new_v4()),
            first_name: data.first_name,
            last_name: data.last_name,
            username: data.username
        };

        let sb = sqlb::insert()
            .table(USER_MAC_TABLE)
            .data(new_user.not_none_fields())
            .returning(USER_MAC_COLUMNS);

        let user = sb.fetch_one(db).await?;

        Ok(user)
    }

    pub async fn list(db: &Db) -> Result<Vec<User>, model::Error> {
        let sb = sqlb::select()
            .table(USER_MAC_TABLE)
            .columns(USER_MAC_COLUMNS);

        let users = sb.fetch_all(db).await?;
        
        Ok(users)
    }

    pub async fn get(db: &Db, id: Uuid) -> Result<User, model::Error> {
        let sb = sqlb::select()
            .table(USER_MAC_TABLE)
            .columns(USER_MAC_COLUMNS)
            .and_where_eq("id", id);

        let user = sb.fetch_one(db).await?;

        Ok(user)
    }
}

// endregion: User MAC

// region: Account MAC

pub struct AccountMac;

impl AccountMac {
    pub async fn list(db: &Db) -> Result<Vec<Account>, model::Error> {
        let sb = sqlb::select()
            .table(ACCOUNT_MAC_TABLE)
            .columns(ACCOUNT_MAC_COLUMNS);
        
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
}

// endregion: Account MAC

// region: Transaction MAC

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

// endregion: Transaction MAC

// endregion: Finance MACs

#[cfg(test)]
#[path = "../_tests/model_finances.rs"]
mod tests;