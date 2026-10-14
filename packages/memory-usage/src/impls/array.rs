use crate::{ MemoryUsage, MemoryUsageConst, MemoryUsageStatic };
use crate::macros::{ impl_memory_usage, impl_memory_usage_only_const, impl_memory_usage_only_static };

impl_memory_usage! {
	{ T: MemoryUsage, const N: usize } [T; N] |self| {
		// each element counts its own stack usage too
		self.iter().map(T::memory_usage).sum()
	};
}

impl_memory_usage_only_const! {
	{ T: MemoryUsageConst, const N: usize }
	{ T: [const] MemoryUsageConst, const N: usize }
	[T; N] |self| {
		// each element counts its own stack usage too
		// must use verbose syntax because traits are not const
		// (this matters with nightly feature enabled)
		let mut total = 0;
		let mut i = 0;
		let len = self.len();

		while i < len {
			total += self[i].memory_usage_const();
			i += 1;
		}

		total
	};
}
impl_memory_usage_only_static! {
	{ T: MemoryUsageStatic, const N: usize }
	{ T: [const] MemoryUsageStatic, const N: usize }
	[T; N] => {
		// each element counts its own stack usage too
		T::MEMORY_USAGE * N
	};
}
