pub mod builder;

use std::net::TcpStream;
use crate::stream::{Buffer, StreamError, write_buf};

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
		&self.data
	}

	pub fn data(&self) -> &Buffer {
		&self.data
	}

	pub fn write_to_stream(&self, stream: &mut TcpStream) -> Result<(), StreamError> {
		match write_buf(stream, &self.data) {
			Ok(_) => Ok(()),
			Err(_) => Err(StreamError::InvalidStream),
		}
	}
}

pub use crate::sheet::builder::*;
