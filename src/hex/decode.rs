use crate::encoding_utils::UnsafeBufWriteGuard;
use super::DecodeError;

// table is 256 long
static TABLE_DECODER: &[Option<u8>] = &[
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

/// number of rounds is the same as input / 2,
/// or the count of output bytes
pub(super) unsafe fn generic(
	mut bytes_ptr: *const u8,
	dest: &mut UnsafeBufWriteGuard,
	rounds: usize
) -> Result<(), DecodeError> {
	let table_ptr = TABLE_DECODER as *const [Option<u8>] as *const Option<u8>;

	for _ in 0..rounds {
		unsafe {
			let byte1 = (*bytes_ptr) as usize;
			let byte2 = (*bytes_ptr.add(1)) as usize;

			// SAFETY: a byte can only be between `0..256`, which fits
			// within the lookup table

			let Some(byte1) = *table_ptr.add(byte1) else {
				return Err(DecodeError::InvalidChar)
			};
			let Some(byte2) = *table_ptr.add(byte2) else {
				return Err(DecodeError::InvalidChar)
			};

			dest.write_bytes_const::<1>(&((byte1 << 4) | byte2));
			bytes_ptr = bytes_ptr.add(2);
		}
	}

	Ok(())
}
