use std::{
	error::Error,
	fmt::{Display, Formatter},
};

pub use crate::error::PaperError;

#[derive(PartialEq, Debug)]
pub enum ErrorKind {
	Empty,
	InvalidStream,
	InvalidData,
}

#[derive(Debug)]
pub struct SheetError {
	kind: ErrorKind,
	message: String,
}

impl SheetError {
	pub fn new(kind: ErrorKind, message: &str) -> Self {
		SheetError {
			kind,
			message: message.to_owned(),
		}
	}

	pub fn kind(&self) -> &ErrorKind {
		&self.kind
	}
}

impl PaperError for SheetError {
	fn message(&self) -> &str {
		&self.message
	}
}

impl Error for SheetError {}

impl Display for SheetError {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}
