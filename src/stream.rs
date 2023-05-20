pub mod error;
pub mod reader;

use std::io;
use tokio::net::TcpStream;

pub type Buffer = Vec<u8>;

pub async fn read_buf(stream: &TcpStream, buf_size: &usize) -> Result<Buffer, StreamError> {
	let mut buf = vec![0u8; *buf_size];
	let mut read_size: usize = 0;

	loop {
		if let Err(_) = stream.readable().await {
			return Err(StreamError::new(
				ErrorKind::Disconnected,
				"Disconnected from stream."
			));
		}

		match stream.try_read(&mut buf) {
			Ok(0) => {
				if read_size == *buf_size {
					return Ok(buf);
				}

				return Err(StreamError::new(
					ErrorKind::Disconnected,
					"Disconnected from stream."
				));
			},

			Ok(size) => {
				read_size += size;

				if read_size == *buf_size {
					return Ok(buf);
				}
			},

			Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
				continue;
			},

			Err(_) => {
				return Err(StreamError::new(
					ErrorKind::InvalidStream,
					"Stream closed unexpectedly."
				));
			},
		}
	}
}

pub async fn write_buf(stream: &TcpStream, buf: &[u8]) -> Result<(), StreamError> {
	loop {
		if let Err(_) = stream.writable().await {
			return Err(StreamError::new(
				ErrorKind::Disconnected,
				"Disconnected from stream."
			));
		}

		match stream.try_write(buf) {
			Ok(_) => {
				return Ok(());
			},

			Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
				continue;
			},

			Err(_) => {
				return Err(StreamError::new(
					ErrorKind::InvalidStream,
					"Could not write to stream."
				));
			},
		}
	}
}

pub use crate::stream::error::*;
pub use crate::stream::reader::*;
