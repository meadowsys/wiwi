#![allow(
	unused_variables,
	reason = "wip"
)]
use crate::prelude::*;
use crate::num::*;
use super::{ Serialise, Serialiser, Deserialise, Input, Output };

#[repr(transparent)]
struct IntSerialiser<Num, const BYTES: usize>
where
	Num: BytesWithinBounds<BYTES>
{
	num: Num
}

impl<Num, const BYTES: usize> Serialiser<'_> for IntSerialiser<Num, BYTES>
where
	Num: BytesWithinBounds<BYTES>
{
	#[inline]
	fn serialise<O>(&self, out: O)
	where
		O: Output
	{
		let bytes = self.num.clone().into_le_bytes();

		let repr = if Num::SIGNED {
			// SAFETY: trait bound `BytesWithinBounds` ensures safety invariant
			unsafe { get_repr_info_signed_le(bytes) }
		} else {
			// SAFETY: same as above
			unsafe { get_repr_info_unsigned_le(bytes) }
		};

		// SAFETY: same as above again lol
		unsafe { serialise_repr_and_marker(out, bytes, repr) }
	}
}

/// # Safety
///
/// See safety comment on [`hint_bytes_valid`]
#[inline]
unsafe fn serialise_repr_and_marker<O, const BYTES: usize>(out: O, bytes: [u8; BYTES], repr: IntReprInfo)
where
	O: Output
{
	// SAFETY: caller promises safety preconditions are met
	unsafe { hint_bytes_valid::<BYTES>() }

	match repr {
		IntReprInfo::Inline => {}
		IntReprInfo::I8 => {}
		IntReprInfo::I16 if BYTES >= 2 => {}
		IntReprInfo::I24 if BYTES >= 3 => {}
		IntReprInfo::I32 if BYTES >= 4 => {}
		IntReprInfo::I48 if BYTES >= 6 => {}
		IntReprInfo::I64 if BYTES >= 8 => {}
		IntReprInfo::I96 if BYTES >= 12 => {}
		IntReprInfo::BigintUnsigned(_len) => {}
		IntReprInfo::BigintSigned(_len) => {}
		_ => {
			// SAFETY: we're using if guards to filter out branches that aren't
			// applicable (and won't happen) for our value of `BYTES`
			unsafe { hint::unreachable_unchecked() }
		}
	}

	todo!()
}

/// Helper marker trait for number types that both have the necessary traits
/// implemented, and are appropriately sized for, usage with [`IntSerialiser`]
///
/// # Safety
///
/// `BYTES` must be greater than 0, and less than or equal to 256.
unsafe trait BytesWithinBounds<const BYTES: usize>
where
	Self: ArrayConversions<BYTES> + Clone + IntSignedness
{}

// SAFETY: size is within bounds
unsafe impl BytesWithinBounds<1> for u8 {}

// SAFETY: size is within bounds
unsafe impl BytesWithinBounds<2> for u16 {}

// SAFETY: size is within bounds
unsafe impl BytesWithinBounds<4> for u32 {}

// SAFETY: size is within bounds
unsafe impl BytesWithinBounds<8> for u64 {}

// SAFETY: size is within bounds
unsafe impl BytesWithinBounds<16> for u128 {}

#[cfg(target_pointer_width = "16")]
// SAFETY: size is within bounds
unsafe impl BytesWithinBounds<2> for usize {}

#[cfg(target_pointer_width = "32")]
// SAFETY: size is within bounds
unsafe impl BytesWithinBounds<4> for usize {}

#[cfg(target_pointer_width = "64")]
// SAFETY: size is within bounds
unsafe impl BytesWithinBounds<8> for usize {}

// SAFETY: size is within bounds
unsafe impl BytesWithinBounds<1> for i8 {}

// SAFETY: size is within bounds
unsafe impl BytesWithinBounds<2> for i16 {}

// SAFETY: size is within bounds
unsafe impl BytesWithinBounds<4> for i32 {}

// SAFETY: size is within bounds
unsafe impl BytesWithinBounds<8> for i64 {}

// SAFETY: size is within bounds
unsafe impl BytesWithinBounds<16> for i128 {}

#[cfg(target_pointer_width = "16")]
// SAFETY: size is within bounds
unsafe impl BytesWithinBounds<2> for isize {}

#[cfg(target_pointer_width = "32")]
// SAFETY: size is within bounds
unsafe impl BytesWithinBounds<4> for isize {}

#[cfg(target_pointer_width = "64")]
// SAFETY: size is within bounds
unsafe impl BytesWithinBounds<8> for isize {}

enum IntReprInfo {
	Inline,
	I8,
	I16,
	I24,
	I32,
	I48,
	I64,
	I96,
	BigintUnsigned(u8),
	BigintSigned(u8)
}

struct UnsignedByteCount {
	count: u8,
	highest_bit_set: bool
}

