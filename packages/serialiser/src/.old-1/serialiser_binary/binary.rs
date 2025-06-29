use super::internal_prelude::*;
use super::NumberSerialiserUnsigned;
use std::ops::Deref;

pub enum Binary<'h> {
	Slice(&'h [u8]),
	Owned(Vec<u8>)
}

impl<'h> Binary<'h> {
	pub fn into_vec(self) -> Vec<u8> {
		match self {
			Self::Slice(inner) => { inner.into() }
			Self::Owned(inner) => { inner }
		}
	}

	pub fn try_into_vec(self) -> Result<Vec<u8>, Self> {
		match self {
			Self::Slice(inner) => { Err(Self::Slice(inner)) }
			Self::Owned(inner) => { Ok(inner) }
		}
	}
}

impl<'h> Deref for Binary<'h> {
	type Target = [u8];

	fn deref(&self) -> &[u8] {
		match self {
			Self::Owned(inner) => { inner }
			Self::Slice(inner) => { inner }
		}
	}
}

impl<'b> Serialise for Binary<'b> {
	type Serialiser<'h> = BinarySerialiser<'h> where Self: 'h;

	fn build_serialiser(&self) -> BinarySerialiser<'_> {
		BinarySerialiser::new(self)
	}
}

pub struct BinarySerialiser<'h> {
	slice: &'h [u8],
	len_ser: Option<<usize as Serialise>::Serialiser<'h>>
}

impl<'h> BinarySerialiser<'h> {
	pub(super) fn new(slice: &'h [u8]) -> Self {
		let len_ser = if slice.len() > u8::MAX.into_usize() {
			Some(NumberSerialiserUnsigned::new(slice.len()))
		} else {
			None
		};

		Self { slice, len_ser }
	}
}

impl<'h> Serialiser<'h> for BinarySerialiser<'h> {
	unsafe fn needed_capacity(&self) -> usize {
		let marker = if let Some(len_ser) = &self.len_ser {
			// one marker + serialised usize
			1 + len_ser.needed_capacity()
		} else {
			// one marker + one byte len
			2
		};

		marker + self.slice.len()
	}

	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if let Some(len_ser) = &self.len_ser {
			buf.write_byte(MARKER_BINARY_XL);
			len_ser.serialise(buf);
		} else {
			buf.write_byte(MARKER_BINARY_8);
			buf.write_byte(self.slice.len().into_u8_lossy());
		}

		buf.write_bytes(self.slice);
	}
}

impl<'h> Deserialise<'h> for Binary<'h> {
	type Error = Error;

	fn deserialise_with_marker<I: Input<'h>>(buf: &mut I, marker: u8) -> Result<Binary<'h>> {
		let len = match marker {
			MARKER_BINARY_8 => {
				use_ok!(
					buf.read_byte(),
					byte => byte.into_usize(),
					#err err => err.expected(DESC_EXPECTED_BINARY).wrap()
				)
			}
			MARKER_BINARY_XL => {
				use_ok!(
					usize::deserialise(buf),
					#err err => err.expected(DESC_EXPECTED_BINARY).wrap()
				)
			}
			_ => {
				return expected(DESC_EXPECTED_BINARY)
					.found_something_else()
					.wrap()
			}
		};

		let slice = use_ok!(
			buf.read_bytes(len),
			#err err => err.expected(DESC_EXPECTED_BINARY).wrap()
		);

		Ok(Binary::Slice(slice))
	}
}
