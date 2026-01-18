use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Invalid Data {0}: {0}")]
	InvalidData(String),
	
	#[error("Database error {0}:")]
	DatabaseError(#[from] sqlx::Error),
	
	#[error("Invalid argument {0}: {0}")]
	InvalidArgument(String)
}