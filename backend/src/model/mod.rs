use thiserror::Error as ThisError;

mod db;
mod reference;
mod finances;
mod structures;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error)
}
