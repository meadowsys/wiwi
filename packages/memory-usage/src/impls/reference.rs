use crate::{ MemoryUsage, MemoryUsageConst, MemoryUsageStatic };

impl<'h, T: MemoryUsage + ?Sized> MemoryUsage for &'h T {
	#[inline]
	fn memory_usage(&self) -> usize {
		size_of::<&'h T>() + T::memory_usage(*self)
	}
}

impl<'h, T: MemoryUsageConst + ?Sized> MemoryUsageConst for &'h T {
	#[inline]
	fn memory_usage_const(&self) -> usize {
		size_of::<&'h T>() + T::memory_usage_const(*self)
	}
}

impl<'h, T: MemoryUsageStatic + ?Sized> MemoryUsageStatic for &'h T {
	const MEMORY_USAGE: usize = size_of::<&'h T>() + T::MEMORY_USAGE;
}

impl<'h, T: MemoryUsage + ?Sized> MemoryUsage for &'h mut T {
	#[inline]
	fn memory_usage(&self) -> usize {
		size_of::<&'h mut T>() + T::memory_usage(*self)
	}

	#[inline]
	fn memory_usage_compacted(&self) -> usize {
		size_of::<&'h mut T>() + T::memory_usage_compacted(*self)
	}

	#[inline]
	fn compact_memory(&mut self) -> usize {
		// reference isn't saving memory but the value behind it could
		T::compact_memory(*self)
	}
}

impl<'h, T: MemoryUsageConst + ?Sized> MemoryUsageConst for &'h mut T {
	#[inline]
	fn memory_usage_const(&self) -> usize {
		size_of::<&'h mut T>() + T::memory_usage_const(*self)
	}
}

impl<'h, T: MemoryUsageStatic + ?Sized> MemoryUsageStatic for &'h mut T {
	const MEMORY_USAGE: usize = size_of::<&'h mut T>() + T::MEMORY_USAGE;
}
