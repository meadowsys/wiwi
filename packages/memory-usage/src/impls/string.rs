#[cfg(feature = "std")]
use crate::macros::impl_memory_usage;
use crate::macros::impl_memory_usage_const;

impl_memory_usage_const! {
	str |self| self.len();
}

#[cfg(feature = "std")]
impl_memory_usage! {
	String
	|self| size_of::<String>() + self.capacity();
	compacted |self| size_of::<String>() + self.len();
	compact |self| {
		let old_cap = self.capacity();
		self.shrink_to_fit();
		old_cap - self.len()
	};
}

#[cfg(test)]
mod tests {
	#[cfg(feature = "std")]
	use crate::MemoryUsageStatic as _;
	use crate::{ MemoryUsage as _, MemoryUsageConst as _ };

	#[test]
	fn str_usage() {
		let str = "awawawa";

		assert_eq!(str.memory_usage(), 7);
		assert_eq!(str.memory_usage_const(), 7);
	}

	#[cfg(feature = "std")]
	#[test]
	fn string_usage() {
		let string_size = <*const u8>::MEMORY_USAGE + usize::MEMORY_USAGE + usize::MEMORY_USAGE;

		let mut string = String::with_capacity(247);
		let string_capacity = string.capacity();
		string.push_str("awawawa");
		let expected = string_size + string_capacity;
		let expected_compacted = string_size + 7;
		let expected_compact = expected - expected_compacted;

		assert_eq!(string.memory_usage(), expected);
		assert_eq!(string.memory_usage_compacted(), expected_compacted);
		assert_eq!(string.compact_memory(), expected_compact);
	}
}
