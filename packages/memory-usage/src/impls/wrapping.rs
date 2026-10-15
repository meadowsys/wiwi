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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		MemoryUsage as _,
		MemoryUsageConst as _,
		MemoryUsageStatic as _
	};

	macro_rules! test_wrapping {
		{ $($test_name:ident($ty:ident))* } => {
			$(
				#[test]
				fn $test_name() {
					let num_wrapping: Wrapping<$ty> = Wrapping(24); // 7
					let num: $ty = 24; // 7

					assert_eq!(num_wrapping.memory_usage(), num.memory_usage());
					assert_eq!(num_wrapping.memory_usage_const(), num.memory_usage_const());
					assert_eq!(<Wrapping<$ty>>::MEMORY_USAGE, $ty::MEMORY_USAGE);
				}
			)*
		}
	}

	test_wrapping! {
		wrapping_u8_usage(u8)
		wrapping_u16_usage(u16)
		wrapping_u32_usage(u32)
		wrapping_u64_usage(u64)
		wrapping_u128_usage(u128)
		wrapping_usize_usage(usize)

		wrapping_i8_usage(i8)
		wrapping_i16_usage(i16)
		wrapping_i32_usage(i32)
		wrapping_i64_usage(i64)
		wrapping_i128_usage(i128)
		wrapping_isize_usage(isize)
	}
}
