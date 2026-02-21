/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub mod builder;

use std::io::Write;

#[cfg(feature = "tokio")]
use tokio::io::AsyncWrite;

#[cfg(feature = "tokio")]
use crate::stream::write_buf_async;
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

	pub fn write(&self, writer: &mut impl Write) -> Result<(), StreamError> {
		match write_buf(writer, &self.data) {
			Ok(_) => Ok(()),
			Err(_) => Err(StreamError::InvalidStream),
		}
	}

	#[cfg(feature = "tokio")]
	pub async fn write_async<W>(&self, writer: &mut W) -> Result<(), StreamError>
	where
		W: AsyncWrite + Unpin,
	{
		match write_buf_async(writer, &self.data).await {
			Ok(_) => Ok(()),
			Err(_) => Err(StreamError::InvalidStream),
		}
	}
}

pub use crate::sheet::builder::*;
