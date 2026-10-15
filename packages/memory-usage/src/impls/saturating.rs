use crate::macros::impl_memory_usage_size_of;
use core::num::Saturating;

impl_memory_usage_size_of! {
	Saturating<u8>
	Saturating<u16>
	Saturating<u32>
	Saturating<u64>
	Saturating<u128>
	Saturating<usize>

	Saturating<i8>
	Saturating<i16>
	Saturating<i32>
	Saturating<i64>
	Saturating<i128>
	Saturating<isize>
}
