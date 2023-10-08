use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
	#[error("Invalid Token {0}")]
	InvalidToken(String),
}