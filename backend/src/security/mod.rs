use thiserror::Error as ThisError;

pub struct UserContext {
	pub user_id: String
}

#[derive(ThisError, Debug)]
pub enum Error {
	#[error("Invalid Token {0}")]
	InvalidToken(String),
}