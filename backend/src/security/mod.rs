use thiserror::Error as ThisError;

pub struct UserContext {
	pub user_id: String
}

pub async fn user_context_from_token(token: &str) -> Result<UserContext, Error> {
	// TODO: Real validation needed
	match token.parse::<String>() {
		Ok(user_id) => Ok(UserContext { user_id }),
		Err(_) => Err(Error::InvalidToken((token.to_string())))
	}

}

#[derive(ThisError, Debug)]
pub enum Error {
	#[error("Invalid Token {0}")]
	InvalidToken(String),
}