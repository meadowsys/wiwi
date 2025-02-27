//! Internal decoding implementations
use crate::num::*;
use crate::prelude::*;
use super::{ DecodeError, UnsafeBufWriteGuard };

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

/// Reads `rounds * 2` bytes from bytes_ptr, encoding pairs of chars into
/// bytes, then writes the decoded bytes into `dest`
///
/// # Safety
///
/// - `bytes_ptr` must be valid for reads for `rounds * 2` bytes
/// - `dest` must have enough capacity to write at least `rounds` bytes
pub(super) unsafe fn generic(
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
		let byte1 = unsafe { (*bytes_ptr).into_usize() };
		// SAFETY: same as above
		let byte2 = unsafe { (*next_byte_ptr).into_usize() };

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
