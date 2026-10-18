use crate::{ MemoryUsage, MemoryUsageStatic };
use crate::macros::{ impl_memory_usage, impl_memory_usage_only_static };

impl_memory_usage! {
	{ T: MemoryUsage, E: MemoryUsage }
	{ T: [const] MemoryUsage, E: [const] MemoryUsage }
	Result<T, E>
	|self| match self {
		Ok(val) => {
			let remaining = size_of::<Result<T, E>>() - size_of::<T>();
			val.memory_usage() + remaining
		}
		Err(err) => {
			let remaining = size_of::<Result<T, E>>() - size_of::<E>();
			err.memory_usage() + remaining
		}
	};
	compacted |self| match self {
		Ok(val) => {
			let remaining = size_of::<Result<T, E>>() - size_of::<T>();
			val.memory_usage_compacted() + remaining
		}
		Err(err) => {
			let remaining = size_of::<Result<T, E>>() - size_of::<E>();
			err.memory_usage_compacted() + remaining
		}
	};
	compact |self| match self {
		Ok(val) => { val.compact_memory() }
		Err(err) => { err.compact_memory() }
	};
	ref compacted |self| match self {
		Ok(val) => {
			let remaining = size_of::<Result<T, E>>() - size_of::<T>();
			val.memory_usage_compacted_ref() + remaining
		}
		Err(err) => {
			let remaining = size_of::<Result<T, E>>() - size_of::<E>();
			err.memory_usage_compacted_ref() + remaining
		}
	};
	ref compact |self| match self {
		Ok(val) => { val.compact_memory_ref() }
		Err(err) => { err.compact_memory_ref() }
	};
}

// See Option for reasoning why we can't implement MemoryUsageStatic for every
// result type. However! if T and E are the same type, Result will always have
// one value of it, regardless of if it ends up being stored in the Ok or the
// Err variant. We can implement MemoryUsageStatic for this case.
impl_memory_usage_only_static! {
	{ TE: MemoryUsageStatic }
	{ TE: [const] MemoryUsageStatic }
	Result<TE, TE>
	=> {
		let remainder = size_of::<Result<TE, TE>>() - size_of::<TE>();
		TE::MEMORY_USAGE + remainder
	};
}
