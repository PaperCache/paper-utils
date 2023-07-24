use std::{
	error::Error,
	fmt::{Display, Formatter},
};

pub use crate::error::PaperError;

#[derive(PartialEq, Debug)]
pub enum ErrorKind {
	EmptyBuf,

	InvalidStream,
	InvalidData,
}

#[derive(Debug)]
pub struct StreamError {
	kind: ErrorKind,
	message: String,
}

impl StreamError {
	pub fn new(kind: ErrorKind, message: &str) -> Self {
		StreamError {
			kind,
			message: message.to_owned(),
		}
	}

	pub fn kind(&self) -> &ErrorKind {
		&self.kind
	}
}

impl PaperError for StreamError {
	fn message(&self) -> &str {
		&self.message
	}
}

impl Error for StreamError {}

impl Display for StreamError {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}
