//! Internal encoding implementations

use super::UnsafeBufWriteGuard;

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

/// Reads `rounds` bytes from `bytes_ptr`, encoding them into 2 hex chars
/// per byte, then writes the output into `dest`
///
/// # SAFETY
///
/// - `bytes_ptr` must be valid for `num_rounds` bytes of reads
/// - `dest` must have enough capacity to write `num_rounds * 2` bytes into
pub unsafe fn generic<const UPPER: bool>(
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
