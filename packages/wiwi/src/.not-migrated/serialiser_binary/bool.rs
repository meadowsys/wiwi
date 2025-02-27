use super::internal_prelude::*;

impl Serialise for bool {
	type Serialiser<'h> = BoolSerialiser;

	fn build_serialiser(&self) -> BoolSerialiser {
		BoolSerialiser::new(*self)
	}
}

pub struct BoolSerialiser {
	val: bool
}

impl BoolSerialiser {
	fn new(val: bool) -> Self {
		Self { val }
	}
}

impl<'h> Serialiser<'h> for BoolSerialiser {
	unsafe fn needed_capacity(&self) -> usize {
		1
	}

	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		// - false is 0xa0, true is 0xa1
		// - false casts to integer 0, true to 1

		// SAFETY: provided buffer is guaranteed by caller to have reserved
		// at least the amount returned by `calc_needed_capacity` (1)
		unsafe { buf.write_byte(self.val as u8 + 0xa0) }
	}
}

impl<'h> Deserialise<'h> for bool {
	type Error = Error;

	fn deserialise_with_marker<I: Input<'h>>(_buf: &mut I, marker: u8) -> Result<bool> {
		if marker >> 1 == MARKER_BOOL_UPPER_BITS_COMMON {
			Ok(marker & 0b1 != 0)
		} else {
			expected(DESC_EXPECTED_BOOL)
				.found_something_else()
				.wrap()
		}
	}
}
