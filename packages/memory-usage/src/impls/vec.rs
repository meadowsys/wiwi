#[cfg(feature = "std")]
use crate::MemoryUsage;
#[cfg(feature = "std")]
use crate::macros::impl_memory_usage;

#[cfg(feature = "std")]
impl_memory_usage! {
	{ T: MemoryUsage }
	Vec<T>
	|self| {
		let vec = size_of::<Vec<T>>();
		let elements = self.as_slice().memory_usage();
		let remaining_capacity = (self.capacity() - self.len()) * size_of::<T>();
		vec + elements + remaining_capacity
	};
	compacted |self| {
		let vec = size_of::<Vec<T>>();
		let elements = self.as_slice().memory_usage();
		vec + elements
	};
	compact |self| {
		let old_cap = self.capacity();
		self.shrink_to_fit();
		(old_cap - self.len()) * size_of::<T>()
	};
}

#[cfg(test)]
mod tests {
	use crate::{
		MemoryUsage as _,
		MemoryUsageStatic as _
	};

	#[test]
	fn vec_usage() {
		let vec_size = <*const u8>::MEMORY_USAGE + usize::MEMORY_USAGE + usize::MEMORY_USAGE;
		let str_size = usize::MEMORY_USAGE + <*const ()>::MEMORY_USAGE;

		let mut vec = Vec::with_capacity(247);
		let vec_capacity = vec.capacity();
		vec.extend_from_slice(&[0u8; 32]);
		let expected = vec_size + vec_capacity;
		let expected_compacted = vec_size + 32;
		let expected_compact = expected - expected_compacted;

		assert_eq!(vec.memory_usage(), expected);
		assert_eq!(vec.memory_usage_compacted(), expected_compacted);
		assert_eq!(vec.compact_memory(), expected_compact);

		let mut vec = Vec::with_capacity(247);
		let vec_capacity = vec.capacity();
		vec.extend_from_slice(&["1", "12", "123"]);
		let expected = vec_size + (vec_capacity * str_size) + 1 + 2 + 3;
		let expected_compacted = vec_size + (3 * str_size) + 1 + 2 + 3;
		let expected_compact = expected - expected_compacted;

		assert_eq!(vec.memory_usage(), expected);
		assert_eq!(vec.memory_usage_compacted(), expected_compacted);
		assert_eq!(vec.compact_memory(), expected_compact);
	}
}
