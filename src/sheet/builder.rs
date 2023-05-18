use byteorder::{LittleEndian, WriteBytesExt};
use crate::sheet::Sheet;

pub struct SheetBuilder {
	is_ok: bool,
	data: Vec<u8>,
}

impl SheetBuilder {
	pub fn new(is_ok: bool) -> Self {
		SheetBuilder {
			is_ok,
			data: Vec::new(),
		}
	}

	pub fn add_u8(mut self, value: &u8) -> Self {
		self.data.push(*value);
		self
	}

	pub fn add_u16(mut self, value: &u16) -> Self {
		let mut wtr = vec![];
		wtr.write_u16::<LittleEndian>(*value).unwrap();

		self.data.extend(wtr);
		self
	}

	pub fn add_u32(mut self, value: &u32) -> Self {
		let mut wtr = vec![];
		wtr.write_u32::<LittleEndian>(*value).unwrap();

		self.data.extend(wtr);
		self
	}

	pub fn add_u64(mut self, value: &u64) -> Self {
		let mut wtr = vec![];
		wtr.write_u64::<LittleEndian>(*value).unwrap();

		self.data.extend(wtr);
		self
	}

	pub fn add_f64(mut self, value: &f64) -> Self {
		let mut wtr = vec![];
		wtr.write_f64::<LittleEndian>(*value).unwrap();

		self.data.extend(wtr);
		self
	}

	pub fn to_sheet(self) -> Sheet {
		Sheet::new(self.is_ok, self.data)
	}
}
