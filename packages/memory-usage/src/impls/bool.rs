use crate::macros::impl_memory_usage_size_of;

impl_memory_usage_size_of! {
	bool
}

#[cfg(test)]
mod tests {
	use crate::{
		MemoryUsage as _,
		MemoryUsageConst as _,
		MemoryUsageStatic as _
	};

	#[test]
	fn bool_usage() {
		let bool = true;

		assert_eq!(bool.memory_usage(), 1);
		assert_eq!(bool.memory_usage_const(), 1);
		assert_eq!(bool::MEMORY_USAGE, 1);
	}
}
