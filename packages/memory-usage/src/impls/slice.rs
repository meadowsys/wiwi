use crate::MemoryUsage;
use crate::macros::impl_memory_usage;

impl_memory_usage! {
	{ T: MemoryUsage }
	{ T: [const] MemoryUsage }
	// todo this is only dyn because mapping methods aren't const, can be rewritten
	dyn [T]
	|self| {
		// each element counts its own stack usage too
		self.iter().map(T::memory_usage).sum()
	};
	compacted |self| self.iter().map(T::memory_usage_compacted).sum();
	compact |self| self.iter_mut().map(T::compact_memory).sum();
	ref compacted |self| self.iter().map(T::memory_usage_compacted_ref).sum();
	ref compact |self| self.iter().map(T::compact_memory_ref).sum();
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::MemoryUsageStatic as _;

	#[test]
	fn slice_usage() {
		let str_size = usize::MEMORY_USAGE + <*const ()>::MEMORY_USAGE;

		let slice = &[1u8, 2, 3, 4, 5] as &[_];

		assert_eq!((*slice).memory_usage(), 5);

		let slice = &["1", "12", "123"] as &[_];
		let expected = (3 * str_size) + 1 + 2 + 3;

		assert_eq!((*slice).memory_usage(), expected);
	}
}
