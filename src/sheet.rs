pub mod error;
pub mod builder;

use tokio::net::TcpStream;
use crate::stream::{Buffer, StreamError, ErrorKind as StreamErrorKind, write_buf};

pub struct Sheet {
	data: Buffer,
}

impl Sheet {
	pub fn new(data: Buffer) -> Self {
		Sheet {
			data,
		}
	}

	pub fn serialize(&self) -> &[u8] {
		self.data.as_slice()
	}

	pub fn data(&self) -> &Buffer {
		&self.data
	}

	pub async fn to_stream(&self, stream: &TcpStream) -> Result<(), StreamError> {
		match write_buf(stream, &self.data).await {
			Ok(_) => Ok(()),

			Err(_) => Err(StreamError::new(
				StreamErrorKind::InvalidStream,
				"Could not write sheet to stream."
			))
		}
	}
}

pub use crate::sheet::error::*;
pub use crate::sheet::builder::*;