/// # Safety
///
/// See safety comment on [`hint_bytes_valid`]
#[inline]
unsafe fn get_repr_info_unsigned_le<const BYTES: usize>(bytes: [u8; BYTES]) -> IntReprInfo {
	// SAFETY: we have same safety invariant as `get_byte_count_unsigned_le`
	let byte_count = unsafe { get_byte_count_unsigned_le(bytes) };

	match (byte_count.count, byte_count.highest_bit_set) {
		(1, false) => { IntReprInfo::I8 }
		(2, false) | (1, true) => { IntReprInfo::I16 }
		(3, false) | (2, true) => { IntReprInfo::I24 }
		(4, false) | (3, true) => { IntReprInfo::I32 }
		(5..=6, false) | (4..=5, true) => { IntReprInfo::I48 }
		(7..=8, false) | (6..=7, true) => { IntReprInfo::I64 }
		(9..=12, false) | (8..=11, true) => { IntReprInfo::I96 }
		(byte_count, _) => { IntReprInfo::BigintUnsigned(byte_count) }
	}
}

/// # Safety
///
/// See safety comment on [`hint_bytes_valid`]
#[inline]
unsafe fn get_repr_info_signed_le<const BYTES: usize>(bytes: [u8; BYTES]) -> IntReprInfo {
	// SAFETY: we have same safety invariant as `get_byte_count_signed_le`
	let byte_count = unsafe { get_byte_count_signed_le(bytes) };

	match byte_count {
		1 => { IntReprInfo::I8 }
		2 => { IntReprInfo::I16 }
		3 => { IntReprInfo::I24 }
		4 => { IntReprInfo::I32 }
		5..=6 => { IntReprInfo::I48 }
		7..=8 => { IntReprInfo::I64 }
		9..=12 => { IntReprInfo::I96 }
		byte_count => { IntReprInfo::BigintSigned(byte_count) }
	}
}

/// # Safety
///
/// See safety comment on [`hint_bytes_valid`]
#[inline]
unsafe fn get_byte_count_unsigned_le<const BYTES: usize>(bytes: [u8; BYTES]) -> UnsignedByteCount {
	// SAFETY: caller promises safety preconditions are met
	unsafe { hint_bytes_valid::<BYTES>() }

	for (i, byte) in bytes.into_iter().enumerate().rev() {
		// simple! just return the first byte (including the byte) where its not
		// all 0s. If the byte has the highest bit set to 1, a regular int would
		// need to go to the next "tier" of int, but a bigint would not need that,
		// so return the information seperately.

		if byte != 0 {
			let highest_bit_set = byte >> 7 != 0;
			return UnsignedByteCount {
				// `i + 1` is because `i` is 0 based index in the array,
				// but `count` is count of bytes
				count: (i + 1).into_u8_lossy(),
				highest_bit_set
			}
		}
	}

	// everything is empty so just return the minimum
	UnsignedByteCount {
		count: 1,
		highest_bit_set: false
	}
}

/// # Safety
///
/// See safety comment on [`hint_bytes_valid`]
#[inline]
unsafe fn get_byte_count_signed_le<const BYTES: usize>(bytes: [u8; BYTES]) -> u8 {
	// SAFETY: caller promises safety preconditions are met
	unsafe { hint_bytes_valid::<BYTES>() }

	let sign_bit = {
		// SAFETY: `bytes` is len `BYTES`, and we assert that `BYTES > 0`,
		// so this will be in bounds
		let byte = unsafe { *bytes.get_unchecked(BYTES - 1) };

		byte >> 7
	};

	// byte that has empty data and can (probably) be safely discarded.
	// if negative, all bits 1, if positive, all bits 0
	let empty_byte = if sign_bit == 0 { 0 } else { u8::MAX };

	for (i, byte) in bytes.into_iter().enumerate().rev() {
		// the following could be shortened to a one liner...
		// but this absolutely sucks for readability/maintainability, so nah
		// if byte != empty_byte { return (i + 1) as u8 + (byte >> 7 != sign_bit) as u8 }

		if byte == empty_byte {
			// byte is full of 1s if negative, or full of 0s if positive
			// this byte can (probably) be safely discarded; continue
		} else if byte >> 7 == sign_bit {
			// sign bit is the same, return up to / including this byte
			// iter range is 0 to BYTES - 1 (inclusive), so this return range
			// will be 1 to BYTES (inclusive), which is correct
			return (i + 1).into_u8_lossy()
		} else {
			// sign bit is different, return this byte and one more after it.
			// if the next byte would have the wrong sign, it would have returned
			// already in the previous branch. This won't ever overflow because
			// the first byte will not have a different sign (as... itself),
			// so will never reach here
			return (i + 2).into_u8_lossy()
		}
	}

	// everything is "empty", just return the minimum
	1
}

/// Helper to hint and assert in debug that `BYTES` is not 0 and less than 256
///
/// # Safety
///
/// `BYTES > 0` and `BYTES <= 256` must both be true. This function adds some
/// compiler hints for it as well, which makes it UB if these invariants aren't
/// met
#[expect(clippy::inline_always, reason = "in release this fn is no-op")]
#[inline(always)]
unsafe fn hint_bytes_valid<const BYTES: usize>() {
	// ints must be at least one byte
	debug_assert!(BYTES > 0);
	// largest bigint we support ~~for now~~ is 256 bytes
	debug_assert!(BYTES <= 256);

	// SAFETY: caller promises this precondition
	unsafe { hint::assert_unchecked(BYTES > 0) }
	// SAFETY: same as above
	unsafe { hint::assert_unchecked(BYTES <= 256) }
}
