use crate::num_traits::*;
use std::hint::unreachable_unchecked;

pub struct Encoder {
	output: Vec<u8>,
	/// Bits that don't yet make up a whole byte
	partial_bits: u8,
	/// SAFETY: this must always be less than 8.
	num_partial_bits: u8
}

impl Encoder {
	#[inline]
	pub fn new() -> Self {
		Self::from_vec(Vec::new())
	}

	#[inline]
	pub fn with_output_capacity(capacity: usize) -> Self {
		Self::from_vec(Vec::with_capacity(capacity))
	}

	// should this function be public?
	#[inline]
	fn from_vec(vec: Vec<u8>) -> Self {
		Self { output: vec, partial_bits: 0, num_partial_bits: 0 }
	}

	/// Returns the encoded byte vector
	#[inline]
	pub fn into_bytes(mut self) -> Vec<u8> {
		if self.num_partial_bits > 0 { self.output.push(self.partial_bits) }
		self.output
	}

	/// Writes the specified amount of bits from the provided u8
	///
	/// # Panics
	///
	/// Panics if the amount of bits to write is more than 8
	#[inline]
	pub fn write_bits_u8(&mut self, num_bits: usize, bits: u8) {
		self.write_bits_u8_checked(num_bits, bits)
			.expect("amount of bits cannot be more than 8")
	}

	/// Writes the specified amount of bits from the provided u16
	///
	/// # Panics
	///
	/// Panics if the amount of bits to write is more than 16
	#[inline]
	pub fn write_bits_u16(&mut self, num_bits: usize, bits: u16) {
		self.write_bits_u16_checked(num_bits, bits)
			.expect("amount of bits cannot be more than 16")
	}

	/// Writes the specified amount of bits from the provided u32
	///
	/// # Panics
	///
	/// Panics if the amount of bits to write is more than 32
	#[inline]
	pub fn write_bits_u32(&mut self, num_bits: usize, bits: u32) {
		self.write_bits_u32_checked(num_bits, bits)
			.expect("amount of bits cannot be more than 32")
	}

	/// Writes the specified amount of bits from the provided u64
	///
	/// # Panics
	///
	/// Panics if the amount of bits to write is more than 64
	#[inline]
	pub fn write_bits_u64(&mut self, num_bits: usize, bits: u64) {
		self.write_bits_u64_checked(num_bits, bits)
			.expect("amount of bits cannot be more than 64")
	}

	/// Writes the specified amount of bits from the provided u128
	///
	/// # Panics
	///
	/// Panics if the amount of bits to write is more than 128
	#[inline]
	pub fn write_bits_u128(&mut self, num_bits: usize, bits: u128) {
		self.write_bits_u128_checked(num_bits, bits)
			.expect("amount of bits cannot be more than 128")
	}

	/// Writes the specified amount of bits from the provided u8, checking that
	/// the amount of bits to write is 8 or less
	#[inline]
	pub fn write_bits_u8_checked(&mut self, num_bits: usize, bits: u8) -> Option<()> {
		(num_bits <= u8::BITS.into_usize())
			.then(|| unsafe { self.write_bits_u8_unchecked(num_bits, bits) })
	}

	/// Writes the specified amount of bits from the provided u16, checking that
	/// the amount of bits to write is 16 or less
	#[inline]
	pub fn write_bits_u16_checked(&mut self, num_bits: usize, bits: u16) -> Option<()> {
		(num_bits <= u16::BITS.into_usize())
			.then(|| unsafe { self.write_bits_u16_unchecked(num_bits, bits) })
	}

	/// Writes the specified amount of bits from the provided u32, checking that
	/// the amount of bits to write is 32 or less
	#[inline]
	pub fn write_bits_u32_checked(&mut self, num_bits: usize, bits: u32) -> Option<()> {
		(num_bits <= u32::BITS.into_usize())
			.then(|| unsafe { self.write_bits_u32_unchecked(num_bits, bits) })
	}

