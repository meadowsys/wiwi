use crate::{ MemoryUsage, MemoryUsageConst, MemoryUsageStatic };
use crate::macros::{
	impl_memory_usage,
	impl_memory_usage_only_const,
	impl_memory_usage_only_static
};

impl_memory_usage! {
	{ 'h, T: MemoryUsage + ?Sized }
	&'h T
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
	{ 'h, T: MemoryUsage + ?Sized }
	&'h mut T
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

#[cfg(test)]
mod tests {
	use crate::{
		MemoryUsage as _,
		MemoryUsageConst as _,
		MemoryUsageStatic as _
	};

	#[test]
	fn reference_usage() {
		let ref_size = cfg_select! {
			target_pointer_width = "16" => { 2 }
			target_pointer_width = "32" => { 4 }
			target_pointer_width = "64" => { 8 }
		};

		let reference = &0u8;
		let expected = ref_size + u8::MEMORY_USAGE;

		assert_eq!((&reference).memory_usage(), expected);
		assert_eq!((&reference).memory_usage_const(), expected);
		assert_eq!(<&u8>::MEMORY_USAGE, expected);
	}

	#[test]
	fn reference_dst_usage() {
		let ref_size = cfg_select! {
			target_pointer_width = "16" => { 4 }
			target_pointer_width = "32" => { 8 }
			target_pointer_width = "64" => { 16 }
		};

		let reference = &[0u8; 5] as &[_];
		let expected = ref_size + (5 * u8::MEMORY_USAGE);

		assert_eq!((&reference).memory_usage(), expected);
		assert_eq!((&reference).memory_usage_const(), expected);
	}
}
