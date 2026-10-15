#[cfg(feature = "std")]
use crate::{ MemoryUsage, MemoryUsageConst };
#[cfg(feature = "std")]
use crate::macros::{ impl_memory_usage, impl_memory_usage_only_const };

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
		let elements = self.as_slice().memory_usage_compacted();
		vec + elements
	};
	compact |self| {
		let old_cap = self.capacity();
		self.shrink_to_fit();
		let vec = (old_cap - self.len()) * size_of::<T>();

		let elements = self.iter_mut().map(T::compact_memory).sum::<usize>();

		vec + elements
	};
	// can compact elements by ref, but cannot shrink vec
	ref compacted |self| {
		let vec = size_of::<Vec<T>>();
		let elements = self.as_slice().memory_usage_compacted_ref();
		vec + elements
	};
	ref compact |self| self.iter().map(T::compact_memory_ref).sum();
}

#[cfg(feature = "std")]
impl_memory_usage_only_const! {
	{ T: MemoryUsageConst }
	{ T: [const] MemoryUsageConst }
	Vec<T>
	|self| {
		let vec = size_of::<Vec<T>>();
		let elements = self.as_slice().memory_usage_const();
		let remaining_capacity = (self.capacity() - self.len()) * size_of::<T>();
		vec + elements + remaining_capacity
	};
}

#[cfg(test)]
mod tests {
	#[cfg(feature = "std")]
	use super::*;
	#[cfg(feature = "std")]
	use crate::MemoryUsageStatic as _;

	#[cfg(feature = "std")]
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

	#[cfg(feature = "std")]
	#[test]
	fn vec_string_recursive_usage() {
		let vec_size = <*const u8>::MEMORY_USAGE + usize::MEMORY_USAGE + usize::MEMORY_USAGE;
		let string_size = vec_size;

		let mut s1 = String::with_capacity(247);
		let s1_capacity = s1.capacity();
		s1.push_str("awawawa");

		let mut s2 = String::with_capacity(247);
		let s2_capacity = s2.capacity();
		s2.push_str("12345678901234567890");

		let mut s3 = String::with_capacity(247);
		let s3_capacity = s3.capacity();
		s3.push_str("this is a 34 character string lmao");

		let mut vec = Vec::with_capacity(247);
		let vec_capacity = vec.capacity();
		vec.push(s1);
		vec.push(s2);
		vec.push(s3);

		let memory_usage =
			// vec itself
			vec_size
			// vec capacity
			+ (vec_capacity * string_size)
			// each of the 3 strings' capacities
			+ s1_capacity
			+ s2_capacity
			+ s3_capacity;
		assert_eq!(vec.memory_usage(), memory_usage);

		let memory_usage_compacted =
			// vec itself
			vec_size
			// vec length
			+ (3 * string_size)
			// each of the 3 strings' lengths
			+ 7
			+ 20
			+ 34;
		assert_eq!(vec.memory_usage_compacted(), memory_usage_compacted);

		// expect this many bytes to be saved running compact_memory()
		let compact_memory = memory_usage - memory_usage_compacted;
		assert_eq!(vec.compact_memory(), compact_memory);
	}
}
