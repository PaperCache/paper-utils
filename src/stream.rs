pub mod error;
pub mod reader;

use std::{
	io::{Read, Write},
	net::TcpStream,
};

pub type Buffer = Vec<u8>;

pub fn read_buf(stream: &mut TcpStream, buf_size: usize) -> Result<Buffer, StreamError> {
	let mut buf = vec![0u8; buf_size];

	match stream.read_exact(&mut buf) {
		Ok(_) => Ok(buf),
		Err(_) => Err(StreamError::ClosedStream),
	}
}

pub fn write_buf(stream: &mut TcpStream, buf: &[u8]) -> Result<(), StreamError> {
	match stream.write(buf) {
		Ok(_) => Ok(()),
		Err(_) => Err(StreamError::InvalidStream),
	}
}

pub use crate::stream::error::*;
pub use crate::stream::reader::*;
