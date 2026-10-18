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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		MemoryUsage as _,
		MemoryUsageStatic as _
	};

	macro_rules! test_saturating {
		{ $($test_name:ident($ty:ident))* } => {
			$(
				#[test]
				fn $test_name() {
					let num_saturating: Saturating<$ty> = Saturating(24); // 7
					let num: $ty = 24; // 7

					assert_eq!(num_saturating.memory_usage(), num.memory_usage());
					assert_eq!(<Saturating<$ty>>::MEMORY_USAGE, $ty::MEMORY_USAGE);
				}
			)*
		}
	}

	test_saturating! {
		saturating_u8_usage(u8)
		saturating_u16_usage(u16)
		saturating_u32_usage(u32)
		saturating_u64_usage(u64)
		saturating_u128_usage(u128)
		saturating_usize_usage(usize)

		saturating_i8_usage(i8)
		saturating_i16_usage(i16)
		saturating_i32_usage(i32)
		saturating_i64_usage(i64)
		saturating_i128_usage(i128)
		saturating_isize_usage(isize)
	}
}
