use crate::prelude::*;
use super::{ ChunkedSlice, UnsafeBufWriteGuard };

// // table unused, for ref only, cause it can be calculated
// pub const TABLE_ENCODER_LEN: usize = 32;
// pub static TABLE_ENCODER: &[u8; TABLE_ENCODER_LEN] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
// pub static TABLE_ENCODER_BASE32HEX: &[u8; TABLE_ENCODER_LEN] = b"0123456789ABCDEFGHIJKLMNOPQRSTUV";

pub const BINARY_FRAME_LEN: usize = 5;
pub const STRING_FRAME_LEN: usize = 8;

/// Encodes the given bytes into a base32 [`String`], as specified in
/// [RFC 4648].
///
/// [RFC 4648]: https://datatracker.ietf.org/doc/html/rfc4648#section-6
#[inline]
pub fn encode_base32(bytes: &[u8]) -> String {
	_encode::<25, b'A', { b'2' - 26 }>(bytes)
}

/// Encodes the given bytes into a base32 [`String`], using
/// the [hex encoding alphabet variant as defined in RFC 4648].
///
/// [hex encoding alphabet variant as defined in RFC 4648]: https://datatracker.ietf.org/doc/html/rfc4648#section-7
#[inline]
pub fn encode_base32hex(bytes: &[u8]) -> String {
	_encode::<9, b'0', { b'A' - 10 }>(bytes)
}

/// - `BREAKPOINT`: the gt comparison against this number to determin when to use
///   LOWER or UPPER_ADJUSTED
/// - `LOWER`: the amount to add to a section when it is lt than `BREAKPOINT`.
///   (ie. the lowest in the range)
/// - `UPPER_ADJUSTED`: the amount to add to a section when it is gte `BREAKPOINT`.
///   "Adjusted" means that `BREAKPOINT` should be subtracted from the UPPER
///   char value, so no subtraction needs to be done in runtime.
fn _encode<
	const BREAKPOINT: u8,
	const LOWER: u8,
	const UPPER_ADJUSTED: u8
>(bytes: &[u8]) -> String {
	// 5 bytes per group of 8 output chars
	let frames = bytes.len() / BINARY_FRAME_LEN;
	let remainder = bytes.len() % BINARY_FRAME_LEN;

	let capacity = if remainder == 0 {
		frames * STRING_FRAME_LEN
	} else {
		(frames + 1) * STRING_FRAME_LEN
	};

	let mut frames_iter = ChunkedSlice::<BINARY_FRAME_LEN>::new(bytes);
	let mut dest = UnsafeBufWriteGuard::with_capacity(capacity);

	for _ in 0..frames {
		// SAFETY: calculated
		let frame = unsafe { frames_iter.next_frame_unchecked() };

		// SAFETY: calculated
		unsafe { encode_frame::<BREAKPOINT, LOWER, UPPER_ADJUSTED>(frame, &mut dest) }
	}

	if remainder > 0 {
		// determine padding amount
		let padding_amount = match remainder {
			1 => { 6 }
			2 => { 4 }
			3 => { 3 }
			4 => { 1 }
			_ => {
				// SAFETY: `remainder` is calculated by mod 5, so it cannot be 5 or
				// more. and we just checked in an if statement that `remainder` is
				// greater than 0. therefore, `remainder` can only be 1, 2, 3, or 4,
				// all of which are covered by match branches.
				unsafe { hint::unreachable_unchecked() }
			}
		};

		let f = |frame: &[u8; 5]| {
			static PADDING: &[u8; 6] = b"======";

			// SAFETY: calculated
			unsafe { encode_frame::<BREAKPOINT, LOWER, UPPER_ADJUSTED>(frame, &mut dest) }

			// SAFETY: calculated
			let ptr = unsafe { dest.as_mut_ptr().sub(padding_amount) };

			// SAFETY: calculated
			unsafe { ptr::copy_nonoverlapping(PADDING.as_ptr(), ptr, padding_amount) }
		};

		// SAFETY: calculated
		unsafe { frames_iter.with_remainder_unchecked(f) }
	}

	// SAFETY: calculated
	let vec = unsafe { dest.into_full_vec() };

	// SAFETY: we only write valid ASCII as part of encoding process
	unsafe {
		debug_assert!(str::from_utf8(&vec).is_ok(), "output bytes valid utf-8");
		String::from_utf8_unchecked(vec)
	}
}

