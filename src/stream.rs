/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub mod error;
pub mod reader;

use std::{
	io::{Read, Write},
	net::TcpStream,
};

pub type Buffer = Box<[u8]>;
pub type StackBuffer<const N: usize> = [u8; N];

pub fn read_buf(stream: &mut TcpStream, buf_size: usize) -> Result<Buffer, StreamError> {
	let mut buf = vec![0u8; buf_size].into_boxed_slice();

	match stream.read_exact(&mut buf) {
		Ok(_) => Ok(buf),
		Err(_) => Err(StreamError::ClosedStream),
	}
}

pub fn read_stack_buf<const N: usize>(stream: &mut TcpStream) -> Result<StackBuffer<N>, StreamError> {
	let mut buf = [0u8; N];

	match stream.read_exact(&mut buf) {
		Ok(_) => Ok(buf),
		Err(_) => Err(StreamError::ClosedStream),
	}
}

pub fn write_buf(stream: &mut TcpStream, buf: &[u8]) -> Result<(), StreamError> {
	match stream.write_all(buf) {
		Ok(_) => Ok(()),
		Err(_) => Err(StreamError::InvalidStream),
	}
}

pub use crate::stream::error::*;
pub use crate::stream::reader::*;
