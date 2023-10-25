pub struct CommandByte;

impl CommandByte {
	pub const PING: u8 = 0;
	pub const VERSION: u8 = 1;

	pub const GET: u8 = 2;
	pub const SET: u8 = 3;
	pub const DEL: u8 = 4;

	pub const HAS: u8 = 5;
	pub const PEEK: u8 = 6;

	pub const WIPE: u8 = 7;

	pub const RESIZE: u8 = 8;
	pub const POLICY: u8 = 9;

	pub const STATS: u8 = 10;
}
