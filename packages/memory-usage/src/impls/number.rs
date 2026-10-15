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
		{ $($test_name:ident($ty:ident): $usage:expr)* } => {
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
		u8_usage(u8): 1
		u16_usage(u16): 2
		u32_usage(u32): 4
		u64_usage(u64): 8
		u128_usage(u128): 16
		usize_usage(usize): SIZE_SIZE

		i8_usage(i8): 1
		i16_usage(i16): 2
		i32_usage(i32): 4
		i64_usage(i64): 8
		i128_usage(i128): 16
		isize_usage(isize): SIZE_SIZE

		f32_usage(f32): 4
		f64_usage(f64): 8
	}

	#[cfg(feature = "nightly")]
	test_num! {
		f16_usage(f16): 2
		f128_usage(f128): 16
	}
}
