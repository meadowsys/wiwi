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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		MemoryUsage as _,
		MemoryUsageConst as _,
		MemoryUsageStatic as _
	};

	macro_rules! test_nonzero {
		{ $($test_name:ident($ty:ident = $val:literal))* } => {
			$(
				#[test]
				fn $test_name() {
					let num_nonzero: NonZero<$ty> = NonZero::new($val).unwrap();
					let num: $ty = $val;

					assert_eq!(num_nonzero.memory_usage(), num.memory_usage());
					assert_eq!(num_nonzero.memory_usage_const(), num.memory_usage_const());
					assert_eq!(<NonZero<$ty>>::MEMORY_USAGE, $ty::MEMORY_USAGE);
				}
			)*
		}
	}

	test_nonzero! {
		nonzero_u8_usage(u8 = 247)
		nonzero_u16_usage(u16 = 247)
		nonzero_u32_usage(u32 = 247)
		nonzero_u64_usage(u64 = 247)
		nonzero_u128_usage(u128 = 247)
		nonzero_usize_usage(usize = 247)

		nonzero_i8_usage(i8 = 24) // 7
		nonzero_i16_usage(i16 = 247)
		nonzero_i32_usage(i32 = 247)
		nonzero_i64_usage(i64 = 247)
		nonzero_i128_usage(i128 = 247)
		nonzero_isize_usage(isize = 247)
		nonzero_char_usage(char = '2') //47
	}
}
