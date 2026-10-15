use crate::macros::{ impl_memory_usage_static, impl_memory_usage_size_of };
use core::sync::atomic::*;

// todo generic atomic when its stable

#[cfg(target_has_atomic = "8")]
impl_memory_usage_size_of! {
	AtomicU8
	AtomicI8
	AtomicBool
}

#[cfg(target_has_atomic = "16")]
impl_memory_usage_size_of! {
	AtomicU16
	AtomicI16
}

#[cfg(target_has_atomic = "32")]
impl_memory_usage_size_of! {
	AtomicU32
	AtomicI32
}

#[cfg(target_has_atomic = "64")]
impl_memory_usage_size_of! {
	AtomicU64
	AtomicI64
}

#[cfg(all(
	target_has_atomic = "128",
	feature = "nightly"
))]
impl_memory_usage_size_of! {
	AtomicU128
	AtomicI128
}

#[cfg(target_has_atomic = "ptr")]
impl_memory_usage_size_of! {
	AtomicUsize
	AtomicIsize
}

impl_memory_usage_static! {
	{ T }
	{ T }
	AtomicPtr<T>
	=> size_of::<AtomicPtr<T>>();
}

impl_memory_usage_size_of! {
	Ordering
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		MemoryUsage as _,
		MemoryUsageConst as _,
		MemoryUsageStatic as _
	};

	#[test]
	fn atomic_ordering_usage() {
		let ordering = Ordering::Relaxed;

		assert_eq!(ordering.memory_usage(), 1);
		assert_eq!(ordering.memory_usage_const(), 1);
		assert_eq!(Ordering::MEMORY_USAGE, 1);
	}

	macro_rules! test_atomic {
		{
			$(
				#[$cfg:meta]
				$test_name:ident($ty_atomic:path, $ty:ty)
			)*
		} => {
			$(
				#[test]
				fn $test_name() {
					let num_atomic: $ty_atomic = <$ty_atomic>::new(Default::default());
					let num: $ty = Default::default();

					assert_eq!(num_atomic.memory_usage(), num.memory_usage());
					assert_eq!(num_atomic.memory_usage_const(), num.memory_usage_const());
					assert_eq!(<$ty_atomic>::MEMORY_USAGE, <$ty>::MEMORY_USAGE);
				}
			)*
		}
	}

	test_atomic! {
		#[cfg(target_has_atomic = "8")]
		atomic_u8_usage(AtomicU8, u8)
		#[cfg(target_has_atomic = "16")]
		atomic_u16_usage(AtomicU16, u16)
		#[cfg(target_has_atomic = "32")]
		atomic_u32_usage(AtomicU32, u32)
		#[cfg(target_has_atomic = "64")]
		atomic_u64_usage(AtomicU64, u64)
		#[cfg(all(
			target_has_atomic = "128",
			feature = "nightly"
		))]
		atomic_u128_usage(AtomicU128, u128)
		#[cfg(target_has_atomic = "ptr")]
		atomic_usize_usage(AtomicUsize, usize)

		#[cfg(target_has_atomic = "8")]
		atomic_i8_usage(AtomicI8, i8)
		#[cfg(target_has_atomic = "16")]
		atomic_i16_usage(AtomicI16, i16)
		#[cfg(target_has_atomic = "32")]
		atomic_i32_usage(AtomicI32, i32)
		#[cfg(target_has_atomic = "64")]
		atomic_i64_usage(AtomicI64, i64)
		#[cfg(all(
			target_has_atomic = "128",
			feature = "nightly"
		))]
		atomic_i128_usage(AtomicI128, i128)
		#[cfg(target_has_atomic = "ptr")]
		atomic_isize_usage(AtomicIsize, isize)

		#[cfg(target_has_atomic = "8")]
		atomic_bool_usage(AtomicBool, bool)
		#[cfg(target_has_atomic = "ptr")]
		atomic_ptr_usage(AtomicPtr<()>, *const ())
	}
}
