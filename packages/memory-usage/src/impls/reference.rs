use crate::{ MemoryUsage, MemoryUsageStatic };
use crate::macros::{ impl_memory_usage, impl_memory_usage_only_static };

impl_memory_usage! {
	{ 'h, T: MemoryUsage + ?Sized }
	{ 'h, T: [const] MemoryUsage + ?Sized }
	&'h T
	|self| size_of::<&'h T>() + (*self).memory_usage();
	ref compacted |self| (**self).memory_usage_compacted_ref();
	ref compact |self| (**self).compact_memory_ref();
}

impl_memory_usage_only_static! {
	{ 'h, T: MemoryUsageStatic + ?Sized }
	{ 'h, T: [const] MemoryUsageStatic + ?Sized }
	&'h T
	=> size_of::<&'h T>() + T::MEMORY_USAGE;
}

impl_memory_usage! {
	{ 'h, T: MemoryUsage + ?Sized }
	{ 'h, T: [const] MemoryUsage + ?Sized }
	&'h mut T
	|self| size_of::<&'h mut T>() + (**self).memory_usage();
	compacted |self| (**self).memory_usage_compacted();
	compact |self| (**self).compact_memory();
	ref compacted |self| (**self).memory_usage_compacted_ref();
	ref compact |self| (**self).compact_memory_ref();
}

impl_memory_usage_only_static! {
	{ 'h, T: MemoryUsageStatic + ?Sized }
	{ 'h, T: [const] MemoryUsageStatic + ?Sized }
	&'h mut T
	=> size_of::<&'h mut T>() + T::MEMORY_USAGE;
}

#[cfg(test)]
mod tests {
	use super::*;

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
	}

	#[test]
	fn reference_mut_usage() {
		let ref_size = cfg_select! {
			target_pointer_width = "16" => { 2 }
			target_pointer_width = "32" => { 4 }
			target_pointer_width = "64" => { 8 }
		};

		let reference = &mut 0u8;
		let expected = ref_size + u8::MEMORY_USAGE;

		assert_eq!((&reference).memory_usage(), expected);
		assert_eq!(<&mut u8>::MEMORY_USAGE, expected);
	}

	#[test]
	fn reference_mut_dst_usage() {
		let ref_size = cfg_select! {
			target_pointer_width = "16" => { 4 }
			target_pointer_width = "32" => { 8 }
			target_pointer_width = "64" => { 16 }
		};

		let reference = &mut [0u8; 5] as &mut [_];
		let expected = ref_size + (5 * u8::MEMORY_USAGE);

		assert_eq!((&reference).memory_usage(), expected);
	}
}
