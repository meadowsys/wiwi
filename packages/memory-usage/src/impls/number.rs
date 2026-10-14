use crate::macros::impl_memory_usage_size_of;

impl_memory_usage_size_of! {
	u8
	u16
	u32
	u64
	u128
	usize

	i8
	i16
	i32
	i64
	i128
	isize

	f32
	f64
}

#[cfg(feature = "nightly")]
impl_memory_usage_size_of! {
	f16
	f128
}

#[cfg(test)]
mod tests {
	use crate::{
		MemoryUsage as _,
		MemoryUsageConst as _,
		MemoryUsageStatic as _
	};

	const SIZE_SIZE: usize = cfg_select! {
		target_pointer_width = "16" => { 2 }
		target_pointer_width = "32" => { 4 }
		target_pointer_width = "64" => { 8 }
	};

	macro_rules! test_num {
		{ $(test $test_name:ident($ty:ident): $usage:expr)* } => {
			$(
				#[test]
				fn $test_name() {
					let num: $ty = 24u8 as _; // 7

					assert_eq!(num.memory_usage(), $usage);
					assert_eq!(num.memory_usage_const(), $usage);
					assert_eq!($ty::MEMORY_USAGE, $usage);
				}
			)*
		}
	}

	test_num! {
		test u8_usage(u8): 1
		test u16_usage(u16): 2
		test u32_usage(u32): 4
		test u64_usage(u64): 8
		test u128_usage(u128): 16
		test usize_usage(usize): SIZE_SIZE

		test i8_usage(i8): 1
		test i16_usage(i16): 2
		test i32_usage(i32): 4
		test i64_usage(i64): 8
		test i128_usage(i128): 16
		test isize_usage(isize): SIZE_SIZE

		test f32_usage(f32): 4
		test f64_usage(f64): 8
	}

	#[cfg(feature = "nightly")]
	test_num! {
		test f16_usage(f16): 2
		test f128_usage(f128): 16
	}
}
