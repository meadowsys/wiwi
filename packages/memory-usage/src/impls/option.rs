use crate::{ MemoryUsage, MemoryUsageConst };
use crate::macros::{ impl_memory_usage, impl_memory_usage_only_const };

impl_memory_usage! {
	{ T: MemoryUsage }
	Option<T>
	|self| match self {
		Some(val) => {
			let remaining = size_of::<Option<T>>() - size_of::<T>();
			val.memory_usage() + remaining
		}
		None => { size_of::<Option<T>>() }
	};
	compacted |self| match self {
		Some(val) => {
			let remaining = size_of::<Option<T>>() - size_of::<T>();
			val.memory_usage_compacted() + remaining
		}
		None => { size_of::<Option<T>>() }
	};
	compact |self| match self {
		Some(val) => { val.compact_memory() }
		None => { 0 }
	};
	ref compacted |self| match self {
		Some(val) => {
			let remaining = size_of::<Option<T>>() - size_of::<T>();
			val.memory_usage_compacted_ref() + remaining
		}
		None => { size_of::<Option<T>>() }
	};
	ref compact |self| match self {
		Some(val) => { val.compact_memory_ref() }
		None => { 0 }
	};
}

impl_memory_usage_only_const! {
	{ T: MemoryUsageConst }
	{ T: [const] MemoryUsageConst }
	Option<T>
	|self| match self {
		Some(val) => {
			let remaining = size_of::<Option<T>>() - size_of::<T>();
			val.memory_usage_const() + remaining
		}
		None => { size_of::<Option<T>>() }
	};

}

// Option can't be MemoryUsageStatic because it might or might not have a value
// with indirect memory (ex. Option<&usize>, &usize is MemoryUsageStatic, but
// Option could be either Some (2 * size_of::<usize>()) or None (size_of::<usize>()))