unsafe fn encode_frame<
	const BREAKPOINT: u8,
	const LOWER: u8,
	const UPPER_ADJUSTED: u8
>(frame: &[u8; BINARY_FRAME_LEN], dest: &mut UnsafeBufWriteGuard) {
	// keep first 5 bits from byte 0, leaving 3 bits left
	let byte1 = frame[0] >> 3;

	// take remaining 3 from byte 0, then 2 from byte 1, leaving 6 bits left
	let byte2 = ((frame[0] << 2) & 0b11100) | (frame[1] >> 6);

	// take 5 in middle of byte 1, leaving 1 bit left
	let byte3 = (frame[1] >> 1) & 0b11111;

	// take last bit from byte 1, then 4 from byte 2, leaving 4 bits left
	let byte4 = ((frame[1] << 4) & 0b10000) | (frame[2] >> 4);

	// take last 4 bits from byte 2, then 1 from byte 3, leaving 7 bits left
	let byte5 = ((frame[2] << 1) & 0b11110) | (frame[3] >> 7);

	// take 5 from byte 3, leaving 2 bits left
	let byte6 = (frame[3] >> 2) & 0b11111;

	// take remaining 2 bits from byte 3, then 3 bits from byte 4, leaving 5 bits left
	let byte7 = ((frame[3] << 3) & 0b11000) | (frame[4] >> 5);

	// take remaining 5 bits
	let byte8 = frame[4] & 0b11111;

	let bytes = [
		// multi cursor editing is great
		if byte1 > BREAKPOINT { byte1 + UPPER_ADJUSTED } else { byte1 + LOWER },
		if byte2 > BREAKPOINT { byte2 + UPPER_ADJUSTED } else { byte2 + LOWER },
		if byte3 > BREAKPOINT { byte3 + UPPER_ADJUSTED } else { byte3 + LOWER },
		if byte4 > BREAKPOINT { byte4 + UPPER_ADJUSTED } else { byte4 + LOWER },
		if byte5 > BREAKPOINT { byte5 + UPPER_ADJUSTED } else { byte5 + LOWER },
		if byte6 > BREAKPOINT { byte6 + UPPER_ADJUSTED } else { byte6 + LOWER },
		if byte7 > BREAKPOINT { byte7 + UPPER_ADJUSTED } else { byte7 + LOWER },
		if byte8 > BREAKPOINT { byte8 + UPPER_ADJUSTED } else { byte8 + LOWER }
	];

	// SAFETY: caller promises we can call this once
	unsafe { dest.write_bytes_const::<8>(bytes.as_ptr()) }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rfc_provided_examples() {
		let examples = [
			("", ""),
			("f", "MY======"),
			("fo", "MZXQ===="),
			("foo", "MZXW6==="),
			("foob", "MZXW6YQ="),
			("fooba", "MZXW6YTB"),
			("foobar", "MZXW6YTBOI======")
		];

		for (bytes, encoded) in examples {
			assert_eq!(encoded, encode_base32(bytes.as_bytes()));
		}
	}

	#[test]
	fn rfc_provided_examples_base32hex() {
		let examples = [
			("", ""),
			("f", "CO======"),
			("fo", "CPNG===="),
			("foo", "CPNMU==="),
			("foob", "CPNMUOG="),
			("fooba", "CPNMUOJ1"),
			("foobar", "CPNMUOJ1E8======")
		];

		for (bytes, encoded) in examples {
			assert_eq!(encoded, encode_base32hex(bytes.as_bytes()));
		}
	}
}
