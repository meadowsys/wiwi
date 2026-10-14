#[cfg(feature = "std")]
use crate::macros::impl_memory_usage;
use crate::macros::impl_memory_usage_const;

impl_memory_usage_const! {
	str |self| self.len();
}

#[cfg(feature = "std")]
impl_memory_usage! {
	String |self| size_of::<String>() + self.capacity();
	compacted |self| size_of::<String>() + self.len();
	compact |self| {
		let old_cap = self.capacity();
		self.shrink_to_fit();
		old_cap - self.len()
	};
}
