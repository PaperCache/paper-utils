/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub mod error;
pub mod reader;

use std::io::{Read, Write};

#[cfg(feature = "tokio")]
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub type Buffer = Box<[u8]>;
pub type StackBuffer<const N: usize> = [u8; N];

pub fn read_buf(reader: &mut impl Read, buf_size: usize) -> Result<Buffer, StreamError> {
	let mut buf = vec![0u8; buf_size].into_boxed_slice();

	match reader.read_exact(&mut buf) {
		Ok(_) => Ok(buf),
		Err(_) => Err(StreamError::ClosedStream),
	}
}

#[cfg(feature = "tokio")]
pub async fn read_buf_async<R>(reader: &mut R, buf_size: usize) -> Result<Buffer, StreamError>
where
	R: AsyncRead + Unpin,
{
	let mut buf = vec![0u8; buf_size].into_boxed_slice();

	match reader.read_exact(&mut buf).await {
		Ok(_) => Ok(buf),
		Err(_) => Err(StreamError::ClosedStream),
	}
}

pub fn read_stack_buf<const N: usize>(
	reader: &mut impl Read,
) -> Result<StackBuffer<N>, StreamError> {
	let mut buf = [0u8; N];

	match reader.read_exact(&mut buf) {
		Ok(_) => Ok(buf),
		Err(_) => Err(StreamError::ClosedStream),
	}
}

#[cfg(feature = "tokio")]
pub async fn read_stack_buf_async<R, const N: usize>(
	reader: &mut R,
) -> Result<StackBuffer<N>, StreamError>
where
	R: AsyncRead + Unpin,
{
	let mut buf = [0u8; N];

	match reader.read_exact(&mut buf).await {
		Ok(_) => Ok(buf),
		Err(_) => Err(StreamError::ClosedStream),
	}
}

pub fn write_buf(writer: &mut impl Write, buf: &[u8]) -> Result<(), StreamError> {
	match writer.write_all(buf) {
		Ok(_) => Ok(()),
		Err(_) => Err(StreamError::InvalidStream),
	}
}

#[cfg(feature = "tokio")]
pub async fn write_buf_async<W>(writer: &mut W, buf: &[u8]) -> Result<(), StreamError>
where
	W: AsyncWrite + Unpin,
{
	writer
		.write_all(buf)
		.await
		.map_err(|_| StreamError::InvalidStream)?;

	writer
		.flush()
		.await
		.map_err(|_| StreamError::InvalidStream)?;

	Ok(())
}

pub use crate::stream::{error::*, reader::*};
