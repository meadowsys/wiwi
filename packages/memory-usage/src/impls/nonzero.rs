use crate::macros::impl_memory_usage_size_of;
use core::num::NonZero;

// NonZero is generic, but the trait in the trait bound is unstable impl detail
// so we must list it out

impl_memory_usage_size_of! {
	NonZero<u8>
	NonZero<u16>
	NonZero<u32>
	NonZero<u64>
	NonZero<u128>
	NonZero<usize>

	NonZero<i8>
	NonZero<i16>
	NonZero<i32>
	NonZero<i64>
	NonZero<i128>
	NonZero<isize>

	NonZero<char>
}
