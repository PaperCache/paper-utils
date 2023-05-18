pub mod error;
pub mod builder;

use std::io;
use tokio::net::TcpStream;
pub use crate::sheet::error::{SheetError, ErrorKind};

pub const OK_INDICATOR: u8 = 33;
pub const NOT_OK_INDICATOR: u8 = 63;

pub struct Sheet {
	is_ok: bool,
	size: u32,
	data: Vec<u8>,
}

impl Sheet {
	pub fn new(is_ok: bool, data: Vec<u8>) -> Self {
		Sheet {
			is_ok,
			size: data.len() as u32,
			data,
		}
	}

	pub fn serialize(&self) -> Vec<u8> {
		let mut buf = Vec::<u8>::new();
		let valid_byte = if self.is_ok { b"!" } else { b"?" };

		buf.extend_from_slice(valid_byte);
		buf.extend_from_slice(&self.size.to_le_bytes());
		buf.extend_from_slice(&self.data);

		buf
	}

	pub fn from_stream(stream: &TcpStream) -> Result<Self, SheetError> {
		let (is_ok, size) = read_headers(stream)?;

		println!("is_ok = {}, size = {}", is_ok, size);

		let sheet = Sheet {
			is_ok,
			size,
			data: read_data(stream, size as usize)?,
		};

		Ok(sheet)
	}


	pub fn is_ok(&self) -> bool {
		self.is_ok
	}

	pub fn size(&self) -> u32 {
		self.size
	}

	pub fn data(&self) -> &Vec<u8> {
		&self.data
	}

	pub fn to_string(&self) -> String {
		String::from_utf8_lossy(&self.data).to_string()
	}
}

fn read_headers(stream: &TcpStream) -> Result<(bool, u32), SheetError> {
	let mut buf = [0u8; 5];

	loop {
		match stream.try_read(&mut buf) {
			Ok(0) => {
				return Err(SheetError::new(
					ErrorKind::EmptyBuf,
					"Could not read response."
				));
			},

			Ok(size) => {
				if size != 5 {
					return Err(SheetError::new(
						ErrorKind::EmptyBuf,
						"Could not read response."
					));
				}

				let is_ok = match buf[0] {
					OK_INDICATOR => true,
					NOT_OK_INDICATOR => false,

					_ => {
						return Err(SheetError::new(
							ErrorKind::InvalidIndicator,
							"Parse error in response."
						));
					},
				};

				let size = u32::from_le_bytes([
					buf[1],
					buf[2],
					buf[3],
					buf[4],
				]);

				return Ok((is_ok, size));
			},

			Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
				continue;
			},

			Err(_) => {
				return Err(SheetError::new(
					ErrorKind::InvalidStream,
					"Could not read response."
				));
			},
		}
	}
}

fn read_data(stream: &TcpStream, buf_size: usize) -> Result<Vec<u8>, SheetError> {
	let mut data = Vec::<u8>::with_capacity(buf_size);
	let mut buf = [0u8; 4096];
	let mut read_bytes: usize = 0;

	loop {
		match stream.try_read(&mut buf) {
			Ok(0) => {
				if read_bytes == buf_size {
					return Ok(data);
				}

				return Err(SheetError::new(
					ErrorKind::EmptyBuf,
					"Could not read response."
				));
			},

			Ok(size) => {
				data.extend_from_slice(&buf[0..size]);
				read_bytes += size;

				if size == buf_size {
					return Ok(data);
				}

				continue;
			},

			Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
				continue;
			},

			Err(_) => {
				return Err(SheetError::new(
					ErrorKind::InvalidStream,
					"Could not read response."
				));
			},
		}
	}
}

pub use crate::sheet::builder::*;
