use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum StreamError {
	#[error("Could not write to stream.")]
	InvalidStream,

	#[error("Stream closed unexpectedly.")]
	ClosedStream,

	#[error("Could not read data from stream.")]
	InvalidData,
}
