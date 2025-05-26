/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum StreamError {
	#[error("Could not write to stream.")]
	InvalidStream,

	#[error("Stream closed unexpectedly.")]
	ClosedStream,

	#[error("Could not read data from stream.")]
	InvalidData,
}
