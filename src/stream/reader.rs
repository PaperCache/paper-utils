/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};
#[cfg(feature = "tokio")]
use tokio::io::AsyncRead;

use crate::stream::{Buffer, StreamError, read_buf, read_stack_buf};
#[cfg(feature = "tokio")]
use crate::stream::{read_buf_async, read_stack_buf_async};

pub const TRUE_INDICATOR: u8 = 33;
pub const FALSE_INDICATOR: u8 = 63;

pub struct StreamReader<'a, R>
where
	R: Read,
{
	reader: &'a mut R,
}

#[cfg(feature = "tokio")]
pub struct AsyncStreamReader<'a, R>
where
	R: AsyncRead,
{
	reader: &'a mut R,
}

impl<'a, R> StreamReader<'a, R>
where
	R: Read,
{
	pub fn new(reader: &'a mut R) -> Self {
		StreamReader {
			reader,
		}
	}

	pub fn read_bool(&mut self) -> Result<bool, StreamError> {
		let buf = read_stack_buf::<1>(self.reader)?;

		match buf[0] {
			TRUE_INDICATOR => Ok(true),
			FALSE_INDICATOR => Ok(false),

			_ => Err(StreamError::InvalidData),
		}
	}

	pub fn read_u8(&mut self) -> Result<u8, StreamError> {
		let buf = read_stack_buf::<1>(self.reader)?;
		Ok(buf[0])
	}

	pub fn read_u16(&mut self) -> Result<u16, StreamError> {
		let buf = read_stack_buf::<2>(self.reader)?;
		let mut rdr = Cursor::new(buf);

		rdr.read_u16::<LittleEndian>()
			.map_err(|_| StreamError::InvalidData)
	}

	pub fn read_u32(&mut self) -> Result<u32, StreamError> {
		let buf = read_stack_buf::<4>(self.reader)?;
		let mut rdr = Cursor::new(buf);

		rdr.read_u32::<LittleEndian>()
			.map_err(|_| StreamError::InvalidData)
	}

	pub fn read_u64(&mut self) -> Result<u64, StreamError> {
		let buf = read_stack_buf::<8>(self.reader)?;
		let mut rdr = Cursor::new(buf);

		rdr.read_u64::<LittleEndian>()
			.map_err(|_| StreamError::InvalidData)
	}

	pub fn read_f32(&mut self) -> Result<f32, StreamError> {
		let buf = read_stack_buf::<4>(self.reader)?;
		let mut rdr = Cursor::new(buf);

		rdr.read_f32::<LittleEndian>()
			.map_err(|_| StreamError::InvalidData)
	}

	pub fn read_f64(&mut self) -> Result<f64, StreamError> {
		let buf = read_stack_buf::<8>(self.reader)?;
		let mut rdr = Cursor::new(buf);

		rdr.read_f64::<LittleEndian>()
			.map_err(|_| StreamError::InvalidData)
	}

	pub fn read_buf(&mut self) -> Result<Buffer, StreamError> {
		let size = self.read_u32()? as usize;
		read_buf(self.reader, size)
	}

	pub fn read_string(&mut self) -> Result<String, StreamError> {
		let size = self.read_u32()? as usize;
		let buf = read_buf(self.reader, size)?;

		String::from_utf8(buf.to_vec()).map_err(|_| StreamError::InvalidData)
	}
}

#[cfg(feature = "tokio")]
impl<'a, R> AsyncStreamReader<'a, R>
where
	R: AsyncRead + Unpin,
{
	pub fn new(reader: &'a mut R) -> Self {
		AsyncStreamReader {
			reader,
		}
	}

	pub async fn read_bool(&mut self) -> Result<bool, StreamError> {
		let buf = read_stack_buf_async::<R, 1>(self.reader).await?;

		match buf[0] {
			TRUE_INDICATOR => Ok(true),
			FALSE_INDICATOR => Ok(false),

			_ => Err(StreamError::InvalidData),
		}
	}

	pub async fn read_u8(&mut self) -> Result<u8, StreamError> {
		let buf = read_stack_buf_async::<R, 1>(self.reader).await?;
		Ok(buf[0])
	}

	pub async fn read_u16(&mut self) -> Result<u16, StreamError> {
		let buf = read_stack_buf_async::<R, 2>(self.reader).await?;
		let mut rdr = Cursor::new(buf);

		rdr.read_u16::<LittleEndian>()
			.map_err(|_| StreamError::InvalidData)
	}

	pub async fn read_u32(&mut self) -> Result<u32, StreamError> {
		let buf = read_stack_buf_async::<R, 4>(self.reader).await?;
		let mut rdr = Cursor::new(buf);

		rdr.read_u32::<LittleEndian>()
			.map_err(|_| StreamError::InvalidData)
	}

	pub async fn read_u64(&mut self) -> Result<u64, StreamError> {
		let buf = read_stack_buf_async::<R, 8>(self.reader).await?;
		let mut rdr = Cursor::new(buf);

		rdr.read_u64::<LittleEndian>()
			.map_err(|_| StreamError::InvalidData)
	}

	pub async fn read_f32(&mut self) -> Result<f32, StreamError> {
		let buf = read_stack_buf_async::<R, 4>(self.reader).await?;
		let mut rdr = Cursor::new(buf);

		rdr.read_f32::<LittleEndian>()
			.map_err(|_| StreamError::InvalidData)
	}

	pub async fn read_f64(&mut self) -> Result<f64, StreamError> {
		let buf = read_stack_buf_async::<R, 8>(self.reader).await?;
		let mut rdr = Cursor::new(buf);

		rdr.read_f64::<LittleEndian>()
			.map_err(|_| StreamError::InvalidData)
	}

	pub async fn read_buf(&mut self) -> Result<Buffer, StreamError> {
		let size = self.read_u32().await? as usize;
		read_buf_async(self.reader, size).await
	}

	pub async fn read_string(&mut self) -> Result<String, StreamError> {
		let size = self.read_u32().await? as usize;
		let buf = read_buf_async(self.reader, size).await?;

		String::from_utf8(buf.to_vec()).map_err(|_| StreamError::InvalidData)
	}
}
