use std::io::Cursor;
use tokio::net::TcpStream;
use byteorder::{LittleEndian, ReadBytesExt};
use crate::stream::{Buffer, StreamError, ErrorKind, read_buf};

pub const TRUE_INDICATOR: u8 = 33;
pub const FALSE_INDICATOR: u8 = 63;

pub struct StreamReader<'a> {
	stream: &'a TcpStream,
}

impl<'a> StreamReader<'a> {
	pub fn new(stream: &'a TcpStream) -> Self {
		StreamReader {
			stream,
		}
	}

	pub async fn read_bool(&self) -> Result<bool, StreamError> {
		let buf = read_buf(self.stream, &1).await?;

		match buf[0] {
			TRUE_INDICATOR => Ok(true),
			FALSE_INDICATOR => Ok(false),

			_ => Err(StreamError::new(
				ErrorKind::InvalidData,
				"Could not read data from stream."
			)),
		}
	}

	pub async fn read_u8(&self) -> Result<u8, StreamError> {
		let buf = read_buf(self.stream, &1).await?;
		Ok(buf[0])
	}

	pub async fn read_u16(&self) -> Result<u16, StreamError> {
		let buf = read_buf(self.stream, &2).await?;
		let mut rdr = Cursor::new(buf);

		match rdr.read_u16::<LittleEndian>() {
			Ok(data) => Ok(data),

			Err(_) => Err(StreamError::new(
				ErrorKind::InvalidData,
				"Could not read data from stream."
			)),
		}
	}

	pub async fn read_u32(&self) -> Result<u32, StreamError> {
		let buf = read_buf(self.stream, &4).await?;
		let mut rdr = Cursor::new(buf);

		match rdr.read_u32::<LittleEndian>() {
			Ok(data) => Ok(data),

			Err(_) => Err(StreamError::new(
				ErrorKind::InvalidData,
				"Could not read data from stream."
			)),
		}
	}

	pub async fn read_u64(&self) -> Result<u64, StreamError> {
		let buf = read_buf(self.stream, &8).await?;
		let mut rdr = Cursor::new(buf);

		match rdr.read_u64::<LittleEndian>() {
			Ok(data) => Ok(data),

			Err(_) => Err(StreamError::new(
				ErrorKind::InvalidData,
				"Could not read data from stream."
			)),
		}
	}

	pub async fn read_f32(&self) -> Result<f32, StreamError> {
		let buf = read_buf(self.stream, &4).await?;
		let mut rdr = Cursor::new(buf);

		match rdr.read_f32::<LittleEndian>() {
			Ok(data) => Ok(data),

			Err(_) => Err(StreamError::new(
				ErrorKind::InvalidData,
				"Could not read data from stream."
			)),
		}
	}

	pub async fn read_f64(&self) -> Result<f64, StreamError> {
		let buf = read_buf(self.stream, &8).await?;
		let mut rdr = Cursor::new(buf);

		match rdr.read_f64::<LittleEndian>() {
			Ok(data) => Ok(data),

			Err(_) => Err(StreamError::new(
				ErrorKind::InvalidData,
				"Could not read data from stream."
			)),
		}
	}

	pub async fn read_buf(&self) -> Result<Buffer, StreamError> {
		let size = self.read_u32().await? as usize;
		read_buf(self.stream, &size).await
	}

	pub async fn read_string(&self) -> Result<String, StreamError> {
		let size = self.read_u32().await? as usize;
		let buf = read_buf(self.stream, &size).await?;

		match String::from_utf8(buf) {
			Ok(string) => Ok(string),

			Err(_) => Err(StreamError::new(
				ErrorKind::InvalidData,
				"Could not read data from stream."
			)),
		}
	}
}