	/// Writes the specified amount of bits from the provided u64, checking that
	/// the amount of bits to write is 64 or less
	#[inline]
	pub fn write_bits_u64_checked(&mut self, num_bits: usize, bits: u64) -> Option<()> {
		(num_bits <= u64::BITS.into_usize())
			.then(|| unsafe { self.write_bits_u64_unchecked(num_bits, bits) })
	}

	/// Writes the specified amount of bits from the provided u128, checking that
	/// the amount of bits to write is 128 or less
	#[inline]
	pub fn write_bits_u128_checked(&mut self, num_bits: usize, bits: u128) -> Option<()> {
		(num_bits <= u128::BITS.into_usize())
			.then(|| unsafe { self.write_bits_u128_unchecked(num_bits, bits) })
	}

	/// Writes the specified amount of bits from the provided u8
	///
	/// Currently, this just casts `bits` to `u128` and delegates the work to
	/// [`write_bits_u128_unchecked`]. Not the most efficient, but it works correctly.
	///
	/// # Safety
	///
	/// This function will not check that `num_bits` is less than or equal to 8.
	/// It is undefined behaviour if the provided amount is greater than 8.
	///
	/// # Examples
	///
	/// TODO
	///
	/// [`write_bits_u128_unchecked`]: Self::write_bits_u128_unchecked
	#[inline]
	pub unsafe fn write_bits_u8_unchecked(&mut self, num_bits: usize, bits: u8) {
		if num_bits > u8::BITS.into_usize() { unreachable_unchecked() }
		self.write_bits_u128_unchecked(num_bits, bits.into_u128());
	}

	/// Writes the specified amount of bits from the provided u16
	///
	/// Currently, this just casts `bits` to `u128` and delegates the work to
	/// [`write_bits_u128_unchecked`]. Not the most efficient, but it works correctly.
	///
	/// # Safety
	///
	/// This function will not check that `num_bits` is less than or equal to 16.
	/// It is undefined behaviour if the provided amount is greater than 16.
	///
	/// # Examples
	///
	/// TODO
	///
	/// [`write_bits_u128_unchecked`]: Self::write_bits_u128_unchecked
	#[inline]
	pub unsafe fn write_bits_u16_unchecked(&mut self, num_bits: usize, bits: u16) {
		if num_bits > u16::BITS.into_usize() { unreachable_unchecked() }
		self.write_bits_u128_unchecked(num_bits, bits.into_u128());
	}

	/// Writes the specified amount of bits from the provided u32
	///
	/// Currently, this just casts `bits` to `u128` and delegates the work to
	/// [`write_bits_u128_unchecked`]. Not the most efficient, but it works correctly.
	///
	/// # Safety
	///
	/// This function will not check that `num_bits` is less than or equal to 32.
	/// It is undefined behaviour if the provided amount is greater than 32.
	///
	/// # Examples
	///
	/// TODO
	///
	/// [`write_bits_u128_unchecked`]: Self::write_bits_u128_unchecked
	#[inline]
	pub unsafe fn write_bits_u32_unchecked(&mut self, num_bits: usize, bits: u32) {
		if num_bits > u32::BITS.into_usize() { unreachable_unchecked() }
		self.write_bits_u128_unchecked(num_bits, bits.into_u128());
	}

	/// Writes the specified amount of bits from the provided u64
	///
	/// Currently, this just casts `bits` to `u128` and delegates the work to
	/// [`write_bits_u128_unchecked`]. Not the most efficient, but it works correctly.
	///
	/// # Safety
	///
	/// This function will not check that `num_bits` is less than or equal to 64.
	/// It is undefined behaviour if the provided amount is greater than 64.
	///
	/// # Examples
	///
	/// TODO
	///
	/// [`write_bits_u128_unchecked`]: Self::write_bits_u128_unchecked
	#[inline]
	pub unsafe fn write_bits_u64_unchecked(&mut self, num_bits: usize, bits: u64) {
		if num_bits > u64::BITS.into_usize() { unreachable_unchecked() }
		self.write_bits_u128_unchecked(num_bits, bits.into_u128());
	}

