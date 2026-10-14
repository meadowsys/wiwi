use crate::{ MemoryUsage, MemoryUsageConst, MemoryUsageStatic };
use crate::macros::{
	impl_memory_usage,
	impl_memory_usage_only_const,
	impl_memory_usage_only_static
};

impl_memory_usage! {
	{ 'h, T: MemoryUsage + ?Sized } &'h T
	|self| size_of::<&'h T>() + T::memory_usage(*self);
}

impl_memory_usage_only_const! {
	{ 'h, T: MemoryUsageConst + ?Sized }
	{ 'h, T: [const] MemoryUsageConst + ?Sized }
	&'h T
	|self| size_of::<&'h T>() + T::memory_usage_const(*self);
}

impl_memory_usage_only_static! {
	{ 'h, T: MemoryUsageStatic + ?Sized }
	{ 'h, T: [const] MemoryUsageStatic + ?Sized }
	&'h T
	=> size_of::<&'h T>() + T::MEMORY_USAGE;
}

impl_memory_usage! {
	{ 'h, T: MemoryUsage + ?Sized } &'h mut T
	|self| size_of::<&'h mut T>() + T::memory_usage(*self);
	compacted |self| size_of::<&'h mut T>() + T::memory_usage_compacted(*self);
	compact |self| T::compact_memory(*self);
}

impl_memory_usage_only_const! {
	{ 'h, T: MemoryUsageConst + ?Sized }
	{ 'h, T: [const] MemoryUsageConst + ?Sized }
	&'h mut T
	|self| size_of::<&'h mut T>() + T::memory_usage_const(*self);
}

impl_memory_usage_only_static! {
	{ 'h, T: MemoryUsageStatic + ?Sized }
	{ 'h, T: [const] MemoryUsageStatic + ?Sized }
	&'h mut T
	=> size_of::<&'h mut T>() + T::MEMORY_USAGE;
}
