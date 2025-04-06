extern crate thiserror;

use crate::prelude::*;
use super::UnsafeBufWriteGuard;

mod encode;
mod decode;

pub use self::encode::{
	TABLE_ENCODER_LEN,
	TABLE_ENCODER_LOWER,
	TABLE_ENCODER_UPPER
};

/// Encodes a slice of bytes into a String, using lowercase characters
#[inline]
pub fn encode_hex(bytes: &[u8]) -> String {
	_encode::<false>(bytes)
}

/// Encodes a slice of bytes into a String, using uppercase characters
#[inline]
pub fn encode_hex_upper(bytes: &[u8]) -> String {
	_encode::<true>(bytes)
}

/// Inner function with const generic `UPPER`
fn _encode<const UPPER: bool>(bytes: &[u8]) -> String {
	debug_assert!(bytes.len() >> (usize::BITS - 1) == 0, "size overflow");

	let len = bytes.len();
	// shl 1 is same as multiplying by 2
	let capacity = len << 1;
	let ptr = bytes.as_ptr();
	let mut dest = UnsafeBufWriteGuard::with_capacity(capacity);

	// SAFETY: we obtained `ptr` and `len` from `bytes`, so `ptr` is valid for `len`
	// reads, and we calculated and requested `dest` to allocate `len * 2` bytes
	unsafe { encode::generic::<UPPER>(ptr, &mut dest, len) };

	// SAFETY: we wrote into all the space we requested (`len * 2`)
	let vec = unsafe { dest.into_full_vec() };

	// SAFETY: `encode::generic` will only ever write the ASCII chars `0-9`, `a-f`,
	// and `A-F` into vec. ASCII is valid UTF-8
	unsafe {
		debug_assert!(str::from_utf8(&vec).is_ok(), "output bytes are valid utf-8");
		String::from_utf8_unchecked(vec)
	}
}

/// Decodes a slice of hex bytes into a byte vector. This function handles and
/// supports both uppercase and lowercase characters.
#[inline]
pub fn decode_hex(bytes: &[u8]) -> Result<Vec<u8>, DecodeError> {
	let len = bytes.len();

	// `AND 0b1` is chopping off all the other bits
	// if the last bit is 1 then it's odd, which is invalid
	if len & 0b1 != 0 { return Err(DecodeError::InvalidLength) }

	// shr 1 is same as div 2
	let capacity = len >> 1;
	let mut dest = UnsafeBufWriteGuard::with_capacity(capacity);
	let ptr = bytes.as_ptr();

	// SAFETY: ptr is readable for `capacity * 2` bytes (since `capacity` is
	// `len / 2` and `ptr` is readable for `len` bytes), and we requested `capacity`
	// bytes in `dest`
	unsafe { decode::generic(ptr, &mut dest, capacity)? }

	// SAFETY: we wrote into all the space we requested (`len / 2`)
	Ok(unsafe { dest.into_full_vec() })
}

/// Errors that can be encountered on decoding data (encoding data does not error)
// TODO: these errors could be improved.
#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
	/// Invalid length. Length is expected to be a multiple of two
	#[error("invalid length")]
	InvalidLength,
	/// Invalid character. Characters are only allowed to be in `0-9`, `a-f`, `A-F`
	#[error("invalid character")]
	InvalidChar
}

#[cfg(test)]
mod tests {
	extern crate hex;
	extern crate rand;

	use crate::prelude::*;
	use super::*;
	use rand::{ Rng, thread_rng };

	#[test]
	fn rfc_provided_examples() {
		let examples = [
			("", ""),
			("f", "66"),
			("fo", "666F"),
			("foo", "666F6F"),
			("foob", "666F6F62"),
			("fooba", "666F6F6261"),
			("foobar", "666F6F626172")
		];

		for (bytes, encoded) in examples {
			assert_eq!(encoded, encode_hex_upper(bytes.as_bytes()));
			assert_eq!(encoded.to_lowercase(), encode_hex(bytes.as_bytes()));
		}
	}

	#[test]
	fn randomised() {
		// (in_len, out_len)
		let expected_lengths = [
			(0usize, 0usize),
			(1, 2),
			(2, 4),
			(3, 6),
			(4, 8),
			(5, 10),
			(6, 12),
			(7, 14),
			(8, 16),
			(9, 18),
			(10, 20),
			(11, 22),
			(12, 24),
			(13, 26),
			(14, 28),
			(15, 30),
			(16, 32),
			(17, 34),
			(18, 36),
			(19, 38),
			(20, 40),

			(50, 100),
			(100, 200),
			(500, 1000),
			(1000, 2000),
			(100_000, 200_000),
			(1_000_000, 2_000_000),
		];
		let mut rng = thread_rng();

		for (expected_input_len, expected_output_len) in expected_lengths {
			for _ in 0usize..5 {
				let mut original_input = vec![0u8; expected_input_len];
				rng.fill(&mut *original_input);
				assert_eq!(original_input.len(), expected_input_len);

				let encoded_lower = encode_hex(&original_input);
				assert_eq!(encoded_lower.len(), expected_output_len);
				let encoded_upper = encode_hex_upper(&original_input);
				assert_eq!(encoded_upper.len(), expected_output_len);

				let decoded_lower = decode_hex(encoded_lower.as_bytes())
					.expect("can round trip decode just encoded data");
				assert_eq!(decoded_lower.len(), expected_input_len);
				assert_eq!(original_input, decoded_lower);

				let decoded_upper = decode_hex(encoded_upper.as_bytes())
					.expect("can round trip decode just encoded data");
				assert_eq!(decoded_upper.len(), expected_input_len);
				assert_eq!(original_input, decoded_upper);
			}
		}
	}

	#[test]
	fn hex_crate_compat() {
		let mut rng = thread_rng();

		let mut bytes = vec![0u8; 1000];
		rng.fill(&mut *bytes);
		let bytes = &*bytes;

		let wiwi_encoded = encode_hex(bytes);
		let hex_encoded = hex::encode(bytes);
		assert_eq!(wiwi_encoded, hex_encoded);

		let wiwi_decoded_hex = decode_hex(hex_encoded.as_bytes())
			.expect("wiwi can decode hex");
		let hex_decoded_wiwi = hex::decode(wiwi_encoded.as_bytes())
			.expect("hex can decode wiwi");

		assert_eq!(wiwi_decoded_hex, hex_decoded_wiwi);
	}
}
