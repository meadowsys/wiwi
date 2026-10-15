use crate::macros::impl_memory_usage_size_of;
use core::num::Wrapping;

impl_memory_usage_size_of! {
	Wrapping<u8>
	Wrapping<u16>
	Wrapping<u32>
	Wrapping<u64>
	Wrapping<u128>
	Wrapping<usize>

	Wrapping<i8>
	Wrapping<i16>
	Wrapping<i32>
	Wrapping<i64>
	Wrapping<i128>
	Wrapping<isize>
}
