pub struct PolicyByte;

impl PolicyByte {
	pub const LFU: u8 = 0;
	pub const FIFO: u8 = 1;
	pub const LRU: u8 = 2;
	pub const MRU: u8 = 3;
}
