#![allow(warnings, reason = "deprecated")]
#![deprecated(note = "awa")]

use wiwi_util::prelude::*;
use crate::util_old::UnsafeBufWriteGuard;

/// Length of encoding table (not actually used in encoding/decoding data)
pub const TABLE_ENCODER_LEN: usize = 16;

/// Encoding table of lowercased characters, length 16, mapping a value from 0-15
/// to a hex byte (lower letters)
///
/// Note: this table is not actually used in the encoding/decoding implementation
pub static TABLE_ENCODER_LOWER: [u8; TABLE_ENCODER_LEN] = *b"0123456789abcdef";

/// Encoding table of uppercased characters, length 16, mapping a value from 0-15
/// to a hex byte (upper letters)
///
/// Note: this table is not actually used in the encoding/decoding implementation
pub static TABLE_ENCODER_UPPER: [u8; TABLE_ENCODER_LEN] = *b"0123456789ABCDEF";

/// Length of the table decoder (256)
const TABLE_DECODER_LEN: usize = 256;

/// Decoding table (with mappings for both upper and lower hex)
// TODO: this table is mostly empty... I wonder what we could do here to shrink it,
// without compromising speed (we could do what we did with z85 with None == 0xff)?
static TABLE_DECODER: &[Option<u8>; TABLE_DECODER_LEN] = &[
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	Some(0x00), Some(0x01), Some(0x02), Some(0x03), Some(0x04), Some(0x05), Some(0x06), Some(0x07), Some(0x08), Some(0x09), None,       None,       None,       None,       None,       None,
	None,       Some(0x0a), Some(0x0b), Some(0x0c), Some(0x0d), Some(0x0e), Some(0x0f), None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       Some(0x0a), Some(0x0b), Some(0x0c), Some(0x0d), Some(0x0e), Some(0x0f), None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,
	None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None,       None
];

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
	unsafe { encode_generic::<UPPER>(ptr, &mut dest, len) };

	// SAFETY: we wrote into all the space we requested (`len * 2`)
	let vec = unsafe { dest.into_full_vec() };

	// SAFETY: `encode::generic` will only ever write the ASCII chars `0-9`, `a-f`,
	// and `A-F` into vec. ASCII is valid UTF-8
	unsafe {
		debug_assert!(str::from_utf8(&vec).is_ok(), "output bytes are valid utf-8");
		String::from_utf8_unchecked(vec)
	}
}

/// Reads `rounds` bytes from `bytes_ptr`, encoding them into 2 hex chars
/// per byte, then writes the output into `dest`
///
/// # SAFETY
///
/// - `bytes_ptr` must be valid for `num_rounds` bytes of reads
/// - `dest` must have enough capacity to write `num_rounds * 2` bytes into
fn encode_generic<const UPPER: bool>(
	mut bytes_ptr: *const u8,
	dest: &mut UnsafeBufWriteGuard,
	rounds: usize
) {
	let char_a = if UPPER { b'A' } else { b'a' } - 10;
	let char_0 = b'0';

	for _ in 0..rounds {
		// SAFETY: we loop `num_rounds` times only, reading a byte each time,
		// and caller promises that `bytes_ptr` is valid to read for at least
		// that many bytes
		let byte = unsafe { *bytes_ptr };

		// SAFETY: same invariant as above. It's sound to have the pointer pointing
		// to the end of the memory section (as long as it isn't dereferenced)
		bytes_ptr = unsafe { bytes_ptr.add(1) };

		let char1 = byte >> 4;
		let char2 = byte & 0xf;

		let chars = [
			if char1 > 9 { char_a } else { char_0 } + char1,
			if char2 > 9 { char_a } else { char_0 } + char2
		];

		// SAFETY: caller guarantees `dest` is writeable to for at least
		// `num_rounds * 2` bytes, so we can write 2 bytes every iteration
		unsafe { dest.write_bytes_const::<2>(chars.as_ptr()) }
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
	unsafe { decode_generic(ptr, &mut dest, capacity)? }

	// SAFETY: we wrote into all the space we requested (`len / 2`)
	Ok(unsafe { dest.into_full_vec() })
}

/// Reads `rounds * 2` bytes from bytes_ptr, encoding pairs of chars into
/// bytes, then writes the decoded bytes into `dest`
///
/// # Safety
///
/// - `bytes_ptr` must be valid for reads for `rounds * 2` bytes
/// - `dest` must have enough capacity to write at least `rounds` bytes
unsafe fn decode_generic(
	mut bytes_ptr: *const u8,
	dest: &mut UnsafeBufWriteGuard,
	rounds: usize
) -> Result<(), DecodeError> {
	let table_ptr = TABLE_DECODER.as_ptr();

	for _ in 0..rounds {
		// SAFETY: bytes_ptr must be valid for `rounds * 2` bytes, and we loop
		// `rounds` times, each time reading 2 bytes (n, and n + 1)
		let next_byte_ptr = unsafe { bytes_ptr.add(1) };

		// SAFETY: caller promises `bytes_ptr` is valid for at least `rounds * 2`
		// bytes, see prev comment for details
		let byte1 = unsafe { (*bytes_ptr) as usize };
		// SAFETY: same as above
		let byte2 = unsafe { (*next_byte_ptr) as usize };

		/// # Safety
		///
		/// The valud stored in the variable passed in must be within the range of
		/// 0..=255 (for indexing the decode table)
		macro_rules! decode_byte_unsafe {
			($byte:ident) => {
				// SAFETY: macro caller promises var is within 0..=84,
				// ie. within range to index table ptr
				let ptr = unsafe { table_ptr.add($byte) };

				// SAFETY: as described above, ptr is valid to read
				let $byte = unsafe { *ptr };

				let $byte = match $byte {
					Some(byte) => { byte }
					None => { return Err(DecodeError::InvalidChar) }
				};
			}
		}

		// SAFETY: a byte can only be between `0..256`, which fits
		// within the lookup table

		// SAFETY: both vars were casted from bytes, which only has a range of 0..=255
		decode_byte_unsafe!(byte1);
		decode_byte_unsafe!(byte2);

		let byte = (byte1 << 4) | byte2;

		// SAFETY: we loop `rounds` times, and caller promises `dest` is writeable
		// for at least `rounds` bytes, so writing 1 per iteration is good
		unsafe { dest.write_bytes_const::<1>(&byte) }

		// SAFETY: caller promises `bytes_ptr` is readable from for at least
		// `num_rounds * 2` bytes, adding 2 per iter is sound. Also it's sound to
		// have the pointer pointing to the end of the memory section (as long as
		// it isn't dereferenced)
		unsafe { bytes_ptr = bytes_ptr.add(2) }
	}

	Ok(())
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
	use super::*;
	use wiwi_util::prelude::*;

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
