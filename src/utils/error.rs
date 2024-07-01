//! Main Crate Error
#![allow(unused)] // For beginning only.

#[derive(thiserror::Error, Debug)]
pub enum MyError {
	/// For starter, to remove as code matures.
	#[error("Generic error: {0}")]
	Generic(String),
	/// For starter, to remove as code matures.
	#[error("Static error: {0}")]
	Static(&'static str),

	#[error(transparent)]
	IO(#[from] std::io::Error),
}

pub type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;