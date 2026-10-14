use crate::{ MemoryUsage, MemoryUsageConst, MemoryUsageStatic };
use crate::macros::{ impl_memory_usage, impl_memory_usage_only_const, impl_memory_usage_only_static };

impl_memory_usage! {
	{ T: MemoryUsage, const N: usize } [T; N]
	|self| <[T]>::memory_usage(self);
}

impl_memory_usage_only_const! {
	{ T: MemoryUsageConst, const N: usize }
	{ T: [const] MemoryUsageConst, const N: usize }
	[T; N]
	|self| <[T]>::memory_usage_const(self);
}

impl_memory_usage_only_static! {
	{ T: MemoryUsageStatic, const N: usize }
	{ T: [const] MemoryUsageStatic, const N: usize }
	[T; N] => {
		// each element counts its own stack usage too
		T::MEMORY_USAGE * N
	};
}
