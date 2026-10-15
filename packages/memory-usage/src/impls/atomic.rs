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

#[cfg(feature = "nightly")]
#[cfg(target_has_atomic = "128")]
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
