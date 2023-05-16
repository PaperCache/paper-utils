use std::error::Error;

pub trait PaperError: Error {
	fn message(&self) -> &str;
}
