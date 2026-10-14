use crate::macros::impl_memory_usage_size_of;

impl_memory_usage_size_of! {
	char
}

#[cfg(test)]
mod tests {
	use crate::{
		MemoryUsage as _,
		MemoryUsageConst as _,
		MemoryUsageStatic as _
	};

	#[test]
	fn char_usage() {
		let char = 'A';

		assert_eq!(char.memory_usage(), 4);
		assert_eq!(char.memory_usage_const(), 4);
		assert_eq!(char::MEMORY_USAGE, 4);
	}
}
