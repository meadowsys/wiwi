use ::std::{ ptr, slice };

pub const TABLE_ENCODER_LEN: usize = 16;
pub const TABLE_ENCODER_LOWER: [u8; TABLE_ENCODER_LEN] = *b"0123456789abcdef";
pub const TABLE_ENCODER_UPPER: [u8; TABLE_ENCODER_LEN] = *b"0123456789ABCDEF";

pub const TABLE_DECODER_LEN: usize = 256;
pub const TABLE_DECODER: [Option<u8>; TABLE_DECODER_LEN] = [
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

#[inline]
pub fn encode_hex(bytes: &[u8]) -> String {
	_encode(bytes, &TABLE_ENCODER_LOWER)
}

#[inline]
pub fn encode_hex_upper(bytes: &[u8]) -> String {
	_encode(bytes, &TABLE_ENCODER_UPPER)
}

fn _encode(bytes: &[u8], table: &[u8; 16]) -> String {
	let bytes_len = bytes.len();
	let capacity = bytes_len * 2;

	let mut dest = Vec::with_capacity(capacity);
	debug_assert!(dest.capacity() == capacity, "it didn't allocate???");

	let mut bytes_ptr = bytes as *const [u8] as *const u8;
	let mut dest_ptr = dest.as_mut_ptr();

	for _ in 0..bytes_len {
		debug_assert!(dest.len() + 2 <= dest.capacity(), "enough capacity allocated");

		unsafe {
			let byte = *bytes_ptr;
			bytes_ptr = bytes_ptr.add(1);

			let chars = [
				*table.get_unchecked((byte >> 4) as usize),
				*table.get_unchecked((byte & 0xf) as usize)
			];

			let chars = &chars as *const [u8] as *const u8;
			ptr::copy_nonoverlapping(chars, dest_ptr, 2);
			dest_ptr = dest_ptr.add(2);
		};
	}

	unsafe { dest.set_len(capacity) }
	debug_assert!(String::from_utf8(dest.clone()).is_ok(), "output bytes are valid utf-8");
	unsafe { String::from_utf8_unchecked(dest) }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn _encode_neon_uint8x8(
	mut bytes_ptr: *const u8,
	mut dest_ptr: *mut u8,
	num_rounds: usize
) -> *const u8 {
	use ::std::arch::aarch64::*;

	let four_lower_bits = vdup_n_u8(0xf);
	let nine = vdup_n_u8(9);
	let char_0 = vdup_n_u8(b'0');
	let char_a = vdup_n_u8(b'a' - 10);

	for _ in 0..num_rounds {
		let vec = vld1_u8(bytes_ptr);

		let upper_vals = vshr_n_u8::<4>(vec);
		let lower_vals = vand_u8(vec, four_lower_bits);

		let upper_cmp = vcgt_u8(upper_vals, nine);
		let lower_cmp = vcgt_u8(lower_vals, nine);

		let upper = vbsl_u8(upper_cmp, char_a, char_0);
		let lower = vbsl_u8(lower_cmp, char_a, char_0);

		let upper = vadd_u8(upper_vals, upper);
		let lower = vadd_u8(lower_vals, lower);

		let zipped = vzip_u8(upper, lower);
		vst1_u8_x2(dest_ptr, zipped);

		bytes_ptr = bytes_ptr.add(8);
		dest_ptr = dest_ptr.add(16);
	}

	bytes_ptr
}



pub fn decode_hex(bytes: &[u8]) -> Result<Vec<u8>, DecodeError> {
	if bytes.len() & 0b1 != 0 { return Err(DecodeError::InvalidLength) }

	// shift right 1 is same as dividing by 2
	let capacity = bytes.len() >> 1;
	let mut dest = Vec::with_capacity(capacity);
	debug_assert!(dest.capacity() == capacity, "it didn't allocate???");

	let mut bytes_ptr = bytes as *const [u8] as *const u8;
	let mut dest_ptr = dest.as_mut_ptr();

	// chunks of 2, so we take the half len, aka the capacity
	for _ in 0..capacity {
		unsafe {
			let byte1 = (*bytes_ptr) as usize;
			let byte2 = (*bytes_ptr.add(1)) as usize;

			let Some(byte1) = *TABLE_DECODER.get_unchecked(byte1) else {
				return Err(DecodeError::InvalidChar)
			};
			let Some(byte2) = *TABLE_DECODER.get_unchecked(byte2) else {
				return Err(DecodeError::InvalidChar)
			};

			*dest_ptr = (byte1 << 4) | byte2;

			bytes_ptr = bytes_ptr.add(2);
			dest_ptr = dest_ptr.add(1);
		}
	}

	unsafe { dest.set_len(capacity) }
	Ok(dest)
}

#[derive(Debug, ::thiserror::Error)]
pub enum DecodeError {
	#[error("invalid length")]
	InvalidLength,
	#[error("invalid character")]
	InvalidChar
}

#[cfg(test)]
mod tests {
	use super::*;
	use ::rand::{ Rng, thread_rng };

	#[test]
	fn randomised() {
		// (bytes_len, encoded_len)
		// (expected_input_len, expected_output_len)
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

				// this is enforced by debug_assert! in the code, so this already
				// is validated if tests are run in debug, but still,
				assert_eq!(encoded_lower.len(), encoded_lower.capacity());
				assert_eq!(decoded_lower.len(), decoded_lower.capacity());
				assert_eq!(encoded_upper.len(), encoded_upper.capacity());
				assert_eq!(decoded_upper.len(), decoded_upper.capacity());
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
		let hex_encoded = ::hex::encode(bytes);
		assert_eq!(wiwi_encoded, hex_encoded);

		let wiwi_decoded_hex = decode_hex(hex_encoded.as_bytes())
			.expect("wiwi can decode hex");
		let hex_decoded_wiwi = ::hex::decode(wiwi_encoded.as_bytes())
			.expect("hex can decode wiwi");

		assert_eq!(wiwi_decoded_hex, hex_decoded_wiwi);
	}

	#[test]
	fn test_neon_impl() {
		const IN_SIZE: usize = 1024 * 1024;
		const NUM_ROUNDS: usize = IN_SIZE / 8;

		let mut rng = thread_rng();

		let mut bytes = vec![0u8; IN_SIZE];
		rng.fill(&mut *bytes);
		let bytes = &*bytes;

		let regular_encoded = encode_hex(bytes);
		let neon_encoded = unsafe {
			let capacity = bytes.len() * 2;
			let mut dest = Vec::with_capacity(capacity);
			_encode_neon_uint8x8(
				bytes as *const [u8] as *const u8,
				dest.as_mut_ptr(),
				NUM_ROUNDS
			);
			dest.set_len(capacity);
			unsafe { String::from_utf8_unchecked(dest) }
		};

		assert_eq!(regular_encoded, neon_encoded);
	}
}
