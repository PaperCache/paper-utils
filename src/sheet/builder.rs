use byteorder::{LittleEndian, WriteBytesExt};
use crate::sheet::Sheet;

pub struct SheetBuilder {
	data: Vec<u8>,
}

impl SheetBuilder {
	pub fn new() -> Self {
		SheetBuilder {
			data: Vec::new(),
		}
	}

	pub fn write_bool(mut self, value: &bool) -> Self {
		let byte = if *value { b"!" } else { b"?" };
		self.data.extend_from_slice(byte);
		self
	}

	pub fn write_u8(mut self, value: &u8) -> Self {
		self.data.push(*value);
		self
	}

	pub fn write_u16(mut self, value: &u16) -> Self {
		let mut wtr = vec![];
		wtr.write_u16::<LittleEndian>(*value).unwrap();

		self.data.extend(wtr);
		self
	}

	pub fn write_u32(mut self, value: &u32) -> Self {
		let mut wtr = vec![];
		wtr.write_u32::<LittleEndian>(*value).unwrap();

		self.data.extend(wtr);
		self
	}

	pub fn write_u64(mut self, value: &u64) -> Self {
		let mut wtr = vec![];
		wtr.write_u64::<LittleEndian>(*value).unwrap();

		self.data.extend(wtr);
		self
	}

	pub fn write_f32(mut self, value: &f32) -> Self {
		let mut wtr = vec![];
		wtr.write_f32::<LittleEndian>(*value).unwrap();

		self.data.extend(wtr);
		self
	}

	pub fn write_f64(mut self, value: &f64) -> Self {
		let mut wtr = vec![];
		wtr.write_f64::<LittleEndian>(*value).unwrap();

		self.data.extend(wtr);
		self
	}

	pub fn write_buf(mut self, value: &[u8]) -> Self {
		self = self.write_u32(&(value.len() as u32));
		self.data.extend(value);
		self
	}

	pub fn write_str(self, value: &str) -> Self {
		self.write_buf(value.as_bytes())
	}

	pub fn to_sheet(self) -> Sheet {
		Sheet::new(self.data)
	}
}

impl Default for SheetBuilder {
	fn default() -> Self {
		SheetBuilder::new()
	}
}
