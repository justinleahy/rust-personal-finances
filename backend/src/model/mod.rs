use thiserror::Error as ThisError;

mod db;
mod reference;
mod finances;
mod structures;

pub use db::Db;
pub use db::init_db;
pub use structures::Transaction;
pub use structures::TransactionMac;
pub use structures::Account;
pub use structures::AccountMac;
pub use structures::User;
pub use structures::UserMac;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error)
}