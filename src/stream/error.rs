use thiserror::Error;

#[derive(Debug, Error)]
pub enum StreamError {
	#[error("Could not write to stream.")]
	InvalidStream,

	#[error("Stream closed unexpectedly.")]
	ClosedStream,

	#[error("Could not read data from stream.")]
	InvalidData,
}
