use crate::{ MemoryUsage, MemoryUsageConst };
use crate::macros::{ impl_memory_usage, impl_memory_usage_only_const };

impl_memory_usage! {
	{ T: MemoryUsage }
	[T]
	|self| {
		// each element counts its own stack usage too
		self.iter().map(T::memory_usage).sum()
	};
	compacted |self| self.iter().map(T::memory_usage_compacted).sum();
	compact |self| self.iter_mut().map(T::compact_memory).sum();
	ref compacted |self| self.iter().map(T::memory_usage_compacted_ref).sum();
	ref compact |self| self.iter().map(T::compact_memory_ref).sum();
}

impl_memory_usage_only_const! {
	{ T: MemoryUsageConst }
	{ T: [const] MemoryUsageConst }
	[T]
	|self| {
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::MemoryUsageStatic as _;

	#[test]
	fn slice_usage() {
		let str_size = usize::MEMORY_USAGE + <*const ()>::MEMORY_USAGE;

		let slice = &[1u8, 2, 3, 4, 5] as &[_];

		assert_eq!((*slice).memory_usage(), 5);
		assert_eq!((*slice).memory_usage_const(), 5);

		let slice = &["1", "12", "123"] as &[_];
		let expected = (3 * str_size) + 1 + 2 + 3;

		assert_eq!((*slice).memory_usage(), expected);
		assert_eq!((*slice).memory_usage_const(), expected);
	}
}
