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
		let capacity = self.capacity() - self.len();
		vec + elements + capacity
	};
}
