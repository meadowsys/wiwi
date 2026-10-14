use crate::macros::impl_memory_usage_size_of;

impl_memory_usage_size_of! {
	()
}

#[cfg(test)]
mod tests {
	use crate::{
		MemoryUsage as _,
		MemoryUsageConst as _,
		MemoryUsageStatic as _
	};

	#[test]
	fn unit_usage() {
		#[expect(clippy::let_unit_value, reason = "intentional")]
		let unit = ();

		assert_eq!(unit.memory_usage(), 0);
		assert_eq!(unit.memory_usage_const(), 0);
		assert_eq!(<()>::MEMORY_USAGE, 0);
	}
}
