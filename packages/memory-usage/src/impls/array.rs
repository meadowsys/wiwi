use crate::{ MemoryUsage, MemoryUsageConst, MemoryUsageStatic };
use crate::macros::{
	impl_memory_usage,
	impl_memory_usage_only_const,
	impl_memory_usage_only_static
};

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
	[T; N]
	// each element counts its own stack usage too
	=> T::MEMORY_USAGE * N;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn array_usage() {
		let str_size = usize::MEMORY_USAGE + <*const ()>::MEMORY_USAGE;

		let array = [1u8, 2, 3, 4, 5];

		assert_eq!(array.memory_usage(), 5);
		assert_eq!(array.memory_usage_const(), 5);
		assert_eq!(<[u8; 5]>::MEMORY_USAGE, 5);

		let array = ["1", "12", "123"];
		let expected = (3 * str_size) + 1 + 2 + 3;

		assert_eq!(array.memory_usage(), expected);
		assert_eq!(array.memory_usage_const(), expected);
	}
}