	/// Writes the specified amount of bits from the provided u128
	///
	/// The specified amount of bits will be read from the lower bits of the
	/// provided integer, and all other bits are silently ignored. For example,
	/// when writing 3 bits, only when the integer value of `bits` is less than
	/// or equal to 7 will no information be lost.
	///
	/// # Safety
	///
	/// This function will not check that `num_bits` is less than or equal to 128.
	/// It is undefined behaviour if the provided amount is greater than 128.
	///
	/// # Examples
	///
	/// TODO
	pub unsafe fn write_bits_u128_unchecked(&mut self, mut num_bits: usize, mut bits: u128) {
		if num_bits > u128::BITS.into_usize() { unreachable_unchecked() }
		if self.num_partial_bits >= 8 { unreachable_unchecked() }

		// fill existing partial byte, if there's already stuff there
		if self.num_partial_bits > 0 {
			// shift our bits up to fill above existing bits, and merge
			self.partial_bits |= (bits << self.num_partial_bits).into_u8_lossy();

			// check if we filled it
			let num_partial_bits = self.num_partial_bits + num_bits.into_u8_lossy();
			if num_partial_bits >= 8 {
				// we haven't overwritten `self.num_partial_bits` before this
				let prev_empty = 8 - self.num_partial_bits;

				// remove what got stored in that byte
				num_bits -= prev_empty.into_usize();
				bits >>= prev_empty;

				self.output.push(self.partial_bits);

				// reset
				self.partial_bits = 0;
				self.num_partial_bits = 0;
			} else {
				// didn't fill it

				// clear top unfilled bits in case the provided bits had extra over
				// what was asked to push
				// shift up this much, then back down, to clear
				let clearing_shift = 8 - num_partial_bits;
				self.partial_bits <<= clearing_shift;
				self.partial_bits >>= clearing_shift;

				// we haven't overwritten `self.num_partial_bits` before this
				self.num_partial_bits = num_partial_bits;

				// we won't have any more if we didn't even fill the existing bits
				return
			}
		}

		// check for full bytes we can push in straight away
		// int div 8
		let full_bytes = num_bits >> 3;
		// SAFETY: num_bits cannot be greater than 128, and so dividing by 8
		// cannot be more than 16
		if full_bytes > 16 { unreachable_unchecked() }

		if full_bytes > 0 {
			// write the smaller ones before the larger ones - little endian
			let bytes = bits.to_le_bytes();
			self.output.extend_from_slice(&bytes[..full_bytes]);
		}

		// check for uneven remaining amount
		// rem 8
		let partial_bits = num_bits & 0b111;
		// SAFETY: rem 8 must be lt 8
		if partial_bits > 8 { unreachable_unchecked() }

		if partial_bits > 0 {
			// clear top unfilled bits in case the provided bits had extra over
			// what was asked to push
			// shift up this much, then back down, to clear
			let clearing_shift = 8 - partial_bits;

			// shift down to get the upper remainder
			// mul 8 is amount we've covered in full bytes (might be zero but that's fine)
			self.partial_bits = (bits >> (full_bytes << 3)).into_u8_lossy();
			self.partial_bits <<= clearing_shift;
			self.partial_bits >>= clearing_shift;

			self.num_partial_bits = partial_bits.into_u8_lossy();
		}
	}

