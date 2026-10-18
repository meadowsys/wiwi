use crate::MemoryUsage;
use crate::macros::impl_memory_usage;

impl_memory_usage! {
	{ T: MemoryUsage }
	{ T: [const] MemoryUsage }
	Option<T>
	|self| match self {
		Some(val) => {
			let remaining = size_of::<Option<T>>() - size_of::<T>();
			val.memory_usage() + remaining
		}
		None => { size_of::<Option<T>>() }
	};
	compacted |self| match self {
		Some(val) => { val.memory_usage_compacted() }
		None => { 0 }
	};
	compact |self| match self {
		Some(val) => { val.compact_memory() }
		None => { 0 }
	};
	ref compacted |self| match self {
		Some(val) => { val.memory_usage_compacted_ref() }
		None => { 0 }
	};
	ref compact |self| match self {
		Some(val) => { val.compact_memory_ref() }
		None => { 0 }
	};
}

// Option can't be MemoryUsageStatic because it might or might not have a value
// with indirect memory (ex. Option<&usize>, &usize is MemoryUsageStatic, but
// Option could be either Some (2 * size_of::<usize>()) or None (size_of::<usize>()))
