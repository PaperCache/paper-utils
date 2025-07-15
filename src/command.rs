/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub struct CommandByte;

impl CommandByte {
	pub const PING: u8 = 0;
	pub const VERSION: u8 = 1;

	pub const AUTH: u8 = 2;

	pub const GET: u8 = 3;
	pub const SET: u8 = 4;
	pub const DEL: u8 = 5;

	pub const HAS: u8 = 6;
	pub const PEEK: u8 = 7;
	pub const TTL: u8 = 8;
	pub const SIZE: u8 = 9;

	pub const WIPE: u8 = 10;

	pub const RESIZE: u8 = 11;
	pub const POLICY: u8 = 12;

	pub const STATUS: u8 = 13;
}