	/// Writes the specified amount of bits from the provided usize.
	///
	/// The specified amount of bits will be read from the lower bits of the
	/// provided integer, and all other bits are silently ignored. For example,
	/// when writing 3 bits, only when the integer value of `bits` is less than
	/// or equal to 7 will no information be lost.
	///
	/// # Safety
	///
	/// This function will not check that `num_bits` is less than or equal to
	#[cfg_attr(target_pointer_width = "16", doc = "16.")]
	#[cfg_attr(target_pointer_width = "32", doc = "32.")]
	#[cfg_attr(target_pointer_width = "64", doc = "64.")]
	/// It is undefined behaviour if the provided amount is greater than
	#[cfg_attr(target_pointer_width = "16", doc = "16.")]
	#[cfg_attr(target_pointer_width = "32", doc = "32.")]
	#[cfg_attr(target_pointer_width = "64", doc = "64.")]
	///
	/// # Examples
	///
	/// TODO
	#[inline]
	pub unsafe fn write_bits_usize_unchecked(&mut self, num_bits: usize, bits: usize) {
		if num_bits > usize::BITS.into_usize() { unreachable_unchecked() }
		self.write_bits_u128_unchecked(num_bits, bits.into_u128());
	}
}

// whoop
// pub enum ReadStatus<T> {
// 	Some {
// 		bits: T
// 	},
// 	Partial {
// 		bits_read: usize,
// 		bits: T
// 	},
// 	None
// }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn write_bits_u128() {
		unsafe {
			let mut encoder = Encoder::new();
			assert_eq!(encoder.num_partial_bits, 0);
			assert_eq!(encoder.partial_bits, 0);
			assert_eq!(&*encoder.output, &[]);

			// partial bits
			encoder.write_bits_u128_unchecked(3, 0b010);
			assert_eq!(encoder.num_partial_bits, 3);
			assert_eq!(encoder.partial_bits, 0b010);
			assert_eq!(&*encoder.output, &[]);

			// partial bits and ignoring other bits
			encoder.write_bits_u128_unchecked(4, u128::MAX);
			assert_eq!(encoder.num_partial_bits, 7);
			assert_eq!(encoder.partial_bits, 0b1111010);
			assert_eq!(&*encoder.output, &[]);

			// filling partial bits exactly
			encoder.write_bits_u128_unchecked(1, u128::MAX);
			assert_eq!(encoder.num_partial_bits, 0);
			assert_eq!(encoder.partial_bits, 0);
			assert_eq!(&*encoder.output, &[0b11111010]);

			encoder.write_bits_u128_unchecked(5, 0b10101);
			assert_eq!(encoder.num_partial_bits, 5);
			assert_eq!(encoder.partial_bits, 0b10101);
			assert_eq!(&*encoder.output, &[0b11111010]);

			// pushing many bits with existing partial
			// also those upper 2 `11` bytes are ignored
			encoder.write_bits_u128_unchecked(81, 0b11_010110011000011011001011001101001011111111011100001110000000001001000100110011111u128);
			assert_eq!(encoder.num_partial_bits, 6);
			assert_eq!(encoder.partial_bits, 0b010110);
			assert_eq!(&*encoder.output, &[
				0b11111010,
				0b11110101,
				0b00110011,
				0b10010001,
				0b00000000,
				0b00001110,
				0b11110111,
				0b00101111,
				0b11001101,
				0b10110010,
				0b01100001
			]);

			// write max amount of bits (top half 0s, bottom half 1s)
			encoder.write_bits_u128_unchecked(128, u64::MAX.into_u128());
			assert_eq!(encoder.num_partial_bits, 6);
			assert_eq!(encoder.partial_bits, 0b000000);
			assert_eq!(&*encoder.output, &[
				0b11111010,
				0b11110101,
				0b00110011,
				0b10010001,
				0b00000000,
				0b00001110,
				0b11110111,
				0b00101111,
				0b11001101,
				0b10110010,
				0b01100001,
				0b11010110,
				0b11111111,
				0b11111111,
				0b11111111,
				0b11111111,
				0b11111111,
				0b11111111,
				0b11111111,
				0b00111111,
				0b00000000,
				0b00000000,
				0b00000000,
				0b00000000,
				0b00000000,
				0b00000000,
				0b00000000
			]);
		}
	}
}
