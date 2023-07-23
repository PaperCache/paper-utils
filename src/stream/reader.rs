use std::{
    io::Cursor,
    net::TcpStream,
};

use byteorder::{LittleEndian, ReadBytesExt};
use crate::stream::{Buffer, StreamError, ErrorKind, read_buf};

pub const TRUE_INDICATOR: u8 = 33;
pub const FALSE_INDICATOR: u8 = 63;

pub struct StreamReader<'a> {
	stream: &'a mut TcpStream,
}

impl<'a> StreamReader<'a> {
	pub fn new(stream: &'a mut TcpStream) -> Self {
		StreamReader {
			stream,
		}
	}

	pub fn read_bool(&mut self) -> Result<bool, StreamError> {
		let buf = read_buf(self.stream, 1)?;

		match buf[0] {
			TRUE_INDICATOR => Ok(true),
			FALSE_INDICATOR => Ok(false),

			_ => Err(StreamError::new(
				ErrorKind::InvalidData,
				"Could not read data from stream."
			)),
		}
	}

	pub fn read_u8(&mut self) -> Result<u8, StreamError> {
		let buf = read_buf(self.stream, 1)?;
		Ok(buf[0])
	}

	pub fn read_u16(&mut self) -> Result<u16, StreamError> {
		let buf = read_buf(self.stream, 2)?;
		let mut rdr = Cursor::new(buf);

		match rdr.read_u16::<LittleEndian>() {
			Ok(data) => Ok(data),

			Err(_) => Err(StreamError::new(
				ErrorKind::InvalidData,
				"Could not read data from stream."
			)),
		}
	}

	pub fn read_u32(&mut self) -> Result<u32, StreamError> {
		let buf = read_buf(self.stream, 4)?;
		let mut rdr = Cursor::new(buf);

		match rdr.read_u32::<LittleEndian>() {
			Ok(data) => Ok(data),

			Err(_) => Err(StreamError::new(
				ErrorKind::InvalidData,
				"Could not read data from stream."
			)),
		}
	}

	pub fn read_u64(&mut self) -> Result<u64, StreamError> {
		let buf = read_buf(self.stream, 8)?;
		let mut rdr = Cursor::new(buf);

		match rdr.read_u64::<LittleEndian>() {
			Ok(data) => Ok(data),

			Err(_) => Err(StreamError::new(
				ErrorKind::InvalidData,
				"Could not read data from stream."
			)),
		}
	}

	pub fn read_f32(&mut self) -> Result<f32, StreamError> {
		let buf = read_buf(self.stream, 4)?;
		let mut rdr = Cursor::new(buf);

		match rdr.read_f32::<LittleEndian>() {
			Ok(data) => Ok(data),

			Err(_) => Err(StreamError::new(
				ErrorKind::InvalidData,
				"Could not read data from stream."
			)),
		}
	}

	pub fn read_f64(&mut self) -> Result<f64, StreamError> {
		let buf = read_buf(self.stream, 8)?;
		let mut rdr = Cursor::new(buf);

		match rdr.read_f64::<LittleEndian>() {
			Ok(data) => Ok(data),

			Err(_) => Err(StreamError::new(
				ErrorKind::InvalidData,
				"Could not read data from stream."
			)),
		}
	}

	pub fn read_buf(&mut self) -> Result<Buffer, StreamError> {
		let size = self.read_u32()? as usize;
		read_buf(self.stream, size)
	}

	pub fn read_string(&mut self) -> Result<String, StreamError> {
		let size = self.read_u32()? as usize;
		let buf = read_buf(self.stream, size)?;

		match String::from_utf8(buf) {
			Ok(string) => Ok(string),

			Err(_) => Err(StreamError::new(
				ErrorKind::InvalidData,
				"Could not read data from stream."
			)),
		}
	}
}
